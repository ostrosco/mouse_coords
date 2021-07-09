extern crate device_query;

use device_query::{DeviceQuery, DeviceState, MouseState};

fn main() {
    let device_state = DeviceState::new();
    let mut prev_mouse = MouseState::default();
    loop {
        let mouse = device_state.get_mouse();
        if mouse != prev_mouse {
            println!("{:?}", mouse);
        }
        prev_mouse = mouse;
    }
}
