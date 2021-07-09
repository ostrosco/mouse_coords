//! A simple library for querying mouse and keyboard device_state without requiring
//! an active window. Currently works in Windows, Linux, and macOS.
//!
//! ```no_run
//! use device_query::{DeviceQuery, DeviceState, MouseState, Keycode};
//!
//! let device_state = DeviceState::new();
//! let mouse: MouseState = device_state.get_mouse();
//! println!("Current Mouse Coordinates: {:?}", mouse.coords);
//! let keys: Vec<Keycode> = device_state.get_keys();
//! println!("Is A pressed? {}", keys.contains(&Keycode::A));
//! ```

#[macro_use]
extern crate lazy_static;

pub mod keymap;
pub mod mouse_state;
pub mod device_state;
pub mod device_query;
pub mod device_events;

pub use keymap::Keycode;
pub use mouse_state::MouseState;
pub use device_state::DeviceState;
pub use device_query::DeviceQuery;
pub use device_events::{DeviceEvents, CallbackGuard};