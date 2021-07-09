use ::{MouseState, Keycode};
use DeviceState;

pub trait DeviceQuery {
    fn get_mouse(&self) -> MouseState;
    fn get_keys(&self) -> Vec<Keycode>;
}

impl DeviceQuery for DeviceState {
    /// Query for the current mouse position and mouse button device_state.
    fn get_mouse(&self) -> MouseState {
        self.query_pointer()
    }

    /// Query for all keys that are currently pressed down.
    fn get_keys(&self) -> Vec<Keycode> {
        self.query_keymap()
    }
}
