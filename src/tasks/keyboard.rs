use futures_util::stream::StreamExt;
use pc_keyboard::{layouts, DecodedKey, HandleControl, KeyCode, Keyboard, ScancodeSet1};
use toogle::Toggle;
use vga_buffer_rs::BasicBufferManipulation;

use crate::{executor::keyboard::ScancodeStream, vga::WRITER};

pub async fn print_keypresses() {
    let mut ctrl_pressed = false;
    let mut scancodes = ScancodeStream::new();
    let mut keyboard = Keyboard::new(layouts::Azerty, ScancodeSet1, HandleControl::Ignore);

    while let Some(scancode) = scancodes.next().await {
        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
            match key_event.code {
                KeyCode::ControlLeft | KeyCode::ControlRight => ctrl_pressed.toggle(),
                _ => {
                    if let Some(key) = keyboard.process_keyevent(key_event) {
                        match key {
                            DecodedKey::RawKey(KeyCode::ControlLeft) => ctrl_pressed.toggle(),
                            DecodedKey::Unicode('c') if ctrl_pressed => WRITER.lock().clear(),
                            DecodedKey::Unicode(character) => print!("{}", character),
                            DecodedKey::RawKey(key) => print!("{:?}", key),
                        }
                    }
                }
            }
        }
    }
}
