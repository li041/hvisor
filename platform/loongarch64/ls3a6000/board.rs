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
//      Yulong Han <wheatfox17@icloud.com>
//
use crate::pci_dev;
use crate::{arch::zone::HvArchZoneConfig, config::*, pci::vpci_dev::VpciDevType};

pub const BOARD_NAME: &str = "ls3a6000";

pub const BOARD_NCPUS: usize = 4;

pub const ROOT_ZONE_DTB_ADDR: u64 = 0x10000f000;
pub const ROOT_ZONE_KERNEL_ADDR: u64 = 0x200000;
pub const ROOT_ZONE_ENTRY: u64 = 0x9000000000dc7000;
pub const ROOT_ZONE_CPUS: u64 = 1 << 0;

pub const ROOT_ZONE_NAME: &str = "root-linux-la64";

pub const ROOT_ZONE_MEMORY_REGIONS: &[HvConfigMemoryRegion] = &[
    /* legacy/low RAM not in DTS memory@ — map first for early boot */
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_RAM,
        physical_start: 0x1000,
        virtual_start: 0x0,
        size: 0x10000,
    }, // 0x0
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_RAM,
        physical_start: 0x10000,
        virtual_start: 0x10000,
        size: 0x1f0000,
    }, // 0x10000..0x200000 (covers 0xf0000 legacy alias + boot data)
    /* memory regions — aligned with loongson-3a5000-hvisor-root.dts memory@ */
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_RAM,
        physical_start: 0x00200000,
        virtual_start: 0x00200000,
        size: 0x0ec00000,
    }, // bank0
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_RAM,
        physical_start: 0x90400000,
        virtual_start: 0x90400000,
        size: 0x67b60000,
    }, // bank1
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_RAM,
        physical_start: 0xf7f70000,
        virtual_start: 0xf7f70000,
        size: 0x05f10000,
    }, // bank2
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_RAM,
        physical_start: 0xfe440000,
        virtual_start: 0xfe440000,
        size: 0xf1bc0000,
    }, // bank3
    /* shmem — aligned with loongson-3a5000-hvisor-root.dts reserved-memory shmem@0 */
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_RAM,
        physical_start: 0x2_00c0_0000,
        virtual_start: 0x2_00c0_0000,
        size: 0x04000000,
    }, // shmem@0: <0x2 0x00c00000 0 0x1c000000>
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_RAM,
        physical_start: 0x2_04c0_0000,
        virtual_start: 0x2_04c0_0000,
        size: 0x0400_0000,
    }, // shmem@1: <0x2 0x01000000 0 0x04000000>
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_RAM,
        physical_start: 0x2_08c0_0000,
        virtual_start: 0x2_08c0_0000,
        size: 0x0400_0000,
    }, // shmem@2: <0x2 0x01400000 0 0x04000000>
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_RAM,
        physical_start: 0x2_0cc0_0000,
        virtual_start: 0x2_0cc0_0000,
        size: 0x0400_0000,
    }, // shmem@3: <0x2 0x01800000 0 0x04000000>
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_RAM,
        physical_start: 0x2_1cc0_0000,
        virtual_start: 0x2_1cc0_0000,
        size: 0x1_0000_0000,
    }, // bank5: <0x2 0x1cc00000 0x1 0x0>
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_RAM,
        physical_start: 0x3_1cc0_0000,
        virtual_start: 0x3_1cc0_0000,
        size: 0x1_0000_0000,
    }, // bank6: <0x3 0x1cc00000 0x1 0x0>
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_RAM,
        physical_start: 0x4_1cc0_0000,
        virtual_start: 0x4_1cc0_0000,
        size: 0x1_0000_0000,
    }, // bank7: <0x4 0x1cc00000 0x1 0x0>
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_RAM,
        physical_start: 0x5_1cc0_0000,
        virtual_start: 0x5_1cc0_0000,
        size: 0x1_0000_0000,
    }, // bank8: <0x5 0x1cc00000 0x1 0x0>
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_RAM,
        physical_start: 0x6_1cc0_0000,
        virtual_start: 0x6_1cc0_0000,
        size: 0x1_0000_0000,
    }, // bank9: <0x6 0x1cc00000 0x1 0x0>
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_RAM,
        physical_start: 0x7_1cc0_0000,
        virtual_start: 0x7_1cc0_0000,
        size: 0x1_0000_0000,
    }, // bank10: <0x7 0x1cc00000 0x1 0x0>
    /* devices and controllers */
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0x1fe00000,
        virtual_start: 0x1fe00000,
        size: 0x1000,
    }, // uart0
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0x10080000,
        virtual_start: 0x10080000,
        size: 0x1000,
    }, // uart1, passthrough now
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0x100d0000,
        virtual_start: 0x100d0000,
        size: 0x1000,
    }, // rtc, passthrough now
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0x10000000,
        virtual_start: 0x10000000,
        size: 0x1000,
    }, // pch-pic irq controller
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0x1001_0000,
        virtual_start: 0x1001_0000,
        size: 0x0001_0000,
    }, // pch/ls7a misc (e.g. 0x10010490)
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0x100A_0000,
        virtual_start: 0x100A_0000,
        size: 0x1000,
    }, // LS7A PWM0-3 (ACPI LOON0006)
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0x0E0010000000,
        virtual_start: 0x0E0010000000,
        size: 0x1000,
    }, // ACPI PCI0.THS1 board thermal (reg @ +0x414)
    /* PCI related stuffs ... */
    // HvConfigMemoryRegion {
    //     mem_type: MEM_TYPE_IO,
    //     physical_start: 0x1a000000,
    //     virtual_start: 0x1a000000,
    //     size: 0x02000000,
    // }, // pci
    // HvConfigMemoryRegion {
    //     mem_type: MEM_TYPE_IO,
    //     physical_start: 0xefe_0000_0000,
    //     virtual_start: 0xfe_0000_0000,
    //     size: 0x20000000,
    // }, // pci config space (HT)
    // HvConfigMemoryRegion {
    //     mem_type: MEM_TYPE_IO,
    //     physical_start: 0x18408000,
    //     virtual_start: 0x18408000,
    //     size: 0x00008000,
    // }, // pci config space (HT)
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0x18408000,
        virtual_start: 0x18408000,
        size: 0x00008000,
    }, // pci io resource
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0x60000000,
        virtual_start: 0x60000000,
        size: 0x20000000,
    }, // pci mem resource
];

