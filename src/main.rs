#![feature(custom_test_frameworks)]
#![test_runner(crate::tests::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![no_std]
#![no_main]

use fallout_vga_buffer::println;

#[cfg(test)]
mod tests;
#[cfg(not(test))]
mod panic;

static HELLO: &str = "Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("{}", HELLO);

    #[cfg(test)]
    test_main();
    loop {}
}

