// Copyright (c) 2025 Syswonder
// hvisor is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan PSL v2.
// You may obtain a copy of Mulan PSL v2 at:
//     http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY KIND, EITHER
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO NON-INFRINGEMENT, MERCHANTABILITY OR
// FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.
//
// Syswonder Website:
//      https://www.syswonder.org
//
// Authors:
//  ForeverYolo <2572131118@qq.com>

use crate::arch::cpu::this_cpu_id;
use crate::arch::ivc::{IvcInfo, IVC_INFOS};
use crate::arch::mm::LOONGARCH64_CACHED_DMW_PREFIX;
use crate::config::HvZoneConfig;
use crate::config::CONFIG_MAGIC_VERSION;
use crate::cpu_data::this_zone;
use crate::device::virtio_trampoline::MAX_DEVS;
use crate::hypercall::HyperCall;
use crate::hypercall::HyperCallResult;
use crate::zone::this_zone_id;

impl<'a> HyperCall<'a> {
    /// 将本 zone 的 `IvcInfo` 写入客户机 `ivc_info_ipa` 指向的 GPA（经 S2 翻译 + DMW 可缓存前缀后由 HV 存取）。
    pub fn hv_ivc_info(&mut self, ivc_info_ipa: u64) -> HyperCallResult {
        let zone_id = this_zone_id();
        let zone = this_zone();
        let hpa = match unsafe { zone.read().gpm().page_table_query(ivc_info_ipa as _) } {
            Ok((pa, _, _)) => pa as u64,
            Err(_) => {
                return hv_result_err!(EFAULT, "hv_ivc_info: ipa not mapped or query failed");
            }
        };
        let hva = hpa | LOONGARCH64_CACHED_DMW_PREFIX;
        let ivc_info_dst = unsafe { &mut *(hva as *mut IvcInfo) };
        let ivc_infos = IVC_INFOS.lock();
        let zone_snapshot = ivc_infos.get(&(zone_id as usize));
        match zone_snapshot {
            Some(src) => *ivc_info_dst = *src,
            None => {
                return hv_result_err!(
                    ENODEV,
                    format!("Zone {} has no ivc config!", zone_id)
                );
            }
        }
        HyperCallResult::Ok(0)
    }

    pub fn wait_for_interrupt(&mut self, irq_list: &mut [u64; MAX_DEVS + 1]) {
        use crate::device::irqchip::ls7a2000::*;
        let status = GLOBAL_IRQ_INJECT_STATUS.lock();
        drop(status);
        irq_list[0] = 0; // CAUTION: this is a workaround for loongarch64
    }

    pub fn hv_zone_config_check(&self, magic_version: *mut u64) -> HyperCallResult {
        let magic_version_raw = magic_version as u64;
        let magic_version_hva = magic_version_raw | crate::arch::mm::LOONGARCH64_CACHED_DMW_PREFIX;
        let magic_version_hva = magic_version_hva as *mut u64;
        debug!(
            "hv_zone_config_check: magic_version target addr to write = {:#x?}",
            magic_version_hva
        );
        unsafe {
            core::ptr::write(magic_version_hva, CONFIG_MAGIC_VERSION as _);
        }
        HyperCallResult::Ok(0)
    }

    pub fn hv_get_real_pa(&mut self, config_addr: u64) -> u64 {
        // LoongArch64 uses a specific prefix for cached memory addresses.
        let config_addr = config_addr as u64 | crate::arch::mm::LOONGARCH64_CACHED_DMW_PREFIX;
        return config_addr;
    }

    pub fn hv_get_real_list_pa(&mut self, list_addr: u64) -> u64 {
        // LoongArch64 does not have a specific prefix for cached memory, so we return the address as is.
        return list_addr;
    }

    pub fn check_cpu_id(&self) {
        let cpuid = this_cpu_id();
        assert_eq!(cpuid, 0);
    }

    pub fn hv_virtio_get_irq(&self, virtio_irq: *mut u32) -> HyperCallResult {
        trace!("hv_virtio_get_irq is not need for LoongArch64");
        HyperCallResult::Ok(0)
    }
}
