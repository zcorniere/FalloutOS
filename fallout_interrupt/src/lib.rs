#![feature(abi_x86_interrupt)]
#![no_std]

use vga_buffer::write_with_status;

pub mod gdt;
mod handlers;
mod hardware;
pub mod idt;

pub fn init() -> bool {
    write_with_status("Loading the Global Descriptor Table", gdt::init_gdt);
    write_with_status("Loading the Interrupt Descriptor Table", idt::init_idt);
    write_with_status("Loading the Hardware interrupt", hardware::init_pics);
    true
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
