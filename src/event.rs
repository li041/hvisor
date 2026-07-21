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
//
#![allow(unused)]
use crate::{
    arch::cpu::this_cpu_id,
    arch::ipi::{arch_check_events, arch_prepare_send_event, arch_send_event},
    consts::{
        IPI_EVENT_CLEAR_INJECT_IRQ, IPI_EVENT_DWC_MSI_INJECT, IPI_EVENT_SEND_IPI,
        IPI_EVENT_UPDATE_HART_LINE, IPI_EVENT_VCPU_SUSPEND, MAX_CPU_NUM,
    },
    cpu_data::{this_cpu_data, vcpu_suspend, CpuSet},
    device::irqchip::inject_irq,
    platform::IRQ_WAKEUP_VIRTIO_DEVICE,
};
#[cfg(virtio_pci)]
use crate::{
    pci::msix::activate_msix,
    platform::{IRQ_WAKEUP_VIRTIO_PCI_CONFIG, IRQ_WAKEUP_VIRTIO_PCI_DATA},
};
use alloc::{collections::VecDeque, vec::Vec};
use core::sync::atomic::{AtomicUsize, Ordering};
use spin::Mutex;

pub const IPI_EVENT_WAKEUP: usize = 0;
pub const IPI_EVENT_SHUTDOWN: usize = 1;
pub const IPI_EVENT_VIRTIO_INJECT_IRQ: usize = 2;
pub const IPI_EVENT_WAKEUP_VIRTIO_DEVICE: usize = 3;
pub const IPI_EVENT_VIRTIO_PCI_CONFIG: usize = 7;
pub const IPI_EVENT_VIRTIO_PCI_DATA: usize = 8;
pub const IPI_EVENT_VIRTIO_PCI_DONE: usize = 9;

#[percpu::def_percpu]
static PERCPU_EVENTS: Mutex<EventQueueState> = Mutex::new(EventQueueState::new());

struct EventQueueState {
    queue: VecDeque<usize>,
    doorbell_armed: bool,
}

impl EventQueueState {
    const fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            doorbell_armed: false,
        }
    }
}

static EVENT_TRACE_SEQ: AtomicUsize = AtomicUsize::new(0);
const EVENT_TRACE_LIMIT: usize = 64;

// The caller ensures the cpu_id is valid
#[inline(always)]
fn get_percpu_events(cpu: usize) -> &'static Mutex<EventQueueState> {
    unsafe { PERCPU_EVENTS.remote_ref_raw(cpu) }
}

fn add_event(cpu: usize, event_id: usize) -> Option<bool> {
    if cpu >= MAX_CPU_NUM {
        return None;
    }
    let mut e = get_percpu_events(cpu).lock();
    if event_id == IPI_EVENT_SHUTDOWN {
        // If the event is shutdown, we need to clear all previous events, because shutdown will make cpu idle and won't process any events.
        e.queue.clear();
    }
    e.queue.push_back(event_id);
    let need_kick = !e.doorbell_armed;
    e.doorbell_armed = true;
    let seq = EVENT_TRACE_SEQ.fetch_add(1, Ordering::Relaxed);
    if seq < EVENT_TRACE_LIMIT {
        info!(
            "[HVDBG:event-enqueue] seq={} src_cpu={} dst_cpu={} event={} armed={} queue_len={} kick={}",
            seq,
            this_cpu_data().id,
            cpu,
            event_id,
            e.doorbell_armed,
            e.queue.len(),
            need_kick
        );
    }
    Some(need_kick)
}

pub fn fetch_event(cpu: usize) -> Option<usize> {
    if cpu >= MAX_CPU_NUM {
        return None;
    }
    get_percpu_events(cpu).lock().queue.pop_front()
}

pub fn dump_events() {
    for cpu in 0..MAX_CPU_NUM {
        let events = get_percpu_events(cpu).lock();
        if !events.queue.is_empty() {
            debug!("cpu {} events: {:?}", cpu, events.queue);
        }
    }
}

pub fn dump_cpu_events(cpu: usize) -> Vec<usize> {
    if cpu >= MAX_CPU_NUM {
        return Vec::new();
    }
    get_percpu_events(cpu)
        .lock()
        .queue
        .iter()
        .cloned()
        .collect()
}

pub fn clear_events(cpu: usize) {
    if cpu >= MAX_CPU_NUM {
        return;
    }
    let mut events = get_percpu_events(cpu).lock();
    events.queue.clear();
    events.doorbell_armed = false;
}

