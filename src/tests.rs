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
    use fallout_vga_buffer::{WRITER, BUFFER_HEIGHT};

    let s = "Some test string that fits on a single line";
    println!("{}", s);
    for (i, c) in s.chars().enumerate() {
        let screen_char = WRITER.lock().buffer.chars[BUFFER_HEIGHT - 2][i].read();
        assert_eq!(char::from(screen_char.ascii_char), c);
    }
}
