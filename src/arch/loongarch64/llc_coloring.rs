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

use loongArch64::cpu::CPUCFG;

pub fn get_llc_way_size() -> Option<u64> {
    let cfg_10 = CPUCFG::read(10);
    let mut level = 0;
    for (lvl, present_bit, unify_bit) in [(3, 10, 11), (2, 3, 4), (1, 0, 1)] {
        let present = (cfg_10 >> present_bit) & 1 != 0;
        let unify = (cfg_10 >> unify_bit) & 1 != 0;
        if present && unify {
            level = lvl;
            break;
        }
    }
    let cfg_index = match level {
        1 => 0x11,
        2 => 0x13,
        3 => 0x14,
        _ => return None,
    };
    let cfg = CPUCFG::read(cfg_index);
    let num_sets = 1 << cfg.get_bits(16, 23);
    let line_size = 1 << cfg.get_bits(24, 30);

    let way_size = num_sets * line_size;
    info!(
        "LLC found: L{} (line size: {} bytes, sets num: {}",
        level, line_size, num_sets
    );
    Some(way_size)
}
