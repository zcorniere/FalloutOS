use fallout_vga_buffer::println;

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
    use core::fmt::Write;
    use fallout_vga_buffer::{BUFFER_HEIGHT, WRITER};

    let s = "Some test string that fits on a single line";
    x86_64::instructions::interrupts::without_interrupts(|| {
        let mut writer = WRITER.lock();
        writeln!(writer, "\n{}", s).expect("writeln failed");
        for (i, c) in s.chars().enumerate() {
            let screen_char = writer.buffer.chars[BUFFER_HEIGHT - 2][i].read();
            assert_eq!(char::from(screen_char.ascii_char), c);
        }
    });
}

#[test_case]
fn test_breakpoint_exception() {
    x86_64::instructions::interrupts::int3();
}
