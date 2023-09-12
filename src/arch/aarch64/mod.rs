mod contextFrame;
mod cpu;
// mod config;
mod exception;
mod emu;
mod ipi;
mod interrupt;
mod hvc;
mod sync;
mod utils;
mod vcpu;
// mod vcpu_array;
mod vgic;
mod vm;
mod vmConfig;
mod vmPages;
mod manageVm;
mod gic;
mod timer;
mod ept;

pub use cpu::{current_cpu, CPU_INTERFACE_LIST, active_vm, active_vcpu_id, active_vm_id};
pub use ipi::ipi_irq_handler;
pub use vgic::maintenance_irq_handler;
pub use timer::timer_irq_handler;
pub use interrupt::{interrupt_handler, interrupt_init};
pub use gic::{GICC, GICD, GICH, interrupt_arch_enable};

// pub use config::*;

pub use page_table::PageSize;

type ContextFrame = crate::arch::contextFrame::Aarch64ContextFrame;

// Move to ARM register from system coprocessor register.
// MRS Xd, sysreg "Xd = sysreg"
#[macro_export]
macro_rules! mrs {
    ($val: expr, $reg: expr, $asm_width:tt) => {
        unsafe {
            core::arch::asm!(concat!("mrs {0:", $asm_width, "}, ", stringify!($reg)), out(reg) $val, options(nomem, nostack));
        }
    };
    ($val: expr, $reg: expr) => {
        unsafe {
            core::arch::asm!(concat!("mrs {0}, ", stringify!($reg)), out(reg) $val, options(nomem, nostack));
        }
    };
}

// Move to system coprocessor register from ARM register.
// MSR sysreg, Xn "sysreg = Xn"
#[macro_export]
macro_rules! msr {
    ($reg: expr, $val: expr, $asm_width:tt) => {
        unsafe {
            core::arch::asm!(concat!("msr ", stringify!($reg), ", {0:", $asm_width, "}"), in(reg) $val, options(nomem, nostack));
        }
    };
    ($reg: expr, $val: expr) => {
        unsafe {
            core::arch::asm!(concat!("msr ", stringify!($reg), ", {0}"), in(reg) $val, options(nomem, nostack));
        }
    };
}

use core::arch::global_asm;
global_asm!(include_str!("./memset.S"));
global_asm!(include_str!("./memcpy.S"));
extern "C" {
    pub fn memset(s: *mut u8, c: i32, n: usize) -> *mut u8;
    pub fn memcpy(s1: *const u8, s2: *const u8, n: usize) -> *mut u8;
}

pub fn memset_safe(s: *mut u8, c: i32, n: usize) -> *mut u8 {
    if (s as usize) < 0x1000 {
        panic!("illegal addr for memset s {:x}", s as usize);
    }
    unsafe { memset(s, c, n) }
}

pub fn memcpy_safe(s1: *const u8, s2: *const u8, n: usize) -> *mut u8 {
    if (s1 as usize) < 0x1000 || (s2 as usize) < 0x1000 {
        panic!("illegal addr for memcpy s1 {:x} s2 {:x}", s1 as usize, s2 as usize);
    }
    unsafe { memcpy(s1, s2, n) }
}


