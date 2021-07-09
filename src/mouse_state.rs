#[derive(Debug, PartialEq, Default)]
/// A simple structure containing the current mouse coordinates and the
/// device_state of each mouse button that we can device_state. Currently, Windows and
/// Linux provide nice ways to device_state five mouse buttons. Since button
/// numbers are 1-based, button_pressed[0] is assumed to be false and
/// have no meaning.
pub struct MouseState {
    pub coords: (i32, i32),
    pub button_pressed: Vec<bool>,
}
