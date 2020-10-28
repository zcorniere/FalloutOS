#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(fallout_testing_framework::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use fallout_vga_buffer::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    fallout_testing_framework::test_panic_handler(info);
}

#[test_case]
fn test_println() {
    println!("test_println output");
}