fn handle_event(event: Option<usize>) -> bool {
    let cpu_data = this_cpu_data();
    match event {
        Some(IPI_EVENT_WAKEUP) => {
            cpu_data.arch_cpu.run();
            false
        }
        Some(IPI_EVENT_SHUTDOWN) => {
            cpu_data.arch_cpu.idle();
            false
        }
        Some(IPI_EVENT_VIRTIO_INJECT_IRQ) => {
            #[cfg(target_arch = "loongarch64")]
            crate::device::irqchip::ls7a2000::sync_guest_irqs();
            #[cfg(not(target_arch = "loongarch64"))]
            crate::device::virtio_trampoline::handle_virtio_irq();
            true
        }
        Some(IPI_EVENT_WAKEUP_VIRTIO_DEVICE) => {
            inject_irq(IRQ_WAKEUP_VIRTIO_DEVICE, false);
            true
        }
        Some(IPI_EVENT_DWC_MSI_INJECT) => {
            #[cfg(all(target_arch = "aarch64", irq_gicv3, dwc_pcie, dwc_msi))]
            {
                crate::pci::dwc_msi::handle_dwc_msi_inject_event();
            }
            true
        }
        #[cfg(virtio_pci)]
        Some(IPI_EVENT_VIRTIO_PCI_CONFIG) => {
            inject_irq(IRQ_WAKEUP_VIRTIO_PCI_CONFIG, false);
            true
        }
        #[cfg(virtio_pci)]
        Some(IPI_EVENT_VIRTIO_PCI_DATA) => {
            inject_irq(IRQ_WAKEUP_VIRTIO_PCI_DATA, false);
            true
        }
        #[cfg(virtio_pci)]
        Some(IPI_EVENT_VIRTIO_PCI_DONE) => {
            // Virtio PCI notice
            // unsafe {
            //     VIRTIO_MSIX_MANAGER.write().activate_all_pending_irq();
            // }
            activate_msix();
            true
        }
        Some(IPI_EVENT_CLEAR_INJECT_IRQ)
        | Some(IPI_EVENT_UPDATE_HART_LINE)
        | Some(IPI_EVENT_SEND_IPI) => {
            arch_check_events(event);
            true
        }
        Some(IPI_EVENT_VCPU_SUSPEND) => {
            vcpu_suspend();
            true
        }
        // #[cfg(target_arch = "loongarch64")]
        // Some(IPI_EVENT_CLEAR_INJECT_IRQ) => {
        //     use crate::device::irqchip;
        //     irqchip::ls7a2000::clear_hwi_injected_irq();
        //     true
        // }
        // #[cfg(all(target_arch = "riscv64", plic))]
        // Some(IPI_EVENT_UPDATE_HART_LINE) => {
        //     use crate::device::irqchip;
        //     info!("cpu {} update hart line", cpu_data.id);
        //     irqchip::plic::update_hart_line();
        //     true
        // }
        // #[cfg(target_arch = "riscv64")]
        // Some(IPI_EVENT_SEND_IPI) => {
        //     // This event is different from events above, it is used to inject software interrupt.
        //     // While events above will inject external interrupt.
        //     use crate::arch::ipi::arch_ipi_handler;
        //     arch_ipi_handler();
        //     true
        // }
        _ => false,
    }
}

pub fn check_events() -> bool {
    handle_event(fetch_event(this_cpu_data().id))
}

#[cfg(target_arch = "loongarch64")]
pub fn handle_next_loongarch_event(_ipi_int_id: usize) -> bool {
    let cpu = this_cpu_data().id;
    let (event, more_pending) = {
        let mut state = get_percpu_events(cpu).lock();
        let event = state.queue.pop_front();
        let more_pending = !state.queue.is_empty();
        state.doorbell_armed = more_pending;
        (event, more_pending)
    };

    if let Some(event_id) = event {
        let seq = EVENT_TRACE_SEQ.fetch_add(1, Ordering::Relaxed);
        if seq < EVENT_TRACE_LIMIT {
            info!(
                "[HVDBG:event-drain] seq={} cpu={} event={} more_pending={}",
                seq, cpu, event_id, more_pending
            );
        }
        handle_event(Some(event_id));
        true
    } else {
        false
    }
}

pub fn send_event(cpu_id: usize, ipi_int_id: usize, event_id: usize) {
    // Some architectures need preparation before queueing an event.
    arch_prepare_send_event(cpu_id, ipi_int_id, event_id);
    let need_kick = add_event(cpu_id, event_id).unwrap_or(false);
    #[cfg(target_arch = "loongarch64")]
    if need_kick {
        arch_send_event(cpu_id as _, ipi_int_id as _);
    }
    #[cfg(not(target_arch = "loongarch64"))]
    arch_send_event(cpu_id as _, ipi_int_id as _);
}

/// Send event to a cpu set (except self).
pub fn send_event_to_all(cpu_set: CpuSet, ipi_int_id: usize, event_id: usize) {
    let this_cpu_id = this_cpu_id();
    for target_cpu_id in cpu_set.iter() {
        if target_cpu_id == this_cpu_id {
            continue;
        }
        info!(
            "send_event_to_all: send event {} to cpu {}",
            event_id, target_cpu_id
        );
        send_event(target_cpu_id, ipi_int_id, event_id);
    }
}