pub const IRQ_WAKEUP_VIRTIO_DEVICE: usize = 32 + 0x20;
pub const ROOT_ZONE_IRQS_BITMAP: &[BitmapWord] = &get_irqs_bitmap(&[]);
pub const ROOT_ARCH_ZONE_CONFIG: HvArchZoneConfig = HvArchZoneConfig { dummy: 0 };
/// IVC 专用 EXTIOI MSI（全芯片 hwirq 唯一；每 CPU 一个 48 向量 bank：64+cpu*48 ..）。
///
/// 3A6000 固定分配：
/// | CPU | IVC 条数 | slot | hwirq   |
/// |-----|----------|------|---------|
/// | 0   | 3        | 0–2  | 100–102 |
/// | 1   | 1        | 0    | 148     |
/// | 2   | 1        | 0    | 196     |
/// | 3   | 1        | 0    | 244     |
pub const IVC_MSIX_SLOTS_CPU0: u32 = 3;
pub const IVC_MSIX_SLOTS_PER_OTHER_CPU: u32 = 1;
pub const IVC_MSIX_EXTIOI_COUNT: u32 = IVC_MSIX_SLOTS_CPU0 + IVC_MSIX_SLOTS_PER_OTHER_CPU * 3;
pub const IVC_MSIX_SLOT_IN_BANK: u32 = 36;
pub const IVC_MSIX_EXTIOI_BASE: u32 = 64 + IVC_MSIX_SLOT_IN_BANK; // 100 on CPU0

pub const fn ivc_msix_hwirq_for_pcpu(pcpu: u32, slot: u32) -> u32 {
    64 + pcpu * 48 + IVC_MSIX_SLOT_IN_BANK + slot
}

pub const fn ivc_msix_hwirq(slot: u32) -> u32 {
    ivc_msix_hwirq_for_pcpu(0, slot)
}

pub const IVC_MSIX_CPU1_HWIRQ: u32 = ivc_msix_hwirq_for_pcpu(1, 0);
pub const IVC_MSIX_CPU2_HWIRQ: u32 = ivc_msix_hwirq_for_pcpu(2, 0);
pub const IVC_MSIX_CPU3_HWIRQ: u32 = ivc_msix_hwirq_for_pcpu(3, 0);

pub const ROOT_ZONE_IVC_CONFIG: [HvIvcConfig; 3] = [
    HvIvcConfig {
        ivc_id: 0,
        peer_id: 0,
        control_table_ipa: 0x2_00be_0000,
        shared_mem_ipa: 0x2_00be_1000,
        rw_sec_size: 0,
        out_sec_size: 0x1000,
        interrupt_num: ivc_msix_hwirq(0),
        max_peers: 2,
    },
    HvIvcConfig {
        ivc_id: 1,
        peer_id: 0,
        control_table_ipa: 0x2_00be_3000,
        shared_mem_ipa: 0x2_00be_4000,
        rw_sec_size: 0,
        out_sec_size: 0x1000,
        interrupt_num: ivc_msix_hwirq(1),
        max_peers: 2,
    },
    HvIvcConfig {
        ivc_id: 2,
        peer_id: 0,
        control_table_ipa: 0x2_00be_6000,
        shared_mem_ipa: 0x2_00be_7000,
        rw_sec_size: 0,
        out_sec_size: 0x1000,
        interrupt_num: ivc_msix_hwirq(2),
        max_peers: 2,
    },
];

