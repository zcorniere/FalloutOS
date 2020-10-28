#![no_std]
#![no_main]

use fallout_vga_buffer::println;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &str = "Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("{}", HELLO);
    loop {}
}

