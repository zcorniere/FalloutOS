#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
#![feature(const_mut_refs)]
#![test_runner(fallout_testing_framework::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![no_std]
#![no_main]

extern crate alloc;

use bootloader::BootInfo;
use x86_64::VirtAddr;

#[macro_use]
mod vga;
mod panic;
mod tasks;

mod executor;
mod interrupt;
mod memory;

use crate::memory::{allocator::LinkedListAllocator, locked::Locked};

use crate::{tasks::print_keypresses, vga::unwrap_with_msg};
use spin::Mutex;
use vga_buffer_rs::{vga_buffer::VgaBuffer};

#[cfg(test)]
mod tests;

use executor::{task::Task, Executor};

static HELLO: &str = "Hello World!";

lazy_static::lazy_static! {
    pub static ref WRITER: Mutex<VgaBuffer> = Mutex::new(VgaBuffer::new(false));
}

#[global_allocator]
static ALLOCATOR: Locked<LinkedListAllocator> = Locked::new(LinkedListAllocator::new());

bootloader::entry_point!(kernel_main);
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("{}", HELLO);
    init(boot_info);
    println!("End of initialization !");
    println!();

    #[cfg(test)]
    test_main();

    let mut executor = Executor::new();
    executor.spawn(Task::new(print_keypresses()));
    executor.run();
}

fn init(boot_info: &'static BootInfo) {
    println!();
    println!("Setting up CPU interrupts...");
    interrupt::init();
    println!("Done !");
    println!();

    print!("Initializing memory...");
    let physical_mem_off = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(physical_mem_off) };
    println!("[OK]");
    print!("Initializing memory allocator...");
    let mut frame_allocator =
        unsafe { memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };
    println!("[OK]");
    unwrap_with_msg(
        "Initializing the heap",
        memory::allocator::init_heap(&mut mapper, &mut frame_allocator),
    );
}
