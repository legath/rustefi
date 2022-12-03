#![no_std]
#![feature(core_intrinsics)]
#![feature(lang_items)]
#![feature(naked_functions)]
#![feature(allocator_api)]
#![feature(alloc_error_handler)]
extern crate panic_impl;
extern crate tms570;
extern crate alloc;
extern crate linked_list_allocator;

use tms570::serial::{SerialLine, Parity, StopBits, DataBits, Lines, event};
use tms570::scilin::SciChipset;
use tms570::gio::{Gio, GioPorts, GioDirection, Pull};
use tms570::iomm::Iomm;
use tms570::pinmux::PinMux;
use linked_list_allocator::LockedHeap;
use alloc::vec::Vec;
use core::arch::asm;
use tms570::pwm::{Pwm, PwmId};
use tms570::rti::RtiController;

use tms570::syscounter::SysCounter;
use tms570::vim::{Vim, VimType};
//use tms570::system::Sys;

pub mod lang_items;
pub mod handlers;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

pub fn heap_init() {
    unsafe {
        let start = tms570::heap_start() as *mut u8;
        let size = tms570::heap_size() as usize;
        ALLOCATOR.lock().init(start, size);
    }
}

fn test_isr() {
    unsafe { asm!("nop"); }
}

fn main() {
    unsafe {
        // Muxing pins is inherently unsafe
        let pmux = Iomm::new();
        pmux.setup_pins(&[PinMux::SCIRX, PinMux::SCITX]);
    }

    heap_init();

    /*unsafe {
        let vim = Vim::new();
        vim.isr_set(10, test_isr);
        vim.set_type(10, VimType::SysInterrupt);
        vim.interrupt_enable(10,true);
    }*/
    let ioport = Gio::new();
    let mut uart: SciChipset = SerialLine::new(Lines::Lin, DataBits::Eight,
                                               StopBits::One,
                                               Parity::None);
    uart.rx_enable(true)
        .tx_enable(true)
        .set_baudrate(9600);
    //uart.interrupt(event::RX_INT);
    uart.open();
    uart.write(b"Test\n");

    ioport.direction(GioPorts::MibSpiPort3, 0, GioDirection::Input);
    ioport.direction(GioPorts::B, 1, GioDirection::Output);

    //let syscnt: RtiController = SysCounter::new(false);
    //syscnt.set_period(0,10000);
    //syscnt.start_counter(0);

    loop {
        let button = ioport.get(GioPorts::MibSpiPort3, 0);
        if !button {
            ioport.toogle(GioPorts::B, 1);
        }
    }
}

#[alloc_error_handler]
#[no_mangle]
#[allow(unused_unsafe)]
pub fn rust_oom(_: core::alloc::Layout) -> ! {
    unsafe { core::intrinsics::abort() }
}
