#![allow(dead_code)]
mod bindings;
pub mod board;
pub mod canvas_mutex_guard;
pub mod clock;
pub mod firmware;
pub mod rand;
pub mod shutdown;
pub mod sorted_linked_list;
pub mod types;
pub mod usb;

/// This is called from within an ISR.
#[no_mangle]
pub extern "C" fn rust_button_handler(_pressed: bool) {
  // KeepKeyBoard::set_led(LedAction::TglRedLed);

  // let mut canvas = KeepKeyBoard::display_canvas();
  // canvas.set_constant_power(pressed);
  panic!("foobar");
}
