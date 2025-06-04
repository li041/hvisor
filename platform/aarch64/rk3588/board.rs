use crate::{arch::zone::HvArchZoneConfig, config::*};

// pub const ROOT_ZONE_DTB_ADDR: u64 = 0x10000000;
// pub const ROOT_ZONE_KERNEL_ADDR: u64 = 0x09400000;
// pub const ROOT_ZONE_ENTRY: u64 = 0x09400000;
pub const ROOT_ZONE_DTB_ADDR: u64 = 0x10000000;
pub const ROOT_ZONE_KERNEL_ADDR: u64 = 0x09400000;
pub const ROOT_ZONE_ENTRY: u64 = 0x09400000;
//pub const ROOT_ZONE_CPUS: u64 = (1 << 0);
pub const ROOT_ZONE_CPUS: u64 = (1 << 0)|(1<<1);

pub const ROOT_ZONE_NAME: &str = "root-linux";
// 修改数量
pub const ROOT_ZONE_MEMORY_REGIONS: [HvConfigMemoryRegion; 358] = [
/*    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_RAM,
        physical_start: 0x0,
        virtual_start: 0x0,
        // 调整内存分配的大小，防止内存重叠
        // size: 0x7ffffff000,
        size: 0xf0000000,
    },
    */
     HvConfigMemoryRegion {
        mem_type: MEM_TYPE_RAM,
        physical_start: 0x100000,
        virtual_start: 0x100000,
        size: 0xf0000,
    },
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_RAM,
        physical_start: 0x200000,
        virtual_start: 0x200000,
        size: 0x8200000,
    },
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_RAM,
        physical_start: 0x9400000,
        virtual_start: 0x9400000,
        size: 0xe6c00000,
    },
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_RAM,
        physical_start: 0x100000000,
        virtual_start: 0x100000000,
        size: 0x2fc000000,
    },
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_RAM,
        physical_start: 0x3fc500000,
        virtual_start: 0x3fc500000,
        size: 0x3a00000,
    },
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_RAM,
        physical_start: 0x4f0000000,
        virtual_start: 0x4f0000000,
        size: 0x10000000,
    },


    // HvConfigMemoryRegion {
    //     mem_type: MEM_TYPE_RAM,
    //     physical_start: 0x0000000000200000,
    //     virtual_start: 0x0000000000200000,
    //     size: 0x00000000083fffff - 0x0000000000200000 + 1,
    // }, // ram
    // HvConfigMemoryRegion {
    //     mem_type: MEM_TYPE_RAM,
    //     physical_start: 0x0000000009400000,
    //     virtual_start: 0x0000000009400000,
    //     size: 0x00000000efffffff - 0x0000000009400000 + 1,
    // }, // ram
    // HvConfigMemoryRegion {
    //     mem_type: MEM_TYPE_RAM,
    //     physical_start: 0x0000000100000000,
    //     virtual_start: 0x0000000100000000,
    //     size: 0x00000003fbffffff - 0x0000000100000000 + 1,
    // }, // ram
    // HvConfigMemoryRegion {
    //     mem_type: MEM_TYPE_RAM,
    //     physical_start: 0x00000003fc500000,
    //     virtual_start: 0x00000003fc500000,
    //     size: 0x00000003ffefffff - 0x00000003fc500000 + 1,
    // }, // ram
    // HvConfigMemoryRegion {
    //     mem_type: MEM_TYPE_RAM,
    //     physical_start: 0x00000004f0000000,
    //     virtual_start: 0x00000004f0000000,
    //     size: 0x00000004ffffffff - 0x00000004f0000000 + 1,
    // }, // ram
    // HvConfigMemoryRegion {
    //     mem_type: MEM_TYPE_IO,
    //     physical_start: 0xfeb50000,
    //     virtual_start: 0xfeb50000,
    //     size: 0x1000,
    // }, // uart
    // HvConfigMemoryRegion {
    //     mem_type: MEM_TYPE_IO,
    //     physical_start: 0xfd000000,
    //     virtual_start: 0xfd000000,
    //     size: 0x1000000,
    // },
    // HvConfigMemoryRegion {
    //     mem_type: MEM_TYPE_IO,
    //     physical_start: 0x100000,
    //     virtual_start: 0x100000,
    //     size: 0x100000,
    // }
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xf0000000, 
        virtual_start: 0xf0000000, 
        size: 0x1000000, 
    },//PCIe3_4L_S
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xf1000000, 
        virtual_start: 0xf1000000, 
        size: 0x1000000, 
    },//PCIe3_2L_S
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xf2000000, 
        virtual_start: 0xf2000000, 
        size: 0x1000000, 
    },//PCIe3_1L0_S
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xf3000000, 
        virtual_start: 0xf3000000, 
        size: 0x1000000, 
    },//PCIe3_1L1_S
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xf4000000, 
        virtual_start: 0xf4000000, 
        size: 0x1000000, 
    },//PCIe3_1L2_S
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xf5000000, 
        virtual_start: 0xf5000000, 
        size: 0x400000, 
    },//PCIe3_4L_DBI
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xf5400000, 
        virtual_start: 0xf5400000, 
        size: 0x400000, 
    },//PCIe3_2L_DBI
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xf5800000, 
        virtual_start: 0xf5800000, 
        size: 0x400000, 
    },//PCIe3_1L0_DBI
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xf5c00000, 
        virtual_start: 0xf5c00000, 
        size: 0x400000, 
    },//PCIe3_1L1_DBI
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xf6000000, 
        virtual_start: 0xf6000000, 
        size: 0x400000, 
    },//PCIe3_1L2_DBI
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xf6400000, 
        virtual_start: 0xf6400000, 
        size: 0xb00000, 
    },//Reserved
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xf6f00000, 
        virtual_start: 0xf6f00000, 
        size: 0x10000, 
    },//MCU_TCM
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xf6f10000, 
        virtual_start: 0xf6f10000, 
        size: 0x10000, 
    },//MCU_CACHE
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xf6f20000, 
        virtual_start: 0xf6f20000, 
        size: 0x10000, 
    },//MCU_RAM_TEST
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xf6f30000, 
        virtual_start: 0xf6f30000, 
        size: 0xd0000, 
    },//Reserved
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xf7000000, 
        virtual_start: 0xf7000000, 
        size: 0x1000000, 
    },//DDRCTL_0
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xf8000000, 
        virtual_start: 0xf8000000, 
        size: 0x1000000, 
    },//DDRCTL_1
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xf9000000, 
        virtual_start: 0xf9000000, 
        size: 0x1000000, 
    },//DDRCTL_2
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfa000000, 
        virtual_start: 0xfa000000, 
        size: 0x1000000, 
    },//DDRCTL_3
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfb000000, 
        virtual_start: 0xfb000000, 
        size: 0x1000000, 
    },//GPU G610
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfc000000, 
        virtual_start: 0xfc000000, 
        size: 0x400000, 
    },//USB3_0
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfc400000, 
        virtual_start: 0xfc400000, 
        size: 0x400000, 
    },//USB3_1
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfc800000, 
        virtual_start: 0xfc800000, 
        size: 0x80000, 
    },//USB2HOST_0
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfc880000, 
        virtual_start: 0xfc880000, 
        size: 0x80000, 
    },//USB2HOST_1
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfc900000, 
        virtual_start: 0xfc900000, 
        size: 0x200000, 
    },//MMU600_PCIE
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfcb00000, 
        virtual_start: 0xfcb00000, 
        size: 0x200000, 
    },//MMU600_PHP
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfcd00000, 
        virtual_start: 0xfcd00000, 
        size: 0x400000, 
    },//USB3_2
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd100000, 
        virtual_start: 0xfd100000, 
        size: 0x80000, 
    },//DAPLITE2
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd180000, 
        virtual_start: 0xfd180000, 
        size: 0x80000, 
    },//Reserved
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd200000, 
        virtual_start: 0xfd200000, 
        size: 0x10000, 
    },//Reserved
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd210000, 
        virtual_start: 0xfd210000, 
        size: 0x370000, 
    },//Reserved
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd580000, 
        virtual_start: 0xfd580000, 
        size: 0x2000, 
    },//PMU0_SGRF
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd582000, 
        virtual_start: 0xfd582000, 
        size: 0x2000, 
    },//PMU1_SGRF
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd584000, 
        virtual_start: 0xfd584000, 
        size: 0x2000, 
    },//Reserved
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd586000, 
        virtual_start: 0xfd586000, 
        size: 0x1000, 
    },//BUS_SGRF
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd587000, 
        virtual_start: 0xfd587000, 
        size: 0x1000, 
    },//DSU_SGRF
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd588000, 
        virtual_start: 0xfd588000, 
        size: 0x2000, 
    },//PMU0_GRF
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd58a000, 
        virtual_start: 0xfd58a000, 
        size: 0x2000, 
    },//PMU1_GRF
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd58c000, 
        virtual_start: 0xfd58c000, 
        size: 0x4000, 
    },//SYS_GRF
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd590000, 
        virtual_start: 0xfd590000, 
        size: 0x2000, 
    },//BIGCORE0_GRF
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd592000, 
        virtual_start: 0xfd592000, 
        size: 0x2000, 
    },//BIGCORE1_GRF
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd594000, 
        virtual_start: 0xfd594000, 
        size: 0x4000, 
    },//LITCORE_GRF
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd598000, 
        virtual_start: 0xfd598000, 
        size: 0x4000, 
    },//DSU_GRF
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd59c000, 
        virtual_start: 0xfd59c000, 
        size: 0x1000, 
    },//DDR01_GRF
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd59d000, 
        virtual_start: 0xfd59d000, 
        size: 0x1000, 
    },//DDR23_GRF
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd59e000, 
        virtual_start: 0xfd59e000, 
        size: 0x2000, 
    },//CENTER_GRF
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd5a0000, 
        virtual_start: 0xfd5a0000, 
        size: 0x2000, 
    },//GPU_GRF
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd5a2000, 
        virtual_start: 0xfd5a2000, 
        size: 0x2000, 
    },//NPU_GRF
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd5a4000, 
        virtual_start: 0xfd5a4000, 
        size: 0x2000, 
    },//VOP_GRF
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd5a6000, 
        virtual_start: 0xfd5a6000, 
        size: 0x2000, 
    },//VO0_GRF
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd5a8000, 
        virtual_start: 0xfd5a8000, 
        size: 0x4000, 
    },//VO1_GRF
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd5ac000, 
        virtual_start: 0xfd5ac000, 
        size: 0x4000, 
    },//USB_GRF
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd5b0000, 
        virtual_start: 0xfd5b0000, 
        size: 0x4000, 
    },//PHP_GRF
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd5b4000, 
        virtual_start: 0xfd5b4000, 
        size: 0x1000, 
    },//CSIDPHY0_GRF
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd5b5000, 
        virtual_start: 0xfd5b5000, 
        size: 0x1000, 
    },//CSIDPHY1_GRF
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd5b6000, 
        virtual_start: 0xfd5b6000, 
        size: 0x2000, 
    },//Reserved
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd5b8000, 
        virtual_start: 0xfd5b8000, 
        size: 0x4000, 
    },//PCIe3PHY_GRF
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd5bc000, 
        virtual_start: 0xfd5bc000, 
        size: 0x4000, 
    },//PIPE_PHY0_GRF
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd5c0000, 
        virtual_start: 0xfd5c0000, 
        size: 0x4000, 
    },//PIPE_PHY1_GRF
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd5c4000, 
        virtual_start: 0xfd5c4000, 
        size: 0x4000, 
    },//PIPE_PHY2_GRF
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd5c8000, 
        virtual_start: 0xfd5c8000, 
        size: 0x4000, 
    },//USBDPPHY0_GRF
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd5cc000, 
        virtual_start: 0xfd5cc000, 
        size: 0x4000, 
    },//USBDPPHY1_GRF
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd5d0000, 
        virtual_start: 0xfd5d0000, 
        size: 0x4000, 
    },//USB2PHY0_GRF
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd5d4000, 
        virtual_start: 0xfd5d4000, 
        size: 0x4000, 
    },//USB2PHY1_GRF
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd5d8000, 
        virtual_start: 0xfd5d8000, 
        size: 0x4000, 
    },//USB2PHY2_GRF
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd5dc000, 
        virtual_start: 0xfd5dc000, 
        size: 0x4000, 
    },//USB2PHY3_GRF
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd5e0000, 
        virtual_start: 0xfd5e0000, 
        size: 0x4000, 
    },//HDPTXPHY0_GRF
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd5e4000, 
        virtual_start: 0xfd5e4000, 
        size: 0x4000, 
    },//HDPTXPHY1_GRF
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd5e8000, 
        virtual_start: 0xfd5e8000, 
        size: 0x4000, 
    },//MIPIDCPHY0_GRF
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd5ec000, 
        virtual_start: 0xfd5ec000, 
        size: 0x4000, 
    },//MIPIDCPHY1_GRF
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd5f0000, 
        virtual_start: 0xfd5f0000, 
        size: 0x4000, 
    },//PMU1_IOC
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd5f4000, 
        virtual_start: 0xfd5f4000, 
        size: 0x4000, 
    },//PMU2_IOC
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd5f8000, 
        virtual_start: 0xfd5f8000, 
        size: 0x1000, 
    },//BUS_IOC
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd5f9000, 
        virtual_start: 0xfd5f9000, 
        size: 0x1000, 
    },//VCCIO1_4_IOC
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd5fa000, 
        virtual_start: 0xfd5fa000, 
        size: 0x1000, 
    },//VCCIO3_5_IOC
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd5fb000, 
        virtual_start: 0xfd5fb000, 
        size: 0x1000, 
    },//VCCIO2_IOC
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd5fc000, 
        virtual_start: 0xfd5fc000, 
        size: 0x1000, 
    },//VCCIO6_IOC
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd5fd000, 
        virtual_start: 0xfd5fd000, 
        size: 0x1000, 
    },//EMMC_IOC
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd5fe000, 
        virtual_start: 0xfd5fe000, 
        size: 0x2000, 
    },//Reserved
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd600000, 
        virtual_start: 0xfd600000, 
        size: 0x100000, 
    },//SYSTEM_SRAM
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd700000, 
        virtual_start: 0xfd700000, 
        size: 0x40000, 
    },//PMU_MEM
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd740000, 
        virtual_start: 0xfd740000, 
        size: 0x80000, 
    },//Reserved
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd7c0000, 
        virtual_start: 0xfd7c0000, 
        size: 0x8000, 
    },//CRU_NS
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd7c8000, 
        virtual_start: 0xfd7c8000, 
        size: 0x8000, 
    },//PHP_PPLL_CRU
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd7d0000, 
        virtual_start: 0xfd7d0000, 
        size: 0x8000, 
    },//SEC_SCRU
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd7d8000, 
        virtual_start: 0xfd7d8000, 
        size: 0x8000, 
    },//BUS_SCRU
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd7e0000, 
        virtual_start: 0xfd7e0000, 
        size: 0x10000, 
    },//PMU1_SCRU
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd7f0000, 
        virtual_start: 0xfd7f0000, 
        size: 0x10000, 
    },//PMU1_CRU
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd800000, 
        virtual_start: 0xfd800000, 
        size: 0x4000, 
    },//DDRPHY0_CRU
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd804000, 
        virtual_start: 0xfd804000, 
        size: 0x4000, 
    },//DDRPHY1_CRU
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd808000, 
        virtual_start: 0xfd808000, 
        size: 0x4000, 
    },//DDRPHY2_CRU
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd80c000, 
        virtual_start: 0xfd80c000, 
        size: 0x4000, 
    },//DDRPHY3_CRU
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd810000, 
        virtual_start: 0xfd810000, 
        size: 0x2000, 
    },//BIGCORE0_CRU
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd812000, 
        virtual_start: 0xfd812000, 
        size: 0x2000, 
    },//BIGCORE1_CRU
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd814000, 
        virtual_start: 0xfd814000, 
        size: 0x4000, 
    },//Reserved
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd818000, 
        virtual_start: 0xfd818000, 
        size: 0x4000, 
    },//DSU_CRU
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd81c000, 
        virtual_start: 0xfd81c000, 
        size: 0x64000, 
    },//Reserved
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd880000, 
        virtual_start: 0xfd880000, 
        size: 0x10000, 
    },//I2C0
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd890000, 
        virtual_start: 0xfd890000, 
        size: 0x10000, 
    },//UART0
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd8a0000, 
        virtual_start: 0xfd8a0000, 
        size: 0x10000, 
    },//GPIO0
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd8b0000, 
        virtual_start: 0xfd8b0000, 
        size: 0x10000, 
    },//PWM0
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd8c0000, 
        virtual_start: 0xfd8c0000, 
        size: 0x8000, 
    },//PVTM_PMU
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd8c8000, 
        virtual_start: 0xfd8c8000, 
        size: 0x4000, 
    },//HPTIMER
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd8cc000, 
        virtual_start: 0xfd8cc000, 
        size: 0x4000, 
    },//Reserved
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd8d0000, 
        virtual_start: 0xfd8d0000, 
        size: 0x10000, 
    },//PMU
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd8e0000, 
        virtual_start: 0xfd8e0000, 
        size: 0x10000, 
    },//WDT_PMU
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd8f0000, 
        virtual_start: 0xfd8f0000, 
        size: 0x10000, 
    },//TIMER_PMU(2CH)
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd900000, 
        virtual_start: 0xfd900000, 
        size: 0xb0000, 
    },//Reserved
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd9b0000, 
        virtual_start: 0xfd9b0000, 
        size: 0x10000, 
    },//OSC_CHK
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd9c0000, 
        virtual_start: 0xfd9c0000, 
        size: 0x10000, 
    },//SCRAMBLE_KEY
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfd9d0000, 
        virtual_start: 0xfd9d0000, 
        size: 0x70000, 
    },//Reserved
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfda40000, 
        virtual_start: 0xfda40000, 
        size: 0x8000, 
    },//PVTM_CORE_B0
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfda48000, 
        virtual_start: 0xfda48000, 
        size: 0x8000, 
    },//Reserved
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfda50000, 
        virtual_start: 0xfda50000, 
        size: 0x8000, 
    },//PVTM_CORE_B1
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfda58000, 
        virtual_start: 0xfda58000, 
        size: 0x8000, 
    },//Reserved
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfda60000, 
        virtual_start: 0xfda60000, 
        size: 0x10000, 
    },//PVTM_CORE_L
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfda70000, 
        virtual_start: 0xfda70000, 
        size: 0x40000, 
    },//Reserved
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfdab0000, 
        virtual_start: 0xfdab0000, 
        size: 0x10000, 
    },//RKNN C0
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfdac0000, 
        virtual_start: 0xfdac0000, 
        size: 0x10000, 
    },//RKNN C1
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfdad0000, 
        virtual_start: 0xfdad0000, 
        size: 0x10000, 
    },//RKNN C2
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfdae0000, 
        virtual_start: 0xfdae0000, 
        size: 0x10000, 
    },//Reserved
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfdaf0000, 
        virtual_start: 0xfdaf0000, 
        size: 0x8000, 
    },//PVTM_NPU
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfdaf8000, 
        virtual_start: 0xfdaf8000, 
        size: 0x8000, 
    },//WDT_NPU
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfdb00000, 
        virtual_start: 0xfdb00000, 
        size: 0x8000, 
    },//TIMER_NPU(2CH)
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfdb08000, 
        virtual_start: 0xfdb08000, 
        size: 0x28000, 
    },//Reserved
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfdb30000, 
        virtual_start: 0xfdb30000, 
        size: 0x10000, 
    },//PVTM_GPU
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfdb40000, 
        virtual_start: 0xfdb40000, 
        size: 0x10000, 
    },//Reserved
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfdb50000, 
        virtual_start: 0xfdb50000, 
        size: 0x10000, 
    },//VDPU
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfdb60000, 
        virtual_start: 0xfdb60000, 
        size: 0x10000, 
    },//RGA3_0
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfdb70000, 
        virtual_start: 0xfdb70000, 
        size: 0x10000, 
    },//RGA3_1
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfdb80000, 
        virtual_start: 0xfdb80000, 
        size: 0x10000, 
    },//RGA2
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfdb90000, 
        virtual_start: 0xfdb90000, 
        size: 0x10000, 
    },//JPEG_DEC
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfdba0000, 
        virtual_start: 0xfdba0000, 
        size: 0x4000, 
    },//JPEG_ENC0
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfdba4000, 
        virtual_start: 0xfdba4000, 
        size: 0x4000, 
    },//JPEG_ENC1
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfdba8000, 
        virtual_start: 0xfdba8000, 
        size: 0x4000, 
    },//JPEG_ENC2
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfdbac000, 
        virtual_start: 0xfdbac000, 
        size: 0x4000, 
    },//JPEG_ENC3
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfdbb0000, 
        virtual_start: 0xfdbb0000, 
        size: 0x10000, 
    },//IEP
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfdbc0000, 
        virtual_start: 0xfdbc0000, 
        size: 0x10000, 
    },//Reserved
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfdbd0000, 
        virtual_start: 0xfdbd0000, 
        size: 0x10000, 
    },//RKVENC0
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfdbe0000, 
        virtual_start: 0xfdbe0000, 
        size: 0x10000, 
    },//RKVENC1
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfdbf0000, 
        virtual_start: 0xfdbf0000, 
        size: 0x40000, 
    },//Reserved
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfdc30000, 
        virtual_start: 0xfdc30000, 
        size: 0x8000, 
    },//RKVDEC_CCU
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfdc38000, 
        virtual_start: 0xfdc38000, 
        size: 0x8000, 
    },//RKVDEC0
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfdc40000, 
        virtual_start: 0xfdc40000, 
        size: 0x10000, 
    },//RKVDEC1
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfdc50000, 
        virtual_start: 0xfdc50000, 
        size: 0x20000, 
    },//Reserved
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfdc70000, 
        virtual_start: 0xfdc70000, 
        size: 0x40000, 
    },//AV1
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfdcb0000, 
        virtual_start: 0xfdcb0000, 
        size: 0x10000, 
    },//ISP0
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfdcc0000, 
        virtual_start: 0xfdcc0000, 
        size: 0x10000, 
    },//ISP1
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfdcd0000, 
        virtual_start: 0xfdcd0000, 
        size: 0x8000, 
    },//FISHEYE0
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfdcd8000, 
        virtual_start: 0xfdcd8000, 
        size: 0x8000, 
    },//FISHEYE1
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfdce0000, 
        virtual_start: 0xfdce0000, 
        size: 0x20000, 
    },//VICAP
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfdd00000, 
        virtual_start: 0xfdd00000, 
        size: 0x10000, 
    },//Reserved
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfdd10000, 
        virtual_start: 0xfdd10000, 
        size: 0x10000, 
    },//CSI HOST0
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfdd20000, 
        virtual_start: 0xfdd20000, 
        size: 0x10000, 
    },//CSI HOST1
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfdd30000, 
        virtual_start: 0xfdd30000, 
        size: 0x10000, 
    },//CSI HOST2
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfdd40000, 
        virtual_start: 0xfdd40000, 
        size: 0x10000, 
    },//CSI HOST3
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfdd50000, 
        virtual_start: 0xfdd50000, 
        size: 0x10000, 
    },//CSI HOST4
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfdd60000, 
        virtual_start: 0xfdd60000, 
        size: 0x10000, 
    },//CSI HOST5
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfdd70000, 
        virtual_start: 0xfdd70000, 
        size: 0x20000, 
    },//Reserved
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfdd90000, 
        virtual_start: 0xfdd90000, 
        size: 0x10000, 
    },//VOP
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfdda0000, 
        virtual_start: 0xfdda0000, 
        size: 0x10000, 
    },//HDCP0_MMU
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfddb0000, 
        virtual_start: 0xfddb0000, 
        size: 0x8000, 
    },//SPDIF_TX2
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfddb8000, 
        virtual_start: 0xfddb8000, 
        size: 0x8000, 
    },//SPDIF_TX5
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfddc0000, 
        virtual_start: 0xfddc0000, 
        size: 0x8000, 
    },//I2S4_8CH
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfddc8000, 
        virtual_start: 0xfddc8000, 
        size: 0x8000, 
    },//I2S8_8CH
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfddd0000, 
        virtual_start: 0xfddd0000, 
        size: 0x10000, 
    },//HDCP1_MMU
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfdde0000, 
        virtual_start: 0xfdde0000, 
        size: 0x8000, 
    },//SPDIF_TX3
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfdde8000, 
        virtual_start: 0xfdde8000, 
        size: 0x8000, 
    },//SPDIF_TX4
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfddf0000, 
        virtual_start: 0xfddf0000, 
        size: 0x4000, 
    },//I2S5_8CH
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfddf4000, 
        virtual_start: 0xfddf4000, 
        size: 0x4000, 
    },//I2S6_8CH
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfddf8000, 
        virtual_start: 0xfddf8000, 
        size: 0x4000, 
    },//I2S7_8CH
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfddfc000, 
        virtual_start: 0xfddfc000, 
        size: 0x4000, 
    },//I2S9_8CH
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfde00000, 
        virtual_start: 0xfde00000, 
        size: 0x4000, 
    },//I2S10_8CH
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfde04000, 
        virtual_start: 0xfde04000, 
        size: 0x4000, 
    },//Reserved
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfde08000, 
        virtual_start: 0xfde08000, 
        size: 0x8000, 
    },//SPDIF_RX0
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfde10000, 
        virtual_start: 0xfde10000, 
        size: 0x8000, 
    },//SPDIF_RX1
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfde18000, 
        virtual_start: 0xfde18000, 
        size: 0x8000, 
    },//SPDIF_RX2
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfde20000, 
        virtual_start: 0xfde20000, 
        size: 0x10000, 
    },//DSI HOST0
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfde30000, 
        virtual_start: 0xfde30000, 
        size: 0x10000, 
    },//DSI HOST1
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfde40000, 
        virtual_start: 0xfde40000, 
        size: 0x8000, 
    },//HDCP0
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfde48000, 
        virtual_start: 0xfde48000, 
        size: 0x8000, 
    },//HDCP0_TRNG
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfde50000, 
        virtual_start: 0xfde50000, 
        size: 0x10000, 
    },//DP0
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfde60000, 
        virtual_start: 0xfde60000, 
        size: 0x10000, 
    },//DP1
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfde70000, 
        virtual_start: 0xfde70000, 
        size: 0x8000, 
    },//HDCP1
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfde78000, 
        virtual_start: 0xfde78000, 
        size: 0x8000, 
    },//HDCP1_TRNG
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfde80000, 
        virtual_start: 0xfde80000, 
        size: 0x20000, 
    },//HDMI TX0
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfdea0000, 
        virtual_start: 0xfdea0000, 
        size: 0x20000, 
    },//HDMI TX1
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfdec0000, 
        virtual_start: 0xfdec0000, 
        size: 0x10000, 
    },//eDP0
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfded0000, 
        virtual_start: 0xfded0000, 
        size: 0x10000, 
    },//eDP1
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfdee0000, 
        virtual_start: 0xfdee0000, 
        size: 0x10000, 
    },//HDMI RX
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfdef0000, 
        virtual_start: 0xfdef0000, 
        size: 0x8000, 
    },//HDCP_KEY 0
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfdef8000, 
        virtual_start: 0xfdef8000, 
        size: 0x10000, 
    },//HDCP_KEY 1
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfdf08000, 
        virtual_start: 0xfdf08000, 
        size: 0x8000, 
    },//Reserved
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfdf10000, 
        virtual_start: 0xfdf10000, 
        size: 0x4000, 
    },//HDMIRX_S
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfdf14000, 
        virtual_start: 0xfdf14000, 
        size: 0x4000, 
    },//Reserved
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfdf18000, 
        virtual_start: 0xfdf18000, 
        size: 0x4000, 
    },//eDP0_S
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfdf1c000, 
        virtual_start: 0xfdf1c000, 
        size: 0x4000, 
    },//eDP1_S
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfdf20000, 
        virtual_start: 0xfdf20000, 
        size: 0x10000, 
    },//Reserved
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfdf30000, 
        virtual_start: 0xfdf30000, 
        size: 0xe0000, 
    },//INTERCONNECT
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe010000, 
        virtual_start: 0xfe010000, 
        size: 0x8000, 
    },//FIREWALL_DSUDDR
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe018000, 
        virtual_start: 0xfe018000, 
        size: 0x18000, 
    },//Reserved
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe030000, 
        virtual_start: 0xfe030000, 
        size: 0x8000, 
    },//FIREWALL_DDR
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe038000, 
        virtual_start: 0xfe038000, 
        size: 0x8000, 
    },//FIREWALL_SYSMEM
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe040000, 
        virtual_start: 0xfe040000, 
        size: 0x10000, 
    },//Reserved
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe050000, 
        virtual_start: 0xfe050000, 
        size: 0x10000, 
    },//DMA2DDR
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe060000, 
        virtual_start: 0xfe060000, 
        size: 0x4000, 
    },//DDR_MON0
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe064000, 
        virtual_start: 0xfe064000, 
        size: 0x4000, 
    },//DDR_MON1
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe068000, 
        virtual_start: 0xfe068000, 
        size: 0x4000, 
    },//DDR_MON2
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe06c000, 
        virtual_start: 0xfe06c000, 
        size: 0x4000, 
    },//DDR_MON3
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe070000, 
        virtual_start: 0xfe070000, 
        size: 0x50000, 
    },//Reserved
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe0c0000, 
        virtual_start: 0xfe0c0000, 
        size: 0x10000, 
    },//DDRPHY0
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe0d0000, 
        virtual_start: 0xfe0d0000, 
        size: 0x10000, 
    },//DDRPHY1
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe0e0000, 
        virtual_start: 0xfe0e0000, 
        size: 0x10000, 
    },//DDRPHY2
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe0f0000, 
        virtual_start: 0xfe0f0000, 
        size: 0x10000, 
    },//DDRPHY3
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe100000, 
        virtual_start: 0xfe100000, 
        size: 0x10000, 
    },//Reserved
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe110000, 
        virtual_start: 0xfe110000, 
        size: 0x8000, 
    },//WDT_DDR
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe118000, 
        virtual_start: 0xfe118000, 
        size: 0x8000, 
    },//TIMER_DDR(2CH)
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe120000, 
        virtual_start: 0xfe120000, 
        size: 0x8000, 
    },//SHARE_MEM_SLV
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe128000, 
        virtual_start: 0xfe128000, 
        size: 0x8000, 
    },//AHB2APB
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe130000, 
        virtual_start: 0xfe130000, 
        size: 0x20000, 
    },//Reserved
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe150000, 
        virtual_start: 0xfe150000, 
        size: 0x10000, 
    },//PCIe3_4L_APB
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe160000, 
        virtual_start: 0xfe160000, 
        size: 0x10000, 
    },//PCIe3_2L_APB
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe170000, 
        virtual_start: 0xfe170000, 
        size: 0x10000, 
    },//PCIe3_1L0_APB
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe180000, 
        virtual_start: 0xfe180000, 
        size: 0x10000, 
    },//PCIe3_1L1_APB
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe190000, 
        virtual_start: 0xfe190000, 
        size: 0x10000, 
    },//PCIe3_1L2_APB
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe1a0000, 
        virtual_start: 0xfe1a0000, 
        size: 0x10000, 
    },//Reserved
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe1b0000, 
        virtual_start: 0xfe1b0000, 
        size: 0x10000, 
    },//GMAC0
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe1c0000, 
        virtual_start: 0xfe1c0000, 
        size: 0x10000, 
    },//GMAC1
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe1d0000, 
        virtual_start: 0xfe1d0000, 
        size: 0x40000, 
    },//Reserved
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe210000, 
        virtual_start: 0xfe210000, 
        size: 0x10000, 
    },//SATA0
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe220000, 
        virtual_start: 0xfe220000, 
        size: 0x10000, 
    },//SATA1
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe230000, 
        virtual_start: 0xfe230000, 
        size: 0x10000, 
    },//SATA2
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe240000, 
        virtual_start: 0xfe240000, 
        size: 0x70000, 
    },//Reserved
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe2b0000, 
        virtual_start: 0xfe2b0000, 
        size: 0x10000, 
    },//FSPI
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe2c0000, 
        virtual_start: 0xfe2c0000, 
        size: 0x10000, 
    },//SDMMC
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe2d0000, 
        virtual_start: 0xfe2d0000, 
        size: 0x10000, 
    },//SDIO
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe2e0000, 
        virtual_start: 0xfe2e0000, 
        size: 0x10000, 
    },//EMMC
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe2f0000, 
        virtual_start: 0xfe2f0000, 
        size: 0x10000, 
    },//SDMMC_BUF
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe300000, 
        virtual_start: 0xfe300000, 
        size: 0x70000, 
    },//Reserved
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe370000, 
        virtual_start: 0xfe370000, 
        size: 0x8000, 
    },//CRYPTO_NS
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe378000, 
        virtual_start: 0xfe378000, 
        size: 0x8000, 
    },//TRNG_NS
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe380000, 
        virtual_start: 0xfe380000, 
        size: 0x10000, 
    },//KEYLADDER_S
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe390000, 
        virtual_start: 0xfe390000, 
        size: 0x8000, 
    },//CRYPTO_S(Slave)
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe398000, 
        virtual_start: 0xfe398000, 
        size: 0x8000, 
    },//TRNG_S
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe3a0000, 
        virtual_start: 0xfe3a0000, 
        size: 0x10000, 
    },//OTP_S
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe3b0000, 
        virtual_start: 0xfe3b0000, 
        size: 0x10000, 
    },//Reserved
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe3c0000, 
        virtual_start: 0xfe3c0000, 
        size: 0x10000, 
    },//DCF
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe3d0000, 
        virtual_start: 0xfe3d0000, 
        size: 0x10000, 
    },//TIMER_S_0(6CH)
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe3e0000, 
        virtual_start: 0xfe3e0000, 
        size: 0x10000, 
    },//WDT_S
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe3f0000, 
        virtual_start: 0xfe3f0000, 
        size: 0x10000, 
    },//SEC_TRNG_CHK
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe400000, 
        virtual_start: 0xfe400000, 
        size: 0x20000, 
    },//Reserved
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe420000, 
        virtual_start: 0xfe420000, 
        size: 0x10000, 
    },//CRYPTO_S(By Keylad)
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe430000, 
        virtual_start: 0xfe430000, 
        size: 0x40000, 
    },//Reserved
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe470000, 
        virtual_start: 0xfe470000, 
        size: 0x10000, 
    },//I2S0_8CH
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe480000, 
        virtual_start: 0xfe480000, 
        size: 0x10000, 
    },//I2S1_8CH
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe490000, 
        virtual_start: 0xfe490000, 
        size: 0x10000, 
    },//I2S2_2CH
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe4a0000, 
        virtual_start: 0xfe4a0000, 
        size: 0x10000, 
    },//I2S3_2CH
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe4b0000, 
        virtual_start: 0xfe4b0000, 
        size: 0x10000, 
    },//PDM0
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe4c0000, 
        virtual_start: 0xfe4c0000, 
        size: 0x10000, 
    },//PDM1
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe4d0000, 
        virtual_start: 0xfe4d0000, 
        size: 0x10000, 
    },//VAD
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe4e0000, 
        virtual_start: 0xfe4e0000, 
        size: 0x10000, 
    },//SPDIF_TX0
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe4f0000, 
        virtual_start: 0xfe4f0000, 
        size: 0x10000, 
    },//SPDIF_TX1
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe500000, 
        virtual_start: 0xfe500000, 
        size: 0x10000, 
    },//ACDCDIG_DSM
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe510000, 
        virtual_start: 0xfe510000, 
        size: 0x90000, 
    },//Reserved
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe5a0000, 
        virtual_start: 0xfe5a0000, 
        size: 0x10000, 
    },//SPINLOCK
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe5b0000, 
        virtual_start: 0xfe5b0000, 
        size: 0x50000, 
    },//Reserved
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe600000, 
        virtual_start: 0xfe600000, 
        size: 0x400000, 
    },//GIC600
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfea00000, 
        virtual_start: 0xfea00000, 
        size: 0x10000, 
    },//DMAC0_S
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfea10000, 
        virtual_start: 0xfea10000, 
        size: 0x10000, 
    },//DMAC0_NS
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfea20000, 
        virtual_start: 0xfea20000, 
        size: 0x10000, 
    },//DMAC1_S
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfea30000, 
        virtual_start: 0xfea30000, 
        size: 0x10000, 
    },//DMAC1_NS
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfea40000, 
        virtual_start: 0xfea40000, 
        size: 0x10000, 
    },//Reserved
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfea50000, 
        virtual_start: 0xfea50000, 
        size: 0x10000, 
    },//CAN0
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfea60000, 
        virtual_start: 0xfea60000, 
        size: 0x10000, 
    },//CAN1
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfea70000, 
        virtual_start: 0xfea70000, 
        size: 0x10000, 
    },//CAN2
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfea80000, 
        virtual_start: 0xfea80000, 
        size: 0x10000, 
    },//DECOM
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfea90000, 
        virtual_start: 0xfea90000, 
        size: 0x10000, 
    },//I2C1
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfeaa0000, 
        virtual_start: 0xfeaa0000, 
        size: 0x10000, 
    },//I2C2
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfeab0000, 
        virtual_start: 0xfeab0000, 
        size: 0x10000, 
    },//I2C3
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfeac0000, 
        virtual_start: 0xfeac0000, 
        size: 0x10000, 
    },//I2C4
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfead0000, 
        virtual_start: 0xfead0000, 
        size: 0x10000, 
    },//I2C5
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfeae0000, 
        virtual_start: 0xfeae0000, 
        size: 0x8000, 
    },//TIMER_NS_0(6CH)
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfeae8000, 
        virtual_start: 0xfeae8000, 
        size: 0x8000, 
    },//TIMER_NS_1(6CH)
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfeaf0000, 
        virtual_start: 0xfeaf0000, 
        size: 0x10000, 
    },//WDT_NS
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfeb00000, 
        virtual_start: 0xfeb00000, 
        size: 0x10000, 
    },//SPI0
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfeb10000, 
        virtual_start: 0xfeb10000, 
        size: 0x10000, 
    },//SPI1
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfeb20000, 
        virtual_start: 0xfeb20000, 
        size: 0x10000, 
    },//SPI2
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfeb30000, 
        virtual_start: 0xfeb30000, 
        size: 0x10000, 
    },//SPI3
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfeb40000, 
        virtual_start: 0xfeb40000, 
        size: 0x10000, 
    },//UART1
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfeb50000, 
        virtual_start: 0xfeb50000, 
        size: 0x10000, 
    },//UART2
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfeb60000, 
        virtual_start: 0xfeb60000, 
        size: 0x10000, 
    },//UART3
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfeb70000, 
        virtual_start: 0xfeb70000, 
        size: 0x10000, 
    },//UART4
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfeb80000, 
        virtual_start: 0xfeb80000, 
        size: 0x10000, 
    },//UART5
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfeb90000, 
        virtual_start: 0xfeb90000, 
        size: 0x10000, 
    },//UART6
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfeba0000, 
        virtual_start: 0xfeba0000, 
        size: 0x10000, 
    },//UART7
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfebb0000, 
        virtual_start: 0xfebb0000, 
        size: 0x10000, 
    },//UART8
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfebc0000, 
        virtual_start: 0xfebc0000, 
        size: 0x10000, 
    },//UART9
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfebd0000, 
        virtual_start: 0xfebd0000, 
        size: 0x10000, 
    },//PWM1
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfebe0000, 
        virtual_start: 0xfebe0000, 
        size: 0x10000, 
    },//PWM2
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfebf0000, 
        virtual_start: 0xfebf0000, 
        size: 0x10000, 
    },//PWM3
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfec00000, 
        virtual_start: 0xfec00000, 
        size: 0x10000, 
    },//TSADC
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfec10000, 
        virtual_start: 0xfec10000, 
        size: 0x10000, 
    },//SARADC
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfec20000, 
        virtual_start: 0xfec20000, 
        size: 0x10000, 
    },//GPIO1
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfec30000, 
        virtual_start: 0xfec30000, 
        size: 0x10000, 
    },//GPIO2
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfec40000, 
        virtual_start: 0xfec40000, 
        size: 0x10000, 
    },//GPIO3
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfec50000, 
        virtual_start: 0xfec50000, 
        size: 0x10000, 
    },//GPIO4
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfec60000, 
        virtual_start: 0xfec60000, 
        size: 0x10000, 
    },//MAILBOX0(MCU_PMU)
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfec70000, 
        virtual_start: 0xfec70000, 
        size: 0x10000, 
    },//MAILBOX1(MCU_DDR)
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfec80000, 
        virtual_start: 0xfec80000, 
        size: 0x10000, 
    },//I2C6
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfec90000, 
        virtual_start: 0xfec90000, 
        size: 0x10000, 
    },//I2C7
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfeca0000, 
        virtual_start: 0xfeca0000, 
        size: 0x10000, 
    },//I2C8
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfecb0000, 
        virtual_start: 0xfecb0000, 
        size: 0x10000, 
    },//SPI4
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfecc0000, 
        virtual_start: 0xfecc0000, 
        size: 0x10000, 
    },//OTP_NS
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfecd0000, 
        virtual_start: 0xfecd0000, 
        size: 0x10000, 
    },//Reserved
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfece0000, 
        virtual_start: 0xfece0000, 
        size: 0x10000, 
    },//MAILBOX2(MCU_NPU)
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfecf0000, 
        virtual_start: 0xfecf0000, 
        size: 0x8000, 
    },//INTMUX(MCU_PMU)
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfecf8000, 
        virtual_start: 0xfecf8000, 
        size: 0x8000, 
    },//INTMUX(MCU_DDR)
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfed00000, 
        virtual_start: 0xfed00000, 
        size: 0x10000, 
    },//DMAC2_S
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfed10000, 
        virtual_start: 0xfed10000, 
        size: 0x10000, 
    },//DMAC2_NS
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfed20000, 
        virtual_start: 0xfed20000, 
        size: 0x8000, 
    },//JTAG_LOCK
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfed28000, 
        virtual_start: 0xfed28000, 
        size: 0x8000, 
    },//Reserved
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfed30000, 
        virtual_start: 0xfed30000, 
        size: 0x10000, 
    },//TIMER_S_1(6CH)
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfed40000, 
        virtual_start: 0xfed40000, 
        size: 0x20000, 
    },//Reserved
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfed60000, 
        virtual_start: 0xfed60000, 
        size: 0x10000, 
    },//HDPTX Combo PHY0
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfed70000, 
        virtual_start: 0xfed70000, 
        size: 0x10000, 
    },//HDPTX Combo PHY1
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfed80000, 
        virtual_start: 0xfed80000, 
        size: 0x10000, 
    },//USBDP PHY0
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfed90000, 
        virtual_start: 0xfed90000, 
        size: 0x10000, 
    },//USBDP PHY1
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfeda0000, 
        virtual_start: 0xfeda0000, 
        size: 0x10000, 
    },//MIPI CD PHY0
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfedb0000, 
        virtual_start: 0xfedb0000, 
        size: 0x10000, 
    },//MIPI CD PHY1
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfedc0000, 
        virtual_start: 0xfedc0000, 
        size: 0x8000, 
    },//MIPI CSI DPHY0
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfedc8000, 
        virtual_start: 0xfedc8000, 
        size: 0x8000, 
    },//MIPI CSI DPHY1
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfedd0000, 
        virtual_start: 0xfedd0000, 
        size: 0x10000, 
    },//Reserved
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfede0000, 
        virtual_start: 0xfede0000, 
        size: 0x10000, 
    },//HDMI RX PHY
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfedf0000, 
        virtual_start: 0xfedf0000, 
        size: 0x10000, 
    },//Reserved
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfee00000, 
        virtual_start: 0xfee00000, 
        size: 0x10000, 
    },//Combo PIPE PHY0
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfee10000, 
        virtual_start: 0xfee10000, 
        size: 0x10000, 
    },//Combo PIPE PHY1
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfee20000, 
        virtual_start: 0xfee20000, 
        size: 0x10000, 
    },//Combo PIPE PHY2
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfee30000, 
        virtual_start: 0xfee30000, 
        size: 0x4e000, 
    },//Reserved
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfee80000, 
        virtual_start: 0xfee80000, 
        size: 0x80000, 
    },//PCIe3 PHY
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfef00000, 
        virtual_start: 0xfef00000, 
        size: 0x100000, 
    },//Reserved
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xff000000, 
        virtual_start: 0xff000000, 
        size: 0x100000, 
    },//SYSTEM_SRAM
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xff100000, 
        virtual_start: 0xff100000, 
        size: 0x40000, 
    },//PMU_MEM
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0x900000000, 
        virtual_start: 0x900000000, 
        size: 0x40000000, 
    },//PCIe3_4L_S
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0x940000000, 
        virtual_start: 0x940000000, 
        size: 0x40000000, 
    },//PCIe3_2L_S
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0x980000000, 
        virtual_start: 0x980000000, 
        size: 0x40000000, 
    },//PCIe3_1L0_S
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0x9c0000000, 
        virtual_start: 0x9c0000000, 
        size: 0x40000000, 
    },//PCIe3_1L1_S
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xa00000000, 
        virtual_start: 0xa00000000, 
        size: 0x40000000, 
    },//PCIe3_1L2_S
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xa40000000, 
        virtual_start: 0xa40000000, 
        size: 0x400000, 
    },//PCIe3_4L_DBI
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xa40400000, 
        virtual_start: 0xa40400000, 
        size: 0x400000, 
    },//PCIe3_2L_DBI
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xa40800000, 
        virtual_start: 0xa40800000, 
        size: 0x400000, 
    },//PCIe3_1L0_DBI
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xa40c00000, 
        virtual_start: 0xa40c00000, 
        size: 0x400000, 
    },//PCIe3_1L1_DBI
    HvConfigMemoryRegion { 
        mem_type: MEM_TYPE_IO,
        physical_start: 0xa41000000, 
        virtual_start: 0xa41000000, 
        size: 0x400000, 
    },//PCIe3_1L2_DBI

];

