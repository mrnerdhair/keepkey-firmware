#![no_std]

mod kklib;
// mod kkui;

use core::ops::DerefMut;
use embedded_graphics::{
  mono_font::{ascii::FONT_6X10, MonoTextStyle},
  prelude::*,
  text::Text,
};
use embedded_layout::{
  align::{Align, horizontal, vertical},
  prelude::*,
  layout::linear::LinearLayout,
};
use kklib::{board::KeepKeyBoard, types::LedAction};
use ufmt::*;
use usb_device::prelude::*;
use usbd_webusb::*;

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
  KeepKeyBoard::set_led(LedAction::SetRedLed);
  KeepKeyBoard::set_led(LedAction::ClrGreenLed);

  let mut canvas = KeepKeyBoard::display_canvas();
  canvas.clear(GrayColor::BLACK).unwrap();
  core::mem::drop(canvas);

  let usb = KeepKeyBoard::usb();
  let mut winusb = kklib::usb::WinUsb::new(&usb, 0, '!' as u8, true);
  let mut webusb = WebUsb::new(&usb, url_scheme::HTTPS, "app.shapeshift.com");
  let mut kk_interface_callback = |x: &[u8], write: &mut dyn FnMut(&[u8]) -> ()| {
    write(x);
  };
  let mut kk_interface =
    kklib::usb::KeepKeyInterface::new(&usb, "KeepKey Interface", &mut kk_interface_callback);
  let mut kk_debug_link_callback = |x: &[u8], write: &mut dyn FnMut(&[u8]) -> ()| {
    write(x);
  };
  let mut kk_debug_link = kklib::usb::KeepKeyInterface::new(
    &usb,
    "KeepKey Debug Link Interface",
    &mut kk_debug_link_callback,
  );
  let mut kk_u2f = kklib::usb::U2FInterface::new(&usb, "KeepKey U2F Interface");

  let mut serial_number_buf: [u8; 24] = [0; 24];
  hex::encode_to_slice(KeepKeyBoard::serial_number(), &mut serial_number_buf).unwrap();
  let serial_number = core::str::from_utf8_mut(&mut serial_number_buf).unwrap();
  serial_number.make_ascii_uppercase();

  let mut usb_dev = UsbDeviceBuilder::new(&usb, UsbVidPid(0x2B24, 0x0002))
    .product("KeepKey")
    .manufacturer("KeyHodlers, LLC")
    .serial_number(serial_number)
    .device_release(0x0100)
    .max_packet_size_0(64)
    .build();

  // let mut ui = kkui::KeepKeyUI::new();

  KeepKeyBoard::set_led(LedAction::ClrRedLed);
  KeepKeyBoard::set_led(LedAction::ClrGreenLed);

  loop {
    if usb_dev.poll(&mut [
      &mut winusb,
      &mut webusb,
      &mut kk_interface,
      &mut kk_debug_link,
      &mut kk_u2f,
    ]) {
      kk_interface.poll();
      kk_debug_link.poll();
      kk_u2f.poll();
    }

    let mut text: heapless::String<512> = heapless::String::new();
    uwrite!(&mut text, "Uptime: {} ms", KeepKeyBoard::clock_ms()).unwrap();

    let mut canvas = KeepKeyBoard::display_canvas();
    // ui.layout_standard_notification("Hello, World!", &text, kkui::NotificationType::Logo, canvas.deref_mut()).unwrap();

    LinearLayout::vertical(Chain::new(Text::new(
      &text,
      Point::zero(),
      MonoTextStyle::new(&FONT_6X10, GrayColor::BLACK),
    )))
    .with_alignment(horizontal::Center)
    .arrange()
    .align_to(&canvas.bounding_box(), horizontal::Center, vertical::Center)
    .draw(canvas.deref_mut())
    .unwrap();
  }
}
