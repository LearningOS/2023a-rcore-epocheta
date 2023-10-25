//! The main module and entrypoint
//!
//! The operating system and app also starts in this module. Kernel code starts
//! executing from `entry.asm`, after which [`rust_main()`] is called to
//! initialize various pieces of functionality [`clear_bss()`]. (See its source code for
//! details.)
//!
//! We then call [`println!`] to display `Hello, world!`.

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]
#![no_main]
#![feature(panic_info_message)]

use core::arch::global_asm;
use log::*;

#[macro_use]
mod console;
mod lang_items;
mod logging;
mod sbi;

#[path = "boards/qemu.rs"]
mod board;

global_asm!(include_str!("entry.asm"));

/// clear BSS segment
pub fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}

/// the rust entry-point of os
#[no_mangle]
pub fn rust_main() -> ! {
    extern "C" {
        fn stext(); // begin addr of text segment
        fn etext(); // end addr of text segment
        fn srodata(); // start addr of Read-Only data segment
        fn erodata(); // end addr of Read-Only data ssegment
        fn sdata(); // start addr of data segment
        fn edata(); // end addr of data segment
        fn sbss(); // start addr of BSS segment
        fn ebss(); // end addr of BSS segment
        fn boot_stack_lower_bound(); // stack lower bound
        fn boot_stack_top(); // stack top
    }
    clear_bss();
    logging::init();
    println!("\x1b[31m[kernel] Hello, world!\x1b[0m");
    trace!(
        "\x1b[90m[kernel] .text [{:#x}, {:#x})\x1b[0m",
        stext as usize,
        etext as usize
    );
    debug!(
        "\x1b[32m[kernel] .rodata [{:#x}, {:#x})\x1b[0m",
        srodata as usize, erodata as usize
    );
    info!(
        "\x1b[34m[kernel] .data [{:#x}, {:#x})\x1b[0m",
        sdata as usize, edata as usize
    );
    warn!(
        "\x1b[93m[kernel] boot_stack top=bottom={:#x}, lower_bound={:#x}\x1b[0m",
        boot_stack_top as usize, boot_stack_lower_bound as usize
    );
    error!("\x1b[31m[kernel] .bss [{:#x}, {:#x})\x1b[0m", sbss as usize, ebss as usize);

    use crate::board::QEMUExit;
    crate::board::QEMU_EXIT_HANDLE.exit_success(); // CI autotest success
                                                   //crate::board::QEMU_EXIT_HANDLE.exit_failure(); // CI autoest failed
}
