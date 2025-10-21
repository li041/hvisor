use crate::{
    arch::llc_coloring::{self, arch_llc_coloring_init, get_llc_way_size},
    config::OPT_LLC_COLORING,
    consts::PAGE_SIZE,
};
use alloc::vec::Vec;
use spin::Once;

// global config for llc coloring
pub static mut LLC_COLORING_ENABLED: bool = false;
pub static LLC_TOTAL_SIZE: Once<usize> = Once::new();
pub static LLC_NR_WAYS: Once<usize> = Once::new();
pub static DEFAULT_COLORS: Once<Vec<usize>> = Once::new();
pub static HVISOR_DEFAULT_NUM_COLORS: u32 = 4;
pub static HVISOR_COLOR: Once<Vec<usize>> = Once::new();
pub static MAX_NR_COLORS: Once<usize> = Once::new();

#[derive(PartialEq)]
enum LLCColoringMode {
    Unspecified,
    Disabled,
    Enabled,
}

fn opt_llc_coloring_mode() -> LLCColoringMode {
    match OPT_LLC_COLORING.load(core::sync::atomic::Ordering::Relaxed) {
        0 => LLCColoringMode::Disabled,
        1 => LLCColoringMode::Enabled,
        _ => LLCColoringMode::Unspecified,
    }
}

pub fn llc_coloring_init() {
    let opt = opt_llc_coloring_mode();
    let way_size;
    let mut llc_coloring_enabled = matches!(opt, LLCColoringMode::Enabled);
    if opt != LLCColoringMode::Disabled
        && LLC_TOTAL_SIZE.get().is_some()
        && LLC_NR_WAYS.get().is_some()
    {
        llc_coloring_enabled = true;
        way_size = LLC_TOTAL_SIZE.get().unwrap() / LLC_NR_WAYS.get().unwrap();
    } else if !llc_coloring_enabled {
        info!("LLC coloring is disabled by config.");
        return;
    } else {
        way_size = match get_llc_way_size() {
            Some(size) => size,
            None => {
                warn!("Failed to get LLC way size, disable LLC coloring.");
                return;
            }
        };
    }
    // check way_size is multiple of page size
    if (way_size & (PAGE_SIZE - 1)) != 0 {
        panic!("LLC way size {:#x} is not multiple of page size", way_size);
    }
    let max_nr_colors = way_size >> PAGE_SIZE.trailing_zeros();
    unsafe {
        LLC_COLORING_ENABLED = llc_coloring_enabled;
        MAX_NR_COLORS.call_once(|| max_nr_colors);
    }
    // check max_nr_colors is power of 2
    if (max_nr_colors & (max_nr_colors - 1)) != 0 {
        panic!(
            "LLC way size {:#x} does not have power of 2 number of pages",
            way_size
        );
    }
    DEFAULT_COLORS.call_once(|| {
        let mut v = Vec::new();
        for i in 0..max_nr_colors {
            v.push(i);
        }
        v
    });
    if HVISOR_COLOR.get().is_none() {
        let nums_colors = core::cmp::min(HVISOR_DEFAULT_NUM_COLORS as usize, max_nr_colors);
        info!(
            "LLC color config not found. Using first {} colors",
            nums_colors
        );
        HVISOR_COLOR.call_once(|| DEFAULT_COLORS.get().unwrap()[..nums_colors].to_vec());
    } else {
        // Todo: HVISOR_COLOR is set by config
        let nums_colors = HVISOR_COLOR.get().unwrap().len();
        if nums_colors > max_nr_colors {
            panic!(
                "Requested {} LLC colors, but only {} available",
                nums_colors, max_nr_colors
            );
        }
        for &c in HVISOR_COLOR.get().unwrap().iter() {
            if c >= max_nr_colors {
                panic!(
                    "Requested LLC color {} exceeds max color {}",
                    c, max_nr_colors
                );
            }
        }
        info!("Using {} LLC colors", nums_colors);
        for &c in HVISOR_COLOR.get().unwrap().iter() {
            info!("  color {}", c);
        }
    }
    arch_llc_coloring_init();
}
