#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &str = "Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut  writer = vga_buffer::Writer::default();
    writer.write_string(HELLO);
    loop {}
}

