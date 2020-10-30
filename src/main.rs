#![feature(custom_test_frameworks)]
#![test_runner(fallout_testing_framework::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![no_std]
#![no_main]

use bootloader::BootInfo;
use vga_buffer::println;
use x86_64::VirtAddr;

mod panic;

#[cfg(test)]
mod tests;

static HELLO: &str = "Hello World!";

bootloader::entry_point!(kernel_main);
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("{}", HELLO);
    interrupt::init();
    let physical_mem_off = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(physical_mem_off) };
    let mut frame_allocator =
        unsafe { memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };
    memory::allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");
    println!("End of initialization !");

    #[cfg(test)]
    test_main();

    println!("Entering hlt_loop...");
    interrupt::hlt_loop();
}
