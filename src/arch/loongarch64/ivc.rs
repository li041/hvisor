// Copyright (c) 2025 Syswonder
// hvisor is licensed under Mulan PSL v2.
//
// LoongArch64 IVC（Inter-VM Communication）第一版：
// - 与 `arch/aarch64/ivc.rs` 在「配置、共享内存、控制表 MMIO 布局」上对齐，便于同一套 guest 协议。
// - 「通知对端 peer」在 AArch64 上通过 GIC `set_ispender` 实现；本分支在目标 peer 的 pCPU 上
//   先记录待注 guest 中断线，再 `send_event(..., IPI_EVENT_IVC)`，由 `check_events` →
//   `loongarch_ivc_on_ipi_event` → `inject_irq` 完成唤醒（>INT_IPI 的线号暂以 IPI 唤醒，EXTIOI 可后续接）。

use alloc::collections::btree_map::BTreeMap;
use alloc::vec::Vec;
use spin::Mutex;

use crate::{
    config::{HvIvcConfig, CONFIG_MAX_IVC_CONFIGS},
    consts::{IPI_EVENT_IVC, PAGE_SIZE},
    device::irqchip::ls7a2000::loongarch_ivc_set_pending_guest_irq_for_pcpu,
    error::HvResult,
    event::send_event,
    memory::{Frame, GuestPhysAddr, MMIOAccess, MemFlags, MemoryRegion},
    zone::{find_zone, this_zone_id, Zone},
};

// -------- 全局表：与 AArch64 相同的设计 --------

/// `ivc_id` -> 该 IVC 通道在 hypervisor 侧的全局记录（共享物理页、各 peer 信息）
static IVC_RECORDS: Mutex<BTreeMap<u32, IvcRecord>> = Mutex::new(BTreeMap::new());

/// `zone_id` -> 该 zone 从 hypercall 可见的 `IvcInfo` 快照（交给 guest 填自己的结构体用）
pub static IVC_INFOS: Mutex<BTreeMap<usize, IvcInfo>> = Mutex::new(BTreeMap::new());

/// 客户机通过 hypercall 取回、与 `HvIvcConfig` 对应的只读汇总（C 布局，与 AArch64 一致）
#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct IvcInfo {
    pub len: u64,
    ivc_ct_ipas: [u64; CONFIG_MAX_IVC_CONFIGS],
    ivc_shmem_ipas: [u64; CONFIG_MAX_IVC_CONFIGS],
    ivc_ids: [u32; CONFIG_MAX_IVC_CONFIGS],
    ivc_irqs: [u32; CONFIG_MAX_IVC_CONFIGS],
}

impl From<&[HvIvcConfig]> for IvcInfo {
    fn from(configs: &[HvIvcConfig]) -> Self {
        let mut ivc_ids = [0; CONFIG_MAX_IVC_CONFIGS];
        let mut ivc_ct_ipas = [0; CONFIG_MAX_IVC_CONFIGS];
        let mut ivc_shmem_ipas = [0; CONFIG_MAX_IVC_CONFIGS];
        let mut ivc_irqs = [0; CONFIG_MAX_IVC_CONFIGS];
        for i in 0..configs.len() {
            let c = &configs[i];
            ivc_ids[i] = c.ivc_id;
            ivc_ct_ipas[i] = c.control_table_ipa;
            ivc_shmem_ipas[i] = c.shared_mem_ipa;
            ivc_irqs[i] = c.interrupt_num;
        }
        Self {
            len: configs.len() as u64,
            ivc_ids,
            ivc_shmem_ipas,
            ivc_ct_ipas,
            ivc_irqs,
        }
    }
}