//pub const ROOT_ZONE_IRQS: [u32; 1] = [
 //   0x76];
pub const ROOT_ZONE_IRQS: [u32; 29] = [
    39, 41, 42, 43, 45, 46, 64, 120, 121, 235, 237, 247, 248, 250, 251, 252, 265, 266, 309, 312,
    313, 355, 360, 365, 423, 424, 425, 429, 455,
];

pub const ROOT_ARCH_ZONE_CONFIG: HvArchZoneConfig = HvArchZoneConfig {
    gicd_base: 0xfe600000,
    gicd_size: 0x10000,
    gicr_base: 0xfe680000,
    gicr_size: 0x100000,
    gicc_base: 0x8010000,
    gicc_size: 0x10000,
    gicc_offset: 0x0,
    gich_base: 0x8030000,
    gich_size: 0x10000,
    gicv_base: 0x8040000,
    gicv_size: 0x10000,
    gits_base: 0x8080000,
    gits_size: 0x20000,
};

pub const ROOT_PCI_CONFIG: HvPciConfig = HvPciConfig {
    ecam_base: 0x4010000000,
    ecam_size: 0x10000000,
    io_base: 0x3eff0000,
    io_size: 0x10000,
    pci_io_base: 0x0,
    mem32_base: 0x10000000,
    mem32_size: 0x2eff0000,
    pci_mem32_base: 0x10000000,
    mem64_base: 0x8000000000,
    mem64_size: 0x8000000000,
    pci_mem64_base: 0x8000000000,
};

pub const ROOT_ZONE_IVC_CONFIG: [HvIvcConfig; 0] = [];

pub const ROOT_PCI_DEVS: [u64; 2] = [0, 1 << 3];
