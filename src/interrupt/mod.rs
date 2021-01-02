use crate::vga::write_result_bool;

pub mod gdt;
mod handlers;
mod hardware;
pub mod idt;

pub fn init() -> bool {
    write_result_bool("Loading the Global Descriptor Table", gdt::init_gdt);
    write_result_bool("Loading the Interrupt Descriptor Table", idt::init_idt);
    write_result_bool("Loading the Hardware interrupt", hardware::init_pics);
    true
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
