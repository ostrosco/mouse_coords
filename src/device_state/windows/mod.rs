extern crate winapi;

use keymap::Keycode;
use mouse_state::MouseState;
use windows::winapi::shared::windef::POINT;
use windows::winapi::um::winuser;
use windows::winapi::um::winuser::{GetAsyncKeyState, GetCursorPos};

pub struct DeviceState;

impl DeviceState {
    pub fn new() -> DeviceState {
        DeviceState {}
    }

    pub fn query_pointer(&self) -> MouseState {
        let point = &mut POINT { x: 0, y: 0 };
        let button1pressed;
        let button2pressed;
        let button3pressed;
        let button4pressed;
        let button5pressed;
        let coords;
        unsafe {
            coords = if GetCursorPos(point) != 0 {
                ((*point).x, (*point).y)
            } else {
                (0, 0)
            };
            button1pressed =
                GetAsyncKeyState(winuser::VK_LBUTTON) as u32 & 0x8000 != 0;
            button2pressed =
                GetAsyncKeyState(winuser::VK_RBUTTON) as u32 & 0x8000 != 0;
            button3pressed =
                GetAsyncKeyState(winuser::VK_MBUTTON) as u32 & 0x8000 != 0;
            button4pressed =
                GetAsyncKeyState(winuser::VK_XBUTTON1) as u32 & 0x8000 != 0;
            button5pressed =
                GetAsyncKeyState(winuser::VK_XBUTTON2) as u32 & 0x8000 != 0;
        }
        MouseState {
            coords: coords,
            button_pressed: vec![
                false,
                button1pressed,
                button2pressed,
                button3pressed,
                button4pressed,
                button5pressed,
            ],
        }
    }

    pub fn query_keymap(&self) -> Vec<Keycode> {
        let mut keycodes = vec![];
        let mut keymap = vec![];
        unsafe {
            for key in 0..256 {
                keymap.push(GetAsyncKeyState(key));
            }
        }
        for (ix, byte) in keymap.iter().enumerate() {
            if *byte as u32 & 0x8000 != 0 {
                match self.win_key_to_keycode(ix as i32) {
                    Some(k) => keycodes.push(k),
                    None => (),
                }
            }
        }
        keycodes
    }

