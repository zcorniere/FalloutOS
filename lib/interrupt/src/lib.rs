#![feature(abi_x86_interrupt)]
#![no_std]

use fallout_vga_buffer::write_with_status;
use x86_64::structures::idt::InterruptDescriptorTable;

pub mod gdt;
mod handlers;
mod hardware;

lazy_static::lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(handlers::breakpoint_handler);
        idt[hardware::InterruptIndex::Timer.as_usize()]
            .set_handler_fn(handlers::timer_interrupt_handler);
        unsafe {
            idt.double_fault.set_handler_fn(handlers::double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt
    };
}

fn init_idt() -> bool {
    IDT.load();
    true
}

pub fn init() -> bool {
    write_with_status("Loading the Global Descriptor Table", gdt::init_gdt);
    write_with_status("Loading the Interrupt Descriptor Table", init_idt);
    write_with_status("Loading the Hardware interrupt", hardware::init_pics);
    true
}
