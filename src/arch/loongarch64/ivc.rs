// Copyright (c) 2025 Syswonder
// hvisor is licensed under Mulan PSL v2.

use alloc::{collections::btree_map::BTreeMap, vec::Vec};
use spin::Mutex;

use crate::{
    config::{HvIvcConfig, CONFIG_MAX_IVC_CONFIGS},
    consts::PAGE_SIZE,
    device::irqchip::ls7a2000::set_guest_irq_line,
    error::HvResult,
    memory::{Frame, GuestPhysAddr, MMIOAccess, MemFlags, MemoryRegion},
    zone::{find_zone, this_zone_id, Zone},
};

static IVC_RECORDS: Mutex<BTreeMap<u32, IvcRecord>> = Mutex::new(BTreeMap::new());
pub static IVC_INFOS: Mutex<BTreeMap<usize, IvcInfo>> = Mutex::new(BTreeMap::new());

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
        let mut info = Self {
            len: configs.len() as u64,
            ivc_ct_ipas: [0; CONFIG_MAX_IVC_CONFIGS],
            ivc_shmem_ipas: [0; CONFIG_MAX_IVC_CONFIGS],
            ivc_ids: [0; CONFIG_MAX_IVC_CONFIGS],
            ivc_irqs: [0; CONFIG_MAX_IVC_CONFIGS],
        };

        for (index, config) in configs.iter().enumerate() {
            info.ivc_ct_ipas[index] = config.control_table_ipa;
            info.ivc_shmem_ipas[index] = config.shared_mem_ipa;
            info.ivc_ids[index] = config.ivc_id;
            info.ivc_irqs[index] = config.interrupt_num;
        }
        info
    }
}

struct PeerInfo {
    zone_id: u32,
    irq_num: u32,
}

struct IvcRecord {
    max_peers: u32,
    rw_sec_size: u32,
    out_sec_size: u32,
    peer_infos: BTreeMap<u32, PeerInfo>,
    shared_mem: Frame,
}

impl IvcRecord {
    fn new(config: &HvIvcConfig) -> HvResult<Self> {
        let size = config
            .out_sec_size
            .checked_mul(config.max_peers)
            .and_then(|size| size.checked_add(config.rw_sec_size))
            .ok_or(hv_err!(EINVAL))?;
        if config.out_sec_size == 0 || size as usize % PAGE_SIZE != 0 {
            return hv_result_err!(EINVAL, "IVC shared memory size must be page aligned");
        }

        let mut shared_mem = Frame::new_contiguous(size as usize / PAGE_SIZE, 0)?;
        shared_mem.clear();
        Ok(Self {
            max_peers: config.max_peers,
            rw_sec_size: config.rw_sec_size,
            out_sec_size: config.out_sec_size,
            peer_infos: BTreeMap::new(),
            shared_mem,
        })
    }

    fn config_matches(&self, config: &HvIvcConfig) -> bool {
        self.max_peers == config.max_peers
            && self.rw_sec_size == config.rw_sec_size
            && self.out_sec_size == config.out_sec_size
    }
}

fn insert_ivc_record(config: &HvIvcConfig, zone_id: u32) -> HvResult<usize> {
    if config.max_peers == 0 || config.peer_id >= config.max_peers {
        return hv_result_err!(EINVAL, "invalid IVC peer configuration");
    }
    if config.control_table_ipa as usize % PAGE_SIZE != 0
        || config.shared_mem_ipa as usize % PAGE_SIZE != 0
        || config.rw_sec_size as usize % PAGE_SIZE != 0
        || config.out_sec_size as usize % PAGE_SIZE != 0
    {
        return hv_result_err!(EINVAL, "IVC addresses and sections must be page aligned");
    }

    let mut records = IVC_RECORDS.lock();
    if !records.contains_key(&config.ivc_id) {
        records.insert(config.ivc_id, IvcRecord::new(config)?);
    }

    let record = records.get_mut(&config.ivc_id).unwrap();
    if !record.config_matches(config)
        || record.peer_infos.len() >= record.max_peers as usize
        || record.peer_infos.contains_key(&config.peer_id)
    {
        return hv_result_err!(EINVAL, "conflicting IVC channel configuration");
    }
    record.peer_infos.insert(
        config.peer_id,
        PeerInfo {
            zone_id,
            irq_num: config.interrupt_num,
        },
    );
    Ok(record.shared_mem.start_paddr())
}

pub fn cleanup_zone_ivc(zone_id: usize) {
    IVC_INFOS.lock().remove(&zone_id);

    let mut records = IVC_RECORDS.lock();
    let mut empty_channels = Vec::new();
    for (ivc_id, record) in records.iter_mut() {
        record
            .peer_infos
            .retain(|_, peer| peer.zone_id != zone_id as u32);
        if record.peer_infos.is_empty() {
            empty_channels.push(*ivc_id);
        }
    }
    for ivc_id in empty_channels {
        records.remove(&ivc_id);
    }
}

