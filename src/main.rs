#![no_std]
#![feature(core_intrinsics)]
#![feature(lang_items)]
#![feature(naked_functions)]
#![feature(allocator_api)]

extern crate panic_impl;
extern crate tms570;
extern crate alloc;
extern crate linked_list_allocator;

use tms570::serial::{SerialLine, Parity, StopBits, DataBits, Lines};
use tms570::scilin::SciChipset;
use tms570::gio::{Gio, GioPorts, GioDirection, Pull};
use tms570::iomm::Iomm;
use tms570::pinmux::PinMux;
use linked_list_allocator::LockedHeap;
use alloc::vec::Vec;

pub mod lang_items;
pub mod handlers;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

pub fn heap_init() {
    unsafe {
        let start = tms570::heap_start() as usize;
        let size = tms570::heap_size() as usize;
        ALLOCATOR.lock().init(start, size);
    }
}

fn main() {
    unsafe {
        // Muxing pins is inherently unsafe
        let pmux = Iomm::new();
        pmux.setup_pins(&[PinMux::SCIRX, PinMux::SCITX]);
    }

    heap_init();

    let ioport = Gio::new();
    let mut uart: SciChipset = SerialLine::new(Lines::Sci, DataBits::Eight,
                                               StopBits::One,
                                               Parity::None);
    uart.rx_enable(true)
        .tx_enable(true)
        .set_baudrate(115_200);
    uart.open();
    uart.write(b"Rust fw started\r\n");
    ioport.direction(GioPorts::MibSpiPort3, 0, GioDirection::Input);
    ioport.direction(GioPorts::B, 1, GioDirection::Output);


    loop {
        let button = ioport.get(GioPorts::MibSpiPort3, 0);
        if !button {
            ioport.toogle(GioPorts::B, 1);

        }
    }
}

#[lang = "oom"]
#[no_mangle]
pub fn rust_oom(_: core::alloc::Layout) -> ! {
    unsafe { core::intrinsics::abort() }
}
