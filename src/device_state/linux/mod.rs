extern crate input_event_codes as kernel_key;
extern crate x11;

use keymap::Keycode;
use self::x11::xlib;
use mouse_state::MouseState;
use std::os::raw::c_char;
use std::ptr;
use std::slice;

pub struct DeviceState {
    display: *mut xlib::Display,
}

impl Default for DeviceState {
    fn default() -> Self {
        Self::new()
    }
}

impl DeviceState {
    pub fn new() -> DeviceState {
        unsafe {
            let display = xlib::XOpenDisplay(ptr::null());
            if display.as_ref().is_none() {
                panic!("Could not connect to a X display");
            }
            DeviceState { display }
        }
    }

    pub fn query_pointer(&self) -> MouseState {
        let root;
        let mut root_x = 0;
        let mut root_y = 0;
        let mut win_x = 0;
        let mut win_y = 0;
        let mut root_return = 0;
        let mut child_return = 0;
        let mut mask_return = 0;
        unsafe {
            root = xlib::XDefaultRootWindow(self.display);
            xlib::XQueryPointer(
                self.display,
                root,
                &mut root_return,
                &mut child_return,
                &mut root_x,
                &mut root_y,
                &mut win_x,
                &mut win_y,
                &mut mask_return,
            );
        }
        let button1pressed = mask_return & xlib::Button1Mask > 0;
        let button2pressed = mask_return & xlib::Button2Mask > 0;
        let button3pressed = mask_return & xlib::Button3Mask > 0;
        let button4pressed = mask_return & xlib::Button4Mask > 0;
        let button5pressed = mask_return & xlib::Button5Mask > 0;

        // Use 1-based indexing here so people can just device_state the button
        // number they're interested in directly.
        let button_pressed = vec![
            false,
            button1pressed,
            button2pressed,
            button3pressed,
            button4pressed,
            button5pressed,
        ];
        MouseState {
            coords: (win_x, win_y),
            button_pressed,
        }
    }

    pub fn query_keymap(&self) -> Vec<Keycode> {
        let mut keycodes = vec![];
        unsafe {
            let keymap: *mut c_char = [0; 32].as_mut_ptr();
            xlib::XQueryKeymap(self.display, keymap);
            for (ix, byte) in
                slice::from_raw_parts(keymap, 32).iter().enumerate()
            {
                for bit in 0_u8..8_u8 {
                    let bitmask = 1 << bit;
                    if byte & bitmask != 0 {
                        //x11 keycode uses kernel keycode with an offset of 8.
                        let x11_key = ix as u8 * 8 + bit;
                        let kernel_key = x11_key - 8;
                        if let Some(k) = self.kernel_key_to_keycode(kernel_key)
                        {
                            keycodes.push(k)
                        }
                    }
                }
            }
        }
        keycodes
    }