impl Zone {
    pub fn ivc_init(&mut self, configs: &[HvIvcConfig]) -> HvResult {
        if configs.is_empty() {
            return Ok(());
        }

        for config in configs {
            let start_paddr = insert_ivc_record(config, self.id() as u32)?;
            let rw_size = config.rw_sec_size as usize;
            let out_size = config.out_sec_size as usize;
            let mut inner = self.write();

            if rw_size != 0 {
                inner
                    .gpm_mut()
                    .insert(MemoryRegion::new_with_offset_mapper(
                        config.shared_mem_ipa as usize,
                        start_paddr,
                        rw_size,
                        MemFlags::READ | MemFlags::WRITE,
                    ))?;
            }
            for peer_id in 0..config.max_peers as usize {
                let flags = if peer_id == config.peer_id as usize {
                    MemFlags::READ | MemFlags::WRITE
                } else {
                    MemFlags::READ
                };
                inner
                    .gpm_mut()
                    .insert(MemoryRegion::new_with_offset_mapper(
                        config.shared_mem_ipa as usize + rw_size + peer_id * out_size,
                        start_paddr + rw_size + peer_id * out_size,
                        out_size,
                        flags,
                    ))?;
            }
            inner.mmio_region_register(
                config.control_table_ipa as usize,
                PAGE_SIZE,
                mmio_ivc_handler,
                config.control_table_ipa as usize,
            );
            info!(
                "ivc: zone {} channel {} peer {} shared GPA {:#x} HPA {:#x}",
                self.id(),
                config.ivc_id,
                config.peer_id,
                config.shared_mem_ipa,
                start_paddr
            );
        }

        IVC_INFOS.lock().insert(self.id(), IvcInfo::from(configs));
        Ok(())
    }
}

const CT_IVC_ID: GuestPhysAddr = 0x00;
const CT_MAX_PEERS: GuestPhysAddr = 0x04;
const CT_RW_SEC_SIZE: GuestPhysAddr = 0x08;
const CT_OUT_SEC_SIZE: GuestPhysAddr = 0x0c;
const CT_PEER_ID: GuestPhysAddr = 0x10;
const CT_IPI_INVOKE: GuestPhysAddr = 0x14;
const CT_END: GuestPhysAddr = 0x18;

fn read_control_register(ivc_id: u32, zone_id: usize, offset: GuestPhysAddr) -> HvResult<u32> {
    let records = IVC_RECORDS.lock();
    let record = records.get(&ivc_id).ok_or(hv_err!(ENODEV))?;
    match offset {
        CT_IVC_ID => Ok(ivc_id),
        CT_MAX_PEERS => Ok(record.max_peers),
        CT_RW_SEC_SIZE => Ok(record.rw_sec_size),
        CT_OUT_SEC_SIZE => Ok(record.out_sec_size),
        CT_PEER_ID => record
            .peer_infos
            .iter()
            .find(|(_, peer)| peer.zone_id == zone_id as u32)
            .map(|(peer_id, _)| *peer_id)
            .ok_or(hv_err!(ENODEV)),
        CT_IPI_INVOKE => Ok(0),
        _ => hv_result_err!(EFAULT),
    }
}

fn notify_peer(target_zone: u32, irq: u32) -> HvResult {
    let zone = find_zone(target_zone as usize).ok_or(hv_err!(ENODEV))?;
    let target_cpu = zone.read().cpu_set().first_cpu().ok_or(hv_err!(ENODEV))?;
    // IVC is a doorbell. Re-arm the virtual line so a stale asserted state
    // cannot suppress a later notification.
    if !set_guest_irq_line(target_cpu, irq as usize, false) {
        return hv_result_err!(EINVAL, "invalid LoongArch IVC interrupt line");
    }
    if !set_guest_irq_line(target_cpu, irq as usize, true) {
        return hv_result_err!(EINVAL, "invalid LoongArch IVC interrupt line");
    }
    Ok(())
}

pub fn mmio_ivc_handler(mmio: &mut MMIOAccess, base: usize) -> HvResult {
    let zone_id = this_zone_id();
    let offset = if mmio.address >= base {
        mmio.address - base
    } else {
        mmio.address
    };
    if mmio.size == 0 || mmio.size > 8 || offset + mmio.size > CT_END {
        return hv_result_err!(EFAULT);
    }

    let infos = IVC_INFOS.lock();
    let info = infos.get(&zone_id).ok_or(hv_err!(ENODEV))?;
    let ivc_id = (0..info.len as usize)
        .find(|index| info.ivc_ct_ipas[*index] == base as u64)
        .map(|index| info.ivc_ids[index])
        .ok_or(hv_err!(ENODEV))?;
    drop(infos);

    if mmio.is_write {
        if offset == CT_IPI_INVOKE && mmio.size == core::mem::size_of::<u32>() {
            let target = {
                let records = IVC_RECORDS.lock();
                let record = records.get(&ivc_id).ok_or(hv_err!(ENODEV))?;
                record
                    .peer_infos
                    .get(&(mmio.value as u32))
                    .map(|peer| (peer.zone_id, peer.irq_num))
                    .ok_or(hv_err!(EINVAL))?
            };
            return notify_peer(target.0, target.1);
        }
        return hv_result_err!(EFAULT);
    }

    let mut value = 0usize;
    for byte_index in 0..mmio.size {
        let byte_offset = offset + byte_index;
        let register = byte_offset & !0x3;
        let shift = (byte_offset & 0x3) * 8;
        let byte = (read_control_register(ivc_id, zone_id, register)? >> shift) & 0xff;
        value |= (byte as usize) << (byte_index * 8);
    }
    mmio.value = value;
    Ok(())
}
