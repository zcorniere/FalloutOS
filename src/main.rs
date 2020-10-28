#![no_std]
#![no_main]

use fallout_vga_buffer::WRITER;

use core::fmt::Write;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &str = "Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    writeln!(WRITER.lock(), "{}", HELLO).unwrap();
    loop {}
}

