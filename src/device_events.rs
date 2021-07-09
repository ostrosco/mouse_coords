use ::{Keycode, DeviceState};
use DeviceQuery;
use std::thread::{spawn, JoinHandle};

pub struct EventLoop {
    thread: JoinHandle<()>
}

impl EventLoop {
    pub fn new() -> Self {
        let thread = spawn(|| {
            let device_state = DeviceState::new();
            let mut prev_keys = vec![];
            loop {

            } {
                let keys = device_state.get_keys();
                if keys != prev_keys {
                    println!("{:?}", keys);
                }
                prev_keys = keys;
            }
        });
        Self { thread }
    }

    fn on_key_down<Callback: FnMut(Keycode)>(&self, mut callback: Callback) -> CallbackGuard<Callback> {
        CallbackGuard { callback }
    }
}

lazy_static! {
    static ref EVENT_LOOP: EventLoop = EventLoop::new();
}

pub struct CallbackGuard<T> {
    callback: T
}

pub trait DeviceEvents: DeviceQuery {
    fn on_key_down<Callback: FnMut(Keycode)>(&self, mut callback: Callback) -> CallbackGuard<Callback> {
        EVENT_LOOP.on_key_down(callback)
    }
}

impl DeviceEvents for DeviceState {

}