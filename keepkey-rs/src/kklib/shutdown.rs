#![allow(dead_code)]

use core::panic::PanicInfo;

pub use super::board::*;
pub use super::canvas_mutex_guard::*;
pub use super::firmware::*;
pub use super::rand::*;
pub use super::types::*;
use ufmt::*;
// use super::super::kkui;

use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::*;
use embedded_layout::{layout::linear::LinearLayout, prelude::*};
use embedded_text::{
  alignment::HorizontalAlignment,
  style::{HeightMode, TextBoxStyleBuilder},
  TextBox,
};

// const SHUTDOWN_FONT: &embedded_graphics::variable_font::VariableFont = &crate::kkui::fonts::TITLE_FONT;
const SHUTDOWN_FONT: &embedded_graphics::mono_font::MonoFont = &embedded_graphics::mono_font::ascii::FONT_6X10;

pub enum ShutdownMessage<'a> {
  String(&'a str),
  #[cfg(debug_assertions)]
  Panic(&'a PanicInfo<'a>),
  #[cfg(not(debug_assertions))]
  Panic,
  // #[cfg(not(debug_assertions))]
  // Panic(Option<&'a str>),
}

#[no_mangle]
pub extern "C" fn rust_shutdown_hook(error_type: ShutdownError) {
  if error_type == ShutdownError::None {
    KeepKeyBoard::set_led(LedAction::ClrRedLed);
    KeepKeyBoard::set_led(LedAction::ClrGreenLed);
    return;
  }
  layout_shutdown_message(ShutdownMessage::String(match error_type {
    ShutdownError::None => unreachable!(),
    ShutdownError::RustPanic => panic!(),
    ShutdownError::StackSmashingProtection => "Stack Smashing Detected",
    ShutdownError::ClockSecuritySystem => "Clock Glitch Detected",
    ShutdownError::MemoryFault => "Memory Protection Fault",
    ShutdownError::NonMaskableInterrupt => "Non-Maskable Interupt Detected",
    ShutdownError::ResetFailed => "Board Reset Failed",
    ShutdownError::FaultInjectionDefense => "Fault Injection Detected",
  }))
}

pub fn layout_shutdown_message(message: ShutdownMessage) {
  let mut text: heapless::String<512> = heapless::String::new();
  let text_ref: &str = (|| -> Option<()> {
    match message {
      ShutdownMessage::String(msg) => {
        uwrite!(&mut text, "{}", msg).ok()?;
      }
      #[cfg(debug_assertions)]
      ShutdownMessage::Panic(panic_info) => {
        core::fmt::write(&mut text, format_args!("{}", panic_info)).ok()?;
        // uwrite!(&mut text, "Panic").ok()?;
        // match  {
        //   None => uwrite!(&mut text, "Panic").ok()?,
        //   Some(message) => uwrite!(&mut text, "Panic: {}", message).ok()?,
        // }
        // if let Some(location) = panic_info.location() {
        //   uwrite!(&mut text, "\n{}:{}:{}", location.file(), location.line(), location.column()).ok()?;
        // }
      }
      #[cfg(not(debug_assertions))]
      ShutdownMessage::Panic => uwrite!(&mut text, "Panic").ok()?,
      // #[cfg(not(debug_assertions))]
      // ShutdownMessage::Panic(None) => uwrite!(&mut text, "Panic").ok()?,
      // #[cfg(not(debug_assertions))]
      // ShutdownMessage::Panic(Some(payload)) => uwrite!(&mut text, "Panic: {}", payload).ok()?,
    }
    Some(())
  })()
  .map_or("Error Layout Failed", |_| &text);

  let character_style = MonoTextStyle::new(SHUTDOWN_FONT, GrayColor::WHITE);
  let textbox_style = TextBoxStyleBuilder::new()
    .height_mode(HeightMode::FitToText)
    .alignment(HorizontalAlignment::Center)
    .paragraph_spacing(6)
    .build();

  KeepKeyBoard::do_without_interrupts(|| {
    let canvas = unsafe { KeepKeyBoard::display_canvas_leak() };
    canvas.clear(GrayColor::BLACK).unwrap();

    let bounds = Rectangle::new(
      Point::zero(),
      Size::new(canvas.bounding_box().size.width, 0),
    );
    let text_box = TextBox::with_textbox_style(text_ref, bounds, character_style, textbox_style);
    let _ = LinearLayout::vertical(Chain::new(text_box))
      .with_alignment(horizontal::Center)
      .arrange()
      .align_to(&canvas.bounding_box(), horizontal::Center, vertical::Center)
      .draw(canvas)
      .unwrap();

    // We have to do this manually because we stole the canvas from its mutex earlier
    KeepKeyBoard::display_refresh(canvas);
  });
}

#[cfg(all(debug_assertions, not(test)))]
#[panic_handler]
fn handle_panic(info: &PanicInfo) -> ! {
  KeepKeyBoard::do_without_interrupts(|| {
    layout_shutdown_message(ShutdownMessage::Panic(info));
    KeepKeyBoard::shutdown_with_error(ShutdownError::RustPanic);
  });
  unreachable!();
}

#[cfg(all(not(debug_assertions), not(test)))]
#[panic_handler]
fn handle_panic(_info: &PanicInfo) -> ! {
  KeepKeyBoard::do_without_interrupts(|| {
    // Unfortunately, touching the PanicInfo payload causes the compiler to build in all the location info we
    // don't want bloating a release version.
    // layout_shutdown_message(ShutdownMessage::Panic(info.payload().downcast_ref::<&str>().map(|x| *x)));
    layout_shutdown_message(ShutdownMessage::Panic);
    KeepKeyBoard::shutdown_with_error(ShutdownError::RustPanic);
  });
  unreachable!();
}