pub const ROOT_PCI_CONFIG: [HvPciConfig; 1] = [HvPciConfig {
    bus_range_begin: 0x0,
    bus_range_end: 0xff,
    ecam_base: 0xfe00000000,
    ecam_size: 0x20000000,
    io_base: 0x18408000,
    io_size: 0x8000,
    pci_io_base: 0x00008000,
    mem32_base: 0x0,
    mem32_size: 0x0,
    pci_mem32_base: 0x0,
    mem64_base: 0x60000000,
    mem64_size: 0x20000000,
    pci_mem64_base: 0x60000000,
    domain: 0x0,
}];

/* 00:00.0, 00:00.1, 00:00.2, 00:00.3, 00:04.0, 00:04.1*/
/* 00:05.0, 00:05.1, 00:06.0, 00:06.1, 00:06.2 */
/* 00:07.0, 00:08.0, 00:09.0, 00:0a.0, 00:0b.0 */
/* 00:0c.0, 00:0d.0, 00:0f.0, 00:10.0, 00:13.0 */
/* 00:16.0, 00:19.0, 02:00.0, 05:00.0 */
/* BUS 8 on X16 slot */
/* 08:00.0, 08:00.1, 08:00.2, 08:00.3 net */
/* BUS 6 on X4 slot */
/* 06:00.0, 06:00.1, 06:00.2, 06:00.3 net */
pub const ROOT_PCI_DEVS: [HvPciDevConfig; 26] = [
    pci_dev!(0x0, 0x0, 0x0, 0x0, VpciDevType::Physical), // 00:00.0
    pci_dev!(0x0, 0x0, 0x0, 0x1, VpciDevType::Physical), // 00:00.1
    pci_dev!(0x0, 0x0, 0x0, 0x2, VpciDevType::Physical), // 00:00.2
    pci_dev!(0x0, 0x0, 0x0, 0x3, VpciDevType::Physical), // 00:00.3
    pci_dev!(0x0, 0x0, 0x4, 0x0, VpciDevType::Physical), // 00:04.0
    pci_dev!(0x0, 0x0, 0x4, 0x1, VpciDevType::Physical), // 00:04.1
    pci_dev!(0x0, 0x0, 0x5, 0x0, VpciDevType::Physical), // 00:05.0
    pci_dev!(0x0, 0x0, 0x5, 0x1, VpciDevType::Physical), // 00:05.1
    pci_dev!(0x0, 0x0, 0x6, 0x0, VpciDevType::Physical), // 00:06.0
    pci_dev!(0x0, 0x0, 0x6, 0x1, VpciDevType::Physical), // 00:06.1
    pci_dev!(0x0, 0x0, 0x6, 0x2, VpciDevType::Physical), // 00:06.2
    pci_dev!(0x0, 0x0, 0x7, 0x0, VpciDevType::Physical), // 00:07.0
    pci_dev!(0x0, 0x0, 0x8, 0x0, VpciDevType::Physical), // 00:08.0
    pci_dev!(0x0, 0x0, 0x9, 0x0, VpciDevType::Physical), // 00:09.0
    pci_dev!(0x0, 0x0, 0xa, 0x0, VpciDevType::Physical), // 00:0a.0
    pci_dev!(0x0, 0x0, 0xb, 0x0, VpciDevType::Physical), // 00:0b.0
    pci_dev!(0x0, 0x0, 0xc, 0x0, VpciDevType::Physical), // 00:0c.0
    pci_dev!(0x0, 0x0, 0xd, 0x0, VpciDevType::Physical), // 00:0d.0
    pci_dev!(0x0, 0x0, 0xf, 0x0, VpciDevType::Physical), // 00:0f.0
    pci_dev!(0x0, 0x0, 0x10, 0x0, VpciDevType::Physical), // 00:10.0
    pci_dev!(0x0, 0x0, 0x13, 0x0, VpciDevType::Physical), // 00:13.0
    pci_dev!(0x0, 0x0, 0x16, 0x0, VpciDevType::Physical), // 00:16.0
    pci_dev!(0x0, 0x0, 0x19, 0x0, VpciDevType::Physical), // 00:19.0
    pci_dev!(0x0, 0x2, 0x0, 0x0, VpciDevType::Physical), // 02:00.0
    pci_dev!(0x0, 0x5, 0x0, 0x0, VpciDevType::Physical), // 05:00.0
    pci_dev!(0x0, 0x6, 0x0, 0x0, VpciDevType::Physical), // 06:00.0
                                                         // pci_dev!(0x0, 0x6, 0x0, 0x1, VpciDevType::Physical), // 06:00.1
                                                         // pci_dev!(0x0, 0x6, 0x0, 0x2, VpciDevType::Physical), // 06:00.2
                                                         // pci_dev!(0x0, 0x6, 0x0, 0x3, VpciDevType::Physical), // 06:00.3
];

// bus << 8 | dev << 5 | func << 3

// pub const ROOT_PCI_DEVS: [u64; 0] = [];

// "alloc_pci_devs": [0,1,2,3,32,33,40,41,56,64,72,80,88,96,104,120,128,152,176,200,512,1280,2051]
