extern crate device_query;

use device_query::{DeviceQuery, DeviceEvents, DeviceState};

fn main() {
    let device_state = DeviceState::new();
    device_state.on_key_down(|key| {
        println!("{:#?}", key);
    });

    loop {}
}