/// 把某个 `HvIvcConfig` 登记进全局 `IVC_RECORDS`；若同一 `ivc_id` 已有记录则合并 peer，并检查参数一致
fn insert_ivc_record(ivc_config: &HvIvcConfig, zone_id: u32) -> Result<(bool, usize), ()> {
    let mut recs = IVC_RECORDS.lock();
    let ivc_id = ivc_config.ivc_id;
    if let Some(rec) = recs.get_mut(&ivc_id) {
        if rec.max_peers != ivc_config.max_peers
            || rec.rw_sec_size != ivc_config.rw_sec_size
            || rec.out_sec_size != ivc_config.out_sec_size
        {
            error!("ivc config conflicts!!!");
            return Err(());
        }
        if rec.peer_infos.len() == rec.max_peers as _ {
            error!("can't add more peers to ivc_id {}", ivc_id);
            return Err(());
        }
        rec.peer_infos.insert(
            ivc_config.peer_id,
            PeerInfo {
                zone_id,
                irq_num: ivc_config.interrupt_num,
                shared_mem_ipa: ivc_config.shared_mem_ipa,
            },
        );
        Ok((false, rec.shared_mem.start_paddr()))
    } else {
        if ivc_config.rw_sec_size as usize % PAGE_SIZE != 0
            || ivc_config.out_sec_size as usize % PAGE_SIZE != 0
        {
            error!("section size must be page aligned!!!");
            return Err(());
        }
        let mut rec = IvcRecord::from(ivc_config);
        let start_paddr = rec.shared_mem.start_paddr();
        rec.peer_infos.insert(
            ivc_config.peer_id,
            PeerInfo {
                zone_id,
                irq_num: ivc_config.interrupt_num,
                shared_mem_ipa: ivc_config.shared_mem_ipa,
            },
        );
        recs.insert(ivc_id, rec);
        Ok((true, start_paddr))
    }
}

struct IvcRecord {
    max_peers: u32,
    rw_sec_size: u32,
    out_sec_size: u32,
    peer_infos: BTreeMap<u32, PeerInfo>,
    shared_mem: Frame,
}

#[allow(unused)]
struct PeerInfo {
    zone_id: u32,
    irq_num: u32,
    shared_mem_ipa: u64,
}

/// Zone 退出时从全局表里摘掉该 zone 相关条目（与 AArch64 相同）
pub fn cleanup_zone_ivc(zone_id: usize) {
    info!("cleanup IVC for zone {}", zone_id);

    IVC_INFOS.lock().remove(&zone_id);

    let mut recs = IVC_RECORDS.lock();
    let zone_id_u32 = zone_id as u32;
    let mut to_remove: Vec<(u32, u32)> = Vec::new();
    for (ivc_id, rec) in recs.iter() {
        for (peer_id, peer_info) in rec.peer_infos.iter() {
            if peer_info.zone_id == zone_id_u32 {
                to_remove.push((*ivc_id, *peer_id));
            }
        }
    }
    for (ivc_id, peer_id) in to_remove {
        if let Some(rec) = recs.get_mut(&ivc_id) {
            rec.peer_infos.remove(&peer_id);
            info!(
                "removed peer_id {} (zone_id {}) from ivc_id {}",
                peer_id, zone_id, ivc_id
            );
        }
    }
    info!("IVC cleanup completed for zone {}", zone_id);
}

impl From<&HvIvcConfig> for IvcRecord {
    fn from(config: &HvIvcConfig) -> Self {
        let n_pages =
            (config.rw_sec_size + config.out_sec_size * config.max_peers) / (PAGE_SIZE as u32);
        let frames = Frame::new_contiguous(n_pages as usize, 0).unwrap();
        Self {
            max_peers: config.max_peers,
            rw_sec_size: config.rw_sec_size,
            out_sec_size: config.out_sec_size,
            peer_infos: BTreeMap::new(),
            shared_mem: frames,
        }
    }
}

impl Zone {
    /// 根据配置分配共享 RAM、建立 GPA→HPA 映射，并把控制表一页注册为 MMIO（由 `mmio_ivc_handler` 处理）
    pub fn ivc_init(&mut self, ivc_configs: &[HvIvcConfig]) -> HvResult {
        let mut inner = self.write();
        for ivc_config in ivc_configs {
            if let Ok((_, start_paddr)) = insert_ivc_record(ivc_config, self.id() as _) {
                info!(
                    "ivc init: zone {} shared mem hpa=0x{:x}, gpa=0x{:x}",
                    self.id(),
                    start_paddr,
                    ivc_config.shared_mem_ipa
                );
                let rw_sec_size = ivc_config.rw_sec_size as usize;
                let out_sec_size = ivc_config.out_sec_size as usize;
                inner
                    .gpm_mut()
                    .insert(MemoryRegion::new_with_offset_mapper(
                        ivc_config.shared_mem_ipa as _,
                        start_paddr,
                        rw_sec_size,
                        MemFlags::READ | MemFlags::WRITE,
                    ))?;
                for i in 0..ivc_config.max_peers as usize {
                    let flags = if i == ivc_config.peer_id as _ {
                        MemFlags::READ | MemFlags::WRITE
                    } else {
                        MemFlags::READ
                    };
                    inner
                        .gpm_mut()
                        .insert(MemoryRegion::new_with_offset_mapper(
                            ivc_config.shared_mem_ipa as usize + rw_sec_size + i * out_sec_size,
                            start_paddr + rw_sec_size + i * out_sec_size,
                            out_sec_size,
                            flags,
                        ))?;
                }
                inner.mmio_region_register(
                    ivc_config.control_table_ipa as _,
                    PAGE_SIZE,
                    mmio_ivc_handler,
                    ivc_config.control_table_ipa as _,
                );
            } else {
                return hv_result_err!(EINVAL);
            }
        }
        IVC_INFOS
            .lock()
            .insert(self.id(), IvcInfo::from(ivc_configs));
        Ok(())
    }
}

