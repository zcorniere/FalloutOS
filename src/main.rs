#![feature(custom_test_frameworks)]
#![test_runner(fallout_testing_framework::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![no_std]
#![no_main]

use fallout_vga_buffer::println;
use bootloader::BootInfo;
use x86_64::VirtAddr;

mod panic;

#[cfg(test)]
mod tests;

static HELLO: &str = "Hello World!";

bootloader::entry_point!(kernel_main);
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("{}", HELLO);
    fallout_interrupt::init();
    let mut frame_allocator = unsafe { fallout_memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };
    println!("End of initialization !");

    #[cfg(test)]
    test_main();

    println!("Entering hlt_loop...");
    fallout_interrupt::hlt_loop();
}