    fn kernel_key_to_keycode(&self, kernel_code: u8) -> Option<Keycode> {
        match kernel_code as u16 {
            kernel_key::KEY_0 => Some(Keycode::Key0),
            kernel_key::KEY_1 => Some(Keycode::Key1),
            kernel_key::KEY_2 => Some(Keycode::Key2),
            kernel_key::KEY_3 => Some(Keycode::Key3),
            kernel_key::KEY_4 => Some(Keycode::Key4),
            kernel_key::KEY_5 => Some(Keycode::Key5),
            kernel_key::KEY_6 => Some(Keycode::Key6),
            kernel_key::KEY_7 => Some(Keycode::Key7),
            kernel_key::KEY_8 => Some(Keycode::Key8),
            kernel_key::KEY_9 => Some(Keycode::Key9),
            kernel_key::KEY_A => Some(Keycode::A),
            kernel_key::KEY_B => Some(Keycode::B),
            kernel_key::KEY_C => Some(Keycode::C),
            kernel_key::KEY_D => Some(Keycode::D),
            kernel_key::KEY_E => Some(Keycode::E),
            kernel_key::KEY_F => Some(Keycode::F),
            kernel_key::KEY_G => Some(Keycode::G),
            kernel_key::KEY_H => Some(Keycode::H),
            kernel_key::KEY_I => Some(Keycode::I),
            kernel_key::KEY_J => Some(Keycode::J),
            kernel_key::KEY_K => Some(Keycode::K),
            kernel_key::KEY_L => Some(Keycode::L),
            kernel_key::KEY_M => Some(Keycode::M),
            kernel_key::KEY_N => Some(Keycode::N),
            kernel_key::KEY_O => Some(Keycode::O),
            kernel_key::KEY_P => Some(Keycode::P),
            kernel_key::KEY_Q => Some(Keycode::Q),
            kernel_key::KEY_R => Some(Keycode::R),
            kernel_key::KEY_S => Some(Keycode::S),
            kernel_key::KEY_T => Some(Keycode::T),
            kernel_key::KEY_U => Some(Keycode::U),
            kernel_key::KEY_V => Some(Keycode::V),
            kernel_key::KEY_W => Some(Keycode::W),
            kernel_key::KEY_X => Some(Keycode::X),
            kernel_key::KEY_Y => Some(Keycode::Y),
            kernel_key::KEY_Z => Some(Keycode::Z),
            kernel_key::KEY_F1 => Some(Keycode::F1),
            kernel_key::KEY_F2 => Some(Keycode::F2),
            kernel_key::KEY_F3 => Some(Keycode::F3),
            kernel_key::KEY_F4 => Some(Keycode::F4),
            kernel_key::KEY_F5 => Some(Keycode::F5),
            kernel_key::KEY_F6 => Some(Keycode::F6),
            kernel_key::KEY_F7 => Some(Keycode::F7),
            kernel_key::KEY_F8 => Some(Keycode::F8),
            kernel_key::KEY_F9 => Some(Keycode::F9),
            kernel_key::KEY_F10 => Some(Keycode::F10),
            kernel_key::KEY_F11 => Some(Keycode::F11),
            kernel_key::KEY_F12 => Some(Keycode::F12),
            kernel_key::KEY_KP0 => Some(Keycode::Numpad0),
            kernel_key::KEY_KP1 => Some(Keycode::Numpad1),
            kernel_key::KEY_KP2 => Some(Keycode::Numpad2),
            kernel_key::KEY_KP3 => Some(Keycode::Numpad3),
            kernel_key::KEY_KP4 => Some(Keycode::Numpad4),
            kernel_key::KEY_KP5 => Some(Keycode::Numpad5),
            kernel_key::KEY_KP6 => Some(Keycode::Numpad6),
            kernel_key::KEY_KP7 => Some(Keycode::Numpad7),
            kernel_key::KEY_KP8 => Some(Keycode::Numpad8),
            kernel_key::KEY_KP9 => Some(Keycode::Numpad9),
            kernel_key::KEY_KPENTER => Some(Keycode::Enter),
            kernel_key::KEY_KPMINUS => Some(Keycode::NumpadSubtract),
            kernel_key::KEY_KPPLUS => Some(Keycode::NumpadAdd),
            kernel_key::KEY_KPSLASH => Some(Keycode::NumpadDivide),
            kernel_key::KEY_KPASTERISK => Some(Keycode::NumpadMultiply),
            kernel_key::KEY_ESC => Some(Keycode::Escape),
            kernel_key::KEY_SPACE => Some(Keycode::Space),
            kernel_key::KEY_LEFTCTRL => Some(Keycode::LControl),
            kernel_key::KEY_RIGHTCTRL => Some(Keycode::RControl),
            kernel_key::KEY_LEFTSHIFT => Some(Keycode::LShift),
            kernel_key::KEY_RIGHTSHIFT => Some(Keycode::RShift),
            kernel_key::KEY_LEFTALT => Some(Keycode::LAlt),
            kernel_key::KEY_RIGHTALT => Some(Keycode::RAlt),
            kernel_key::KEY_LEFTMETA => Some(Keycode::Meta),
            kernel_key::KEY_RIGHTMETA => Some(Keycode::Meta),
            kernel_key::KEY_ENTER => Some(Keycode::Enter),
            kernel_key::KEY_UP => Some(Keycode::Up),
            kernel_key::KEY_DOWN => Some(Keycode::Down),
            kernel_key::KEY_LEFT => Some(Keycode::Left),
            kernel_key::KEY_RIGHT => Some(Keycode::Right),
            kernel_key::KEY_BACKSPACE => Some(Keycode::Backspace),
            kernel_key::KEY_CAPSLOCK => Some(Keycode::CapsLock),
            kernel_key::KEY_TAB => Some(Keycode::Tab),
            kernel_key::KEY_HOME => Some(Keycode::Home),
            kernel_key::KEY_END => Some(Keycode::End),
            kernel_key::KEY_PAGEUP => Some(Keycode::PageUp),
            kernel_key::KEY_PAGEDOWN => Some(Keycode::PageDown),
            kernel_key::KEY_INSERT => Some(Keycode::Insert),
            kernel_key::KEY_DELETE => Some(Keycode::Delete),
            kernel_key::KEY_GRAVE => Some(Keycode::Grave),
            kernel_key::KEY_MINUS => Some(Keycode::Minus),
            kernel_key::KEY_EQUAL => Some(Keycode::Equal),
            kernel_key::KEY_LEFTBRACE => Some(Keycode::LeftBracket),
            kernel_key::KEY_RIGHTBRACE => Some(Keycode::RightBracket),
            kernel_key::KEY_BACKSLASH => Some(Keycode::BackSlash),
            kernel_key::KEY_SEMICOLON => Some(Keycode::Semicolon),
            kernel_key::KEY_APOSTROPHE => Some(Keycode::Apostrophe),
            kernel_key::KEY_COMMA => Some(Keycode::Comma),
            kernel_key::KEY_DOT => Some(Keycode::Dot),
            kernel_key::KEY_SLASH => Some(Keycode::Slash),
            _ => None,
        }
    }
}
