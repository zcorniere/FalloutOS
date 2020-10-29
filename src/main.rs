#![feature(custom_test_frameworks)]
#![test_runner(fallout_testing_framework::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![no_std]
#![no_main]

use fallout_vga_buffer::println;

mod panic;

#[cfg(test)]
mod tests;

static HELLO: &str = "Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("{}", HELLO);
    fallout_interrupt::init();
    println!("End of initialization !");

    #[cfg(test)]
    test_main();

    loop {}
}
