#[cfg(not(test))]
use vga_buffer::println;

use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    interrupt::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    fallout_testing_framework::test_panic_handler(info);
}
