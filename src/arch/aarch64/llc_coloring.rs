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
//  li041 <3253290158@qq.com>

use aarch64_cpu::registers::{Readable, Writeable, CCSIDR_EL1, CLIDR_EL1, CSSELR_EL1};

use crate::arch::sysreg::read_sysreg;

pub fn get_llc_way_size() -> Option<usize> {
    const CLIDR_CTYPEN_LEVELS: usize = 7;
    const UNIFIED_CACHE: u64 = 0b100;
    const CSSELR_EL1_OFFSET: usize = 0x1;

    let clidr_el1 = CLIDR_EL1.get();
    // for restore
    let csselr_el1 = CSSELR_EL1.get();
    let mut level = 0;
    for n in (1..=CLIDR_CTYPEN_LEVELS).rev() {
        let ctype_n = ((clidr_el1) >> (n * 3)) & 0x7;
        if ctype_n == UNIFIED_CACHE {
            level = n;
            break;
        }
    }
    if level == 0 {
        return None;
    }
    CSSELR_EL1.set(((level - 1) << CSSELR_EL1_OFFSET) as u64);
    unsafe {
        core::arch::asm!("isb");
    }
    let ccsidr_el1 = CCSIDR_EL1.get();
    // bits[2:0] = (Log2(Number of bytes in cache line)) - 4
    let line_size = 1 << ((ccsidr_el1 & 0x7) + 4);
    let num_sets = CCSIDR_EL1.get_num_sets() + 1;
    CSSELR_EL1.set(csselr_el1);
    unsafe {
        core::arch::asm!("isb");
    }
    let way_size = line_size * num_sets;
    info!(
        "LLC found: L{} (line size: {} bytes, sets num: {})",
        level, line_size, num_sets
    );
    Some(way_size as usize)
}

pub fn arch_llc_coloring_init() {
    let id_aa64pfr0_el1 = read_sysreg!(id_aa64pfr0_el1);
    let major = (id_aa64pfr0_el1 >> 40) & 0xf;
    let id_aa64pfr1_el1 = read_sysreg!(id_aa64pfr1_el1);
    let minor = (id_aa64pfr1_el1 >> 16) & 0xf;
    info!(
        "id_aa64pfr0_el1 = {:#b}, id_aa64pfr1_el1 = {:#b}",
        id_aa64pfr0_el1, id_aa64pfr1_el1
    );
    info!("MPAM extension version: {}.{}", major, minor);
}