    fn win_key_to_keycode(&self, win_key: i32) -> Option<Keycode> {
        let mut keycode = match win_key {
            winuser::VK_F1 => Some(Keycode::F1),
            winuser::VK_F2 => Some(Keycode::F2),
            winuser::VK_F3 => Some(Keycode::F3),
            winuser::VK_F4 => Some(Keycode::F4),
            winuser::VK_F5 => Some(Keycode::F5),
            winuser::VK_F6 => Some(Keycode::F6),
            winuser::VK_F7 => Some(Keycode::F7),
            winuser::VK_F8 => Some(Keycode::F8),
            winuser::VK_F9 => Some(Keycode::F9),
            winuser::VK_F10 => Some(Keycode::F10),
            winuser::VK_F11 => Some(Keycode::F11),
            winuser::VK_F12 => Some(Keycode::F12),
            winuser::VK_NUMPAD0 => Some(Keycode::Numpad0),
            winuser::VK_NUMPAD1 => Some(Keycode::Numpad1),
            winuser::VK_NUMPAD2 => Some(Keycode::Numpad2),
            winuser::VK_NUMPAD3 => Some(Keycode::Numpad3),
            winuser::VK_NUMPAD4 => Some(Keycode::Numpad4),
            winuser::VK_NUMPAD5 => Some(Keycode::Numpad5),
            winuser::VK_NUMPAD6 => Some(Keycode::Numpad6),
            winuser::VK_NUMPAD7 => Some(Keycode::Numpad7),
            winuser::VK_NUMPAD8 => Some(Keycode::Numpad8),
            winuser::VK_NUMPAD9 => Some(Keycode::Numpad9),
            winuser::VK_ADD => Some(Keycode::NumpadAdd),
            winuser::VK_SUBTRACT => Some(Keycode::NumpadSubtract),
            winuser::VK_DIVIDE => Some(Keycode::NumpadDivide),
            winuser::VK_MULTIPLY => Some(Keycode::NumpadMultiply),
            winuser::VK_SPACE => Some(Keycode::Space),
            winuser::VK_LCONTROL => Some(Keycode::LControl),
            winuser::VK_RCONTROL => Some(Keycode::RControl),
            winuser::VK_LSHIFT => Some(Keycode::LShift),
            winuser::VK_RSHIFT => Some(Keycode::RShift),
            winuser::VK_LMENU => Some(Keycode::LAlt),
            winuser::VK_RMENU => Some(Keycode::RAlt),
            winuser::VK_LWIN => Some(Keycode::Meta),
            winuser::VK_RWIN => Some(Keycode::Meta),
            winuser::VK_RETURN => Some(Keycode::Enter),
            winuser::VK_ESCAPE => Some(Keycode::Escape),
            winuser::VK_UP => Some(Keycode::Up),
            winuser::VK_DOWN => Some(Keycode::Down),
            winuser::VK_LEFT => Some(Keycode::Left),
            winuser::VK_RIGHT => Some(Keycode::Right),
            winuser::VK_BACK => Some(Keycode::Backspace),
            winuser::VK_CAPITAL => Some(Keycode::CapsLock),
            winuser::VK_TAB => Some(Keycode::Tab),
            winuser::VK_HOME => Some(Keycode::Home),
            winuser::VK_END => Some(Keycode::End),
            winuser::VK_PRIOR => Some(Keycode::PageUp),
            winuser::VK_NEXT => Some(Keycode::PageDown),
            winuser::VK_INSERT => Some(Keycode::Insert),
            winuser::VK_DELETE => Some(Keycode::Delete),
            winuser::VK_OEM_3 => Some(Keycode::Grave),
            winuser::VK_OEM_MINUS => Some(Keycode::Minus),
            winuser::VK_OEM_PLUS => Some(Keycode::Equal),
            winuser::VK_OEM_4 => Some(Keycode::LeftBracket),
            winuser::VK_OEM_6 => Some(Keycode::RightBracket),
            winuser::VK_OEM_5 => Some(Keycode::BackSlash),
            winuser::VK_OEM_1 => Some(Keycode::Semicolon),
            winuser::VK_OEM_7 => Some(Keycode::Apostrophe),
            winuser::VK_OEM_COMMA => Some(Keycode::Comma),
            winuser::VK_OEM_PERIOD => Some(Keycode::Dot),
            winuser::VK_OEM_2 => Some(Keycode::Slash),

            _ => None,
        };

        if keycode.is_none() {
            let win_key = win_key as u8;
            keycode = match win_key as char {
                '0' => Some(Keycode::Key0),
                '1' => Some(Keycode::Key1),
                '2' => Some(Keycode::Key2),
                '3' => Some(Keycode::Key3),
                '4' => Some(Keycode::Key4),
                '5' => Some(Keycode::Key5),
                '6' => Some(Keycode::Key6),
                '7' => Some(Keycode::Key7),
                '8' => Some(Keycode::Key8),
                '9' => Some(Keycode::Key9),
                'A' => Some(Keycode::A),
                'B' => Some(Keycode::B),
                'C' => Some(Keycode::C),
                'D' => Some(Keycode::D),
                'E' => Some(Keycode::E),
                'F' => Some(Keycode::F),
                'G' => Some(Keycode::G),
                'H' => Some(Keycode::H),
                'I' => Some(Keycode::I),
                'J' => Some(Keycode::J),
                'K' => Some(Keycode::K),
                'L' => Some(Keycode::L),
                'M' => Some(Keycode::M),
                'N' => Some(Keycode::N),
                'O' => Some(Keycode::O),
                'P' => Some(Keycode::P),
                'Q' => Some(Keycode::Q),
                'R' => Some(Keycode::R),
                'S' => Some(Keycode::S),
                'T' => Some(Keycode::T),
                'U' => Some(Keycode::U),
                'V' => Some(Keycode::V),
                'W' => Some(Keycode::W),
                'X' => Some(Keycode::X),
                'Y' => Some(Keycode::Y),
                'Z' => Some(Keycode::Z),
                _ => None,
            }
        }
        keycode
    }
}
