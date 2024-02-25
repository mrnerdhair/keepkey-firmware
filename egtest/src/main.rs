#![no_std]
#![forbid(unsafe_code)]
#![recursion_limit = "256"]
#![allow(unused_imports)]

#[macro_use]
extern crate state_machine_future;
extern crate futures;

mod r#async;
mod kkui;

use core::str::FromStr;
use embedded_graphics::{
    image::Image,
    mono_font::{ascii::FONT_6X13_BOLD, MonoFont, MonoTextStyleBuilder},
    pixelcolor::{BinaryColor, Gray8},
    prelude::*,
    primitives::{Circle, Line, PrimitiveStyle, Rectangle},
    text::Text,
    variable_font::VariableFont,
};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window,
};
use embedded_layout::{
    layout::linear::{spacing, LinearLayout},
    prelude::*,
};

fn main () -> Result<(), core::convert::Infallible> {
    spin_on::spin_on(async_main())
}

async fn async_main() -> Result<(), core::convert::Infallible> {
    // println!("request {:?}", kkui::images::CONFIRMING[0].bounding_box().size);
    // println!("confirm {:?}", kkui::images::CONFIRMING[63].bounding_box().size);
    // println!("unplug {:?}", kkui::images::UNPLUG.bounding_box().size);
    // println!("logo {:?}", kkui::images::KK_LOGO.bounding_box().size);

    let mut display = SimulatorDisplay::<Gray8>::new(Size::new(256, 64));
    let mut ui = kkui::KeepKeyUI::new();

    ui.layout_standard_notification(
        "Notification Title",//Which Is Quite Long the quick brown fox jumps over the lazy dog",
        "the quick brown fox jumps over the lazy dog \u{00}\u{01}\u{02}\u{03}",
        kkui::NotificationType::Confirmed,
        &mut display,
    )?;

    // ui.layout_simple_message("the quick brown fox jumps over", &mut display)?;
    ui.layout_progress("Uploading firmware", 800, &mut display)?;
    ui.layout_watermark("DEBUG_LINK", &mut display)?;

    // ui.layout_standard_notification(
    //     "Hello, World!",
    //     "The 'quick' brown fox; jumps: over the \"lazy-dog\". 0123456789 \u{00}\u{01}\u{02}\u{03}",
    //     kkui::NotificationType::Logo,
    //     &mut display
    // )?;

    // ui.layout_standard_notification(
    //     "Notification Title",
    //     "the quick brown fox jumps over the lazy",
    //     &mut display,
    // )?;

    /*LinearLayout::vertical(
        // Chain::new(
        // LinearLayout::horizontal(
        Chain::new(kkui::padding(0,0))
            .append(Image::new(&kkui::images::KK_LOGO, Point::zero()))
            .append(Image::new(&kkui::images::POWEREDBY_LOGO, Point::zero()))
            .append(Image::new(&kkui::images::SALT_LOGO, Point::zero()))
            .append(Image::new(&kkui::images::BLOCKPIT_LOGO, Point::zero()))
            .append(Image::new(
                &kkui::images::BLOCKPIT_SCREENSAVER,
                Point::zero(),
            ))
            .append(Image::new(&kkui::images::DASH_LOGO, Point::zero()))
            .append(Image::new(&kkui::images::DASH_SCREENSAVER, Point::zero()))
            .append(Image::new(&kkui::images::FOX_LOGO, Point::zero()))
            .append(Image::new(&kkui::images::FOX_SCREENSAVER, Point::zero()))
            .append(Image::new(&kkui::images::KASPERSKY_LOGO, Point::zero()))
            .append(Image::new(&kkui::images::CONFIRM_ICON, Point::zero()))
            .append(
                LinearLayout::horizontal(
                    Chain::new(kkui::padding(0, 0))
                        .append(Image::new(&kkui::images::CONFIRMING[0], Point::zero()))
                        .append(Image::new(&kkui::images::CONFIRMING[1], Point::zero()))
                        .append(Image::new(&kkui::images::CONFIRMING[2], Point::zero()))
                        .append(Image::new(&kkui::images::CONFIRMING[3], Point::zero()))
                        .append(Image::new(&kkui::images::CONFIRMING[4], Point::zero()))
                        .append(Image::new(&kkui::images::CONFIRMING[5], Point::zero()))
                        .append(Image::new(&kkui::images::CONFIRMING[6], Point::zero()))
                        .append(Image::new(&kkui::images::CONFIRMING[7], Point::zero()))
                        .append(Image::new(&kkui::images::CONFIRMING[8], Point::zero()))
                        .append(Image::new(&kkui::images::CONFIRMING[9], Point::zero()))
                        .append(Image::new(&kkui::images::CONFIRMING[10], Point::zero()))
                    )
                    .with_alignment(vertical::Top)
                    .arrange(),
                )
                .append(
                    LinearLayout::horizontal(
                        Chain::new(kkui::padding(0, 0))
                        .append(Image::new(&kkui::images::CONFIRMING[11], Point::zero()))
                        .append(Image::new(&kkui::images::CONFIRMING[12], Point::zero()))
                        .append(Image::new(&kkui::images::CONFIRMING[13], Point::zero()))
                        .append(Image::new(&kkui::images::CONFIRMING[14], Point::zero()))
                        .append(Image::new(&kkui::images::CONFIRMING[15], Point::zero()))
                        .append(Image::new(&kkui::images::CONFIRMING[16], Point::zero()))
                        .append(Image::new(&kkui::images::CONFIRMING[17], Point::zero()))
                        .append(Image::new(&kkui::images::CONFIRMING[18], Point::zero()))
                        .append(Image::new(&kkui::images::CONFIRMING[19], Point::zero()))
                        .append(Image::new(&kkui::images::CONFIRMING[20], Point::zero()))
                        .append(Image::new(&kkui::images::CONFIRMING[21], Point::zero()))
                    )
                    .with_alignment(vertical::Top)
                    .arrange(),
                )
                .append(
                    LinearLayout::horizontal(
                        Chain::new(kkui::padding(0, 0))
                        .append(Image::new(&kkui::images::CONFIRMING[22], Point::zero()))
                        .append(Image::new(&kkui::images::CONFIRMING[23], Point::zero()))
                        .append(Image::new(&kkui::images::CONFIRMING[24], Point::zero()))
                        .append(Image::new(&kkui::images::CONFIRMING[25], Point::zero()))
                        .append(Image::new(&kkui::images::CONFIRMING[26], Point::zero()))
                        .append(Image::new(&kkui::images::CONFIRMING[27], Point::zero()))
                        .append(Image::new(&kkui::images::CONFIRMING[28], Point::zero()))
                        .append(Image::new(&kkui::images::CONFIRMING[29], Point::zero()))
                        .append(Image::new(&kkui::images::CONFIRMING[30], Point::zero()))
                        .append(Image::new(&kkui::images::CONFIRMING[31], Point::zero()))
                        .append(Image::new(&kkui::images::CONFIRMING[32], Point::zero()))
                    )
                    .with_alignment(vertical::Top)
                    .arrange(),
                )
                .append(
                    LinearLayout::horizontal(
                        Chain::new(kkui::padding(0, 0))
                        .append(Image::new(&kkui::images::CONFIRMING[33], Point::zero()))
                        .append(Image::new(&kkui::images::CONFIRMING[34], Point::zero()))
                        .append(Image::new(&kkui::images::CONFIRMING[35], Point::zero()))
                        .append(Image::new(&kkui::images::CONFIRMING[36], Point::zero()))
                        .append(Image::new(&kkui::images::CONFIRMING[37], Point::zero()))
                        .append(Image::new(&kkui::images::CONFIRMING[38], Point::zero()))
                        .append(Image::new(&kkui::images::CONFIRMING[39], Point::zero()))
                        .append(Image::new(&kkui::images::CONFIRMING[40], Point::zero()))
                        .append(Image::new(&kkui::images::CONFIRMING[41], Point::zero()))
                        .append(Image::new(&kkui::images::CONFIRMING[42], Point::zero()))
                        .append(Image::new(&kkui::images::CONFIRMING[43], Point::zero()))
                    )
                    .with_alignment(vertical::Top)
                    .arrange(),
                )
                .append(
                    LinearLayout::horizontal(
                        Chain::new(kkui::padding(0, 0))
                        .append(Image::new(&kkui::images::CONFIRMING[44], Point::zero()))
                        .append(Image::new(&kkui::images::CONFIRMING[45], Point::zero()))
                        .append(Image::new(&kkui::images::CONFIRMING[46], Point::zero()))
                        .append(Image::new(&kkui::images::CONFIRMING[47], Point::zero()))
                        .append(Image::new(&kkui::images::CONFIRMING[48], Point::zero()))
                        .append(Image::new(&kkui::images::CONFIRMING[49], Point::zero()))
                        .append(Image::new(&kkui::images::CONFIRMING[50], Point::zero()))
                        .append(Image::new(&kkui::images::CONFIRMING[51], Point::zero()))
                        .append(Image::new(&kkui::images::CONFIRMING[52], Point::zero()))
                        .append(Image::new(&kkui::images::CONFIRMING[53], Point::zero()))
                        .append(Image::new(&kkui::images::CONFIRMING[54], Point::zero()))
                    )
                    .with_alignment(vertical::Top)
                    .arrange(),
                )
                .append(
                    LinearLayout::horizontal(
                        Chain::new(kkui::padding(0, 0))
                        .append(Image::new(&kkui::images::CONFIRMING[55], Point::zero()))
                        .append(Image::new(&kkui::images::CONFIRMING[56], Point::zero()))
                        .append(Image::new(&kkui::images::CONFIRMING[57], Point::zero()))
                        .append(Image::new(&kkui::images::CONFIRMING[58], Point::zero()))
                        .append(Image::new(&kkui::images::CONFIRMING[59], Point::zero()))
                        .append(Image::new(&kkui::images::CONFIRMING[60], Point::zero()))
                        .append(Image::new(&kkui::images::CONFIRMING[61], Point::zero()))
                        .append(Image::new(&kkui::images::CONFIRMING[62], Point::zero()))
                        .append(Image::new(&kkui::images::CONFIRMING[63], Point::zero()))
                )
                .with_alignment(vertical::Top)
                .arrange(),
            )
            .append(Image::new(&kkui::images::WARNING, Point::zero()))
            .append(Image::new(&kkui::images::UNPLUG, Point::zero()))
        //         )
        //         .with_alignment(vertical::Top)
        //         .arrange(),
        //     )
        //     .append(
        //         LinearLayout::horizontal(
        //             Chain::new(Image::new(&kkui::images::CONFIRM_ICON, Point::zero()))
        //                 .append(Image::new(&kkui::images::CONFIRMING_1, Point::zero()))
        //                 .append(Image::new(&kkui::images::CONFIRMING_8, Point::zero()))
        //                 .append(Image::new(&kkui::images::CONFIRMING_16, Point::zero()))
        //                 .append(Image::new(&kkui::images::CONFIRMING_24, Point::zero()))
        //                 .append(Image::new(&kkui::images::CONFIRMING_32, Point::zero()))
        //                 .append(Image::new(&kkui::images::CONFIRMING_48, Point::zero()))
        //                 .append(Image::new(&kkui::images::CONFIRMING_62, Point::zero()))
        //                 .append(Image::new(&kkui::images::CONFIRMING_63, Point::zero()))
        //                 .append(Image::new(&kkui::images::CONFIRMING_64, Point::zero()))
        //                 .append(Image::new(&kkui::images::WARNING_1, Point::zero()))
        //                 .append(Image::new(&kkui::images::UNPLUG, Point::zero())),
        //         )
        //         .with_alignment(vertical::Top)
        //         .arrange(),
        //     ),
    )
    .with_spacing(spacing::Tight)
    .arrange()
    .align_to(&display.bounding_box(), horizontal::Left, vertical::Top)
    .draw(&mut display)?;

    // let line_style = PrimitiveStyle::with_stroke(BinaryColor::On, 1);
    // let font = VariableFont {
    //     mono_font: FONT_6X13_BOLD,
    //     glyph_width_mapping: &|font: &MonoFont<'_>, c: char| match c {
    //         'l' => 1..5,
    //         '!' => 2..4,
    //         ' ' => 0..1,
    //         _ => 0..font.character_size.width,
    //     },
    // };
    // let text_style = MonoTextStyleBuilder::new()
    //     .font(&font::TITLE_FONT)
    //     .text_color(BinaryColor::On)
    //     .build();

    // Circle::new(Point::new(72, 8), 48)
    //     .into_styled(line_style)
    //     .draw(&mut display)?;

    // Line::new(Point::new(48, 16), Point::new(8, 16))
    //     .into_styled(line_style)
    //     .draw(&mut display)?;

    // Line::new(Point::new(48, 16), Point::new(64, 32))
    //     .into_styled(line_style)
    //     .draw(&mut display)?;

    // Rectangle::new(Point::new(79, 15), Size::new(34, 34))
    //     .into_styled(line_style)
    //     .draw(&mut display)?;

    // Text::new("the quick brown fox\njumps over the lazy\ndog \u{00}\u{01}\u{02}\u{03}", Point::new(1, 10), text_style).draw(&mut display)?;
    // Text::new(&"the quick brown fox\njumps over the lazy\ndog \u{00}\u{01}\u{02}\u{03}".to_ascii_uppercase(), Point::new(1, 40), text_style).draw(&mut display)?;

    // Image::new(&font3::PIN_FONT_0X31, Point::new(8 * 0, 0)).draw(&mut display)?;
    // Rectangle::new(Point::new(4, 0), Size::new(4, 12)).into_styled(PrimitiveStyle::with_fill(BinaryColor::On)).draw(&mut display)?;
    // Image::new(&font3::PIN_FONT_0X32, Point::new(8 * 1, 0)).draw(&mut display)?;
    // Image::new(&font3::PIN_FONT_0X33, Point::new(8 * 2, 0)).draw(&mut display)?;
    // Image::new(&font3::PIN_FONT_0X34, Point::new(8 * 3, 0)).draw(&mut display)?;
    // Image::new(&font3::PIN_FONT_0X35, Point::new(8 * 4, 0)).draw(&mut display)?;
    // Image::new(&font3::PIN_FONT_0X36, Point::new(8 * 5, 0)).draw(&mut display)?;
    // Image::new(&font3::PIN_FONT_0X37, Point::new(8 * 6, 0)).draw(&mut display)?;
    // Image::new(&font3::PIN_FONT_0X38, Point::new(8 * 7, 0)).draw(&mut display)?;
    // Image::new(&font3::PIN_FONT_0X39, Point::new(8 * 8, 0)).draw(&mut display)?;
    // Rectangle::new(Point::new(8 * 9, 0), Size::new(4, 12)).into_styled(PrimitiveStyle::with_fill(BinaryColor::On)).draw(&mut display)?;
*/
    let output_settings = OutputSettingsBuilder::new()
        // .theme(BinaryColorTheme::OledBlue)
        .build();
    Window::new("Hello World", &output_settings).show_static(&display);

    Ok(())
}
