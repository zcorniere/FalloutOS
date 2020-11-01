use x86_64::structures::idt::InterruptDescriptorTable;

use crate::{gdt, handlers, hardware};

lazy_static::lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(handlers::breakpoint_handler);
        idt.page_fault.set_handler_fn(handlers::page_fault_handler);
        idt[hardware::InterruptIndex::Timer.as_usize()]
            .set_handler_fn(handlers::timer_interrupt_handler);
        idt[hardware::InterruptIndex::Keyboard.as_usize()]
            .set_handler_fn(handlers::keyboard_interrupt_handler);
        unsafe {
            idt.double_fault.set_handler_fn(handlers::double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt
    };
}

pub fn init_idt() -> bool {
    IDT.load();
    true
}
