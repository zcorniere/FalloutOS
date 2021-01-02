#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[OK]");
    }
}

use fallout_qemu::{exit_qemu, serial_print, serial_println, QemuExitCode};

pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn trivial_assertion() {
    println!("This is a simple output");
}

#[test_case]
fn test_println_many() {
    for _ in 0..200 {
        println!("test_println_many output line");
    }
}
#[test_case]
fn test_println_output() {
    use fallout_vga_buffer::{BUFFER_HEIGHT, WRITER};

    let s = "Some test string that fits on a single line";
    println!("{}", s);
    for (i, c) in s.chars().enumerate() {
        let screen_char = WRITER.lock().buffer.chars[BUFFER_HEIGHT - 2][i].read();
        assert_eq!(char::from(screen_char.ascii_char), c);
    }
}

#[cfg(test)]
use bootloader::BootInfo;

#[cfg(test)]
bootloader::entry_point!(kernel_test_main);
#[cfg(test)]
fn kernel_test_main(_boot_info: &'static BootInfo) -> ! {
    fallout_interrupt::init();
    test_main();
    fallout_interrupt::hlt_loop();
}

pub fn test_panic_handler(info: &core::panic::PanicInfo) -> ! {
    serial_println!("[KO]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failure);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    test_panic_handler(info);
}