fn loongarch_ivc_deliver_to_peer(target_zone_id: u32, guest_irq: u32) -> HvResult {
    let Some(z) = find_zone(target_zone_id as usize) else {
        error!("ivc: target zone {} not found for IPI_INVOKE", target_zone_id);
        return hv_result_err!(EINVAL);
    };
    let Some(pcpu) = z.cpu_set().first_cpu() else {
        error!("ivc: zone {} has no pcpu in cpu_set", target_zone_id);
        return hv_result_err!(EINVAL);
    };
    loongarch_ivc_set_pending_guest_irq_for_pcpu(pcpu, guest_irq);
    trace!(
        "ivc: IPI_INVOKE -> zone {} pcpu {} guest_irq {}",
        target_zone_id,
        pcpu,
        guest_irq
    );
    send_event(pcpu, 0, IPI_EVENT_IVC);
    Ok(())
}

// -------- 控制表布局（与 AArch64 相同，偏移相对 control table 页内） --------

const CT_IVC_ID: GuestPhysAddr = 0x00;
const CT_MAX_PEERS: GuestPhysAddr = 0x04;
const CT_RW_SEC_SIZE: GuestPhysAddr = 0x08;
const CT_OUT_SEC_SIZE: GuestPhysAddr = 0x0C;
const CT_PEER_ID: GuestPhysAddr = 0x10;
const CT_IPI_INVOKE: GuestPhysAddr = 0x14;

/// 客户机访问 IVC 控制表 MMIO 时由 hypervisor 模拟读写
pub fn mmio_ivc_handler(mmio: &mut MMIOAccess, base: usize) -> HvResult {
    let zone_id = this_zone_id();
    let is_write = mmio.is_write;
    let ivc_infos = IVC_INFOS.lock();
    let ivc_info = ivc_infos.get(&zone_id).unwrap();
    let ivc_id = (0..ivc_info.len as usize)
        .find(|&i| ivc_info.ivc_ct_ipas[i] == base as u64)
        .map(|i| ivc_info.ivc_ids[i])
        .unwrap();
    drop(ivc_infos);
    if mmio.address == CT_IPI_INVOKE && is_write {
        let peer_id = mmio.value as u32;
        let recs = IVC_RECORDS.lock();
        let Some(rec) = recs.get(&ivc_id) else {
            drop(recs);
            return hv_result_err!(EINVAL);
        };
        let out = rec.peer_infos.get(&peer_id).map(|i| (i.zone_id, i.irq_num));
        drop(recs);
        let Some((target_zone, guest_irq)) = out else {
            error!("zone {} has no peer {}", zone_id, peer_id);
            return hv_result_err!(EINVAL);
        };
        return loongarch_ivc_deliver_to_peer(target_zone, guest_irq);
    }

    let recs = IVC_RECORDS.lock();
    let rec = recs.get(&ivc_id).unwrap();
    mmio.value = match mmio.address {
        CT_IVC_ID => ivc_id as usize,
        CT_MAX_PEERS => rec.max_peers as usize,
        CT_RW_SEC_SIZE => rec.rw_sec_size as usize,
        CT_OUT_SEC_SIZE => rec.out_sec_size as usize,
        CT_PEER_ID => {
            let peer_id = rec
                .peer_infos
                .iter()
                .find(|&(_, info)| info.zone_id == zone_id as u32)
                .map(|(pid, _)| *pid)
                .unwrap();
            peer_id as usize
        }
        _ => return hv_result_err!(EFAULT),
    };
    Ok(())
}
