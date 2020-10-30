#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(fallout_testing_framework::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;

entry_point!(main);

fn main(boot_info: &'static BootInfo) -> ! {
    use x86_64::VirtAddr;

    interrupt::init();
    let physical_mem_off = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(physical_mem_off) };
    let mut frame_allocator =
        unsafe { memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };
    memory::allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");
    test_main();
    loop {}
}

#[test_case]
fn simple_allocation() {
    use alloc::boxed::Box;

    let heap_value_1 = Box::new(123);
    let heap_value_2 = Box::new(8905);
    assert_eq!(*heap_value_1, 123);
    assert_eq!(*heap_value_2, 8905);
}

#[test_case]
fn large_vec() {
    use alloc::vec::Vec;

    let n = 1000;
    let mut vec = Vec::new();
    for i in 0..n {
        vec.push(i);
    }
    assert_eq!(vec.iter().sum::<u64>(), (n - 1) * n / 2);
}

#[test_case]
fn many_box() {
    use alloc::boxed::Box;

    for i in 0..memory::allocator::HEAP_SIZE {
        let x = Box::new(i);
        assert_eq!(*x, i);
    }
}

#[test_case]
fn many_box_long_lived() {
    use alloc::boxed::Box;

    let long_lived = Box::new(1);
    for i in 0..memory::allocator::HEAP_SIZE {
        let x = Box::new(i);
        assert_eq!(*x, i);
    }
    assert_eq!(*long_lived, 1);
}

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    fallout_testing_framework::test_panic_handler(info);
}
