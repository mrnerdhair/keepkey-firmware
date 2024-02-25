#![allow(dead_code)]

use core::str::FromStr;
use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::{AnchorPoint, Dimensions, Point, Size},
    image::{Image, ImageRaw},
    mono_font::{Font, MonoTextStyle, MonoTextStyleBuilder},
    pixelcolor::{BinaryColor, Gray8, GrayColor, Rgb888},
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle, Styled},
    text::renderer::TextRenderer,
    text::LineHeight,
    text::Text,
    variable_font::VariableFont,
    Drawable,
};
use embedded_layout::{
    layout::linear::{spacing, LinearLayout},
    prelude::*,
};
use embedded_text::{
    alignment::{HorizontalAlignment, VerticalAlignment},
    style::{HeightMode, TextBoxStyle, TextBoxStyleBuilder},
    TextBox,
};

pub mod r#box;
pub mod box_offset;
pub mod fonts;
pub mod images;
pub mod layout;
pub mod resizable;
pub mod state_machines;

use box_offset::{BoxOffset, RectangleExt};
use layout::Layout;

pub(crate) fn padding<C: GrayColor>(
    width: u32,
    height: u32,
) -> Styled<Rectangle, PrimitiveStyle<C>> {
    Rectangle::new(Point::zero(), Size::new(width, height))
        .into_styled(PrimitiveStyle::with_stroke(C::BLACK, 0))
}

fn image_anchored_at_point<'a, 'b, C, D>(
    image: &'a ImageRaw<'a, C>,
    target: &'b D,
    anchor_point: AnchorPoint,
    offset: Point,
) -> Image<'a, ImageRaw<'a, C>>
where
    ImageRaw<'a, C>: ImageDrawable,
    C: PixelColor + From<<C as PixelColor>::Raw>,
    D: DrawTarget<Color = C>,
{
    let image_width: i32 = image.size().width.try_into().unwrap();
    let image_height: i32 = image.size().height.try_into().unwrap();

    let anchor_point_correction = Point::new(
        match anchor_point {
            AnchorPoint::TopLeft | AnchorPoint::CenterLeft | AnchorPoint::BottomLeft => 0,
            AnchorPoint::TopCenter | AnchorPoint::Center | AnchorPoint::BottomCenter => 1,
            AnchorPoint::TopRight | AnchorPoint::CenterRight | AnchorPoint::BottomRight => 1,
        },
        match anchor_point {
            AnchorPoint::TopLeft | AnchorPoint::TopCenter | AnchorPoint::TopRight => 0,
            AnchorPoint::CenterLeft | AnchorPoint::Center | AnchorPoint::CenterRight => 1,
            AnchorPoint::BottomLeft | AnchorPoint::BottomCenter | AnchorPoint::BottomRight => 1,
        },
    );
    let corrected_anchor_point =
        target.bounding_box().anchor_point(anchor_point) + anchor_point_correction;

    let image_origin_offset = Point::new(
        match anchor_point {
            AnchorPoint::TopLeft | AnchorPoint::CenterLeft | AnchorPoint::BottomLeft => 0,
            AnchorPoint::TopCenter | AnchorPoint::Center | AnchorPoint::BottomCenter => {
                -(image_width / 2)
            }
            AnchorPoint::TopRight | AnchorPoint::CenterRight | AnchorPoint::BottomRight => {
                -image_width
            }
        },
        match anchor_point {
            AnchorPoint::TopLeft | AnchorPoint::TopCenter | AnchorPoint::TopRight => 0,
            AnchorPoint::CenterLeft | AnchorPoint::Center | AnchorPoint::CenterRight => {
                -(image_height / 2)
            }
            AnchorPoint::BottomLeft | AnchorPoint::BottomCenter | AnchorPoint::BottomRight => {
                -image_height
            }
        },
    );
    let image_origin = corrected_anchor_point + image_origin_offset + offset;
    // println!("foo {:?}", image_origin);
    Image::new(image, image_origin)
}

fn make_upper(text: &str) -> heapless::String<256> {
    let mut upper: heapless::String<256> =
        heapless::String::from_str(&text[..core::cmp::min(text.len(), 256)]).unwrap();
    upper.make_ascii_uppercase();
    upper
}

fn em<'a, T: Font<'a>>(font: &T, basis_points: u32) -> u32 {
    font.character_height() * basis_points / 10000
}

pub enum NotificationType {
    Logo,
    Info,
    Request,
    Confirmed,
    Unplug,
}

#[non_exhaustive]
pub struct KeepKeyUI<'a> {
    pub title_text_style: MonoTextStyle<'a, Gray8, VariableFont<'a>>,
    pub title_textbox_style: TextBoxStyle,
    pub body_text_style: MonoTextStyle<'a, Gray8, VariableFont<'a>>,
    pub body_textbox_style: TextBoxStyle,
    pub icon_margin: BoxOffset,
    pub notification_spacing: spacing::FixedMargin,
    pub warning_spacing: spacing::FixedMargin,
    // thing_to_show: ThingToShow,
    pub watermark_color: Gray8,
    pub progress_height: u32,
    pub progress_stroke_color: Gray8,
    pub progress_stroke_width: u32,
    pub progress_fill_color: Gray8,
    pub margin: BoxOffset,
}

// enum ThingToShow {
//     Nothing,
//     Notification {
//         title: heapless::String<256>,
//         body: heapless::String<256>,
//         notification_type: NotificationType,
//     }
// }

impl<'a> KeepKeyUI<'a> {
    pub fn new() -> Self {
        Self {
            title_text_style: MonoTextStyleBuilder::new()
                .font(&fonts::TITLE_FONT)
                .text_color(Gray8::WHITE)
                .build(),
            title_textbox_style: TextBoxStyleBuilder::new()
                .alignment(HorizontalAlignment::Left)
                .vertical_alignment(VerticalAlignment::Top)
                .line_height(LineHeight::Pixels(em(&fonts::TITLE_FONT, 14000)))
                .height_mode(HeightMode::FitToText)
                .build(),
            body_text_style: MonoTextStyleBuilder::new()
                .font(&fonts::BODY_FONT)
                .text_color(Gray8::WHITE)
                .build(),
            body_textbox_style: TextBoxStyleBuilder::new()
                .alignment(HorizontalAlignment::Left)
                .vertical_alignment(VerticalAlignment::Top)
                .line_height(LineHeight::Pixels(em(&fonts::BODY_FONT, 14000)))
                .height_mode(HeightMode::FitToText)
                .build(),
            icon_margin: (4, 0).into(),
            notification_spacing: spacing::FixedMargin(em(&fonts::BODY_FONT, 7000).try_into().unwrap()),
            warning_spacing: spacing::FixedMargin(em(&fonts::BODY_FONT, 6000).try_into().unwrap()),
            // thing_to_show: ThingToShow::Nothing,
            watermark_color: Gray8::new(0x22),
            progress_height: 6,
            progress_stroke_color: Gray8::WHITE,
            progress_stroke_width: 1,
            progress_fill_color: Gray8::WHITE,
            margin: 4.into(),
        }
    }

    pub fn layout_clear<D: DrawTarget<Color = Gray8>>(
        &mut self,
        target: &mut D,
    ) -> Result<(), D::Error> {
        target.clear(D::Color::BLACK)?;
        Ok(())
    }

    pub fn layout_standard_notification<D: DrawTarget<Color = Gray8>>(
        &mut self,
        title: &str,
        body: &str,
        notification_type: NotificationType,
        target: &mut D,
    ) -> Result<(), D::Error> {
        self.layout_clear(target)?;

        let null_image = ImageRaw::new(&[], 0);
        let (raw_image, image_offset) = match notification_type {
            NotificationType::Info => (&null_image, Point::zero()),
            NotificationType::Request => (&images::CONFIRM_ICON, Point::new(-5, 4)),
            NotificationType::Confirmed => (&images::CONFIRMING[63], Point::new(-5, 4)),
            NotificationType::Unplug => (&images::UNPLUG, Point::new(-3, 0)),
            NotificationType::Logo => (&images::SWORD_LOGO, Point::new(-2, 0)),
        };

        let frame = target.bounding_box().shrink(self.margin);
        let width = target
            .bounding_box()
            .shrink(self.margin)
            .shrink(BoxOffset::right(image_offset.x))
            .shrink(BoxOffset::right(raw_image.size().width.try_into().unwrap()))
            .size
            .width;
        // println!(
        //     "{:?}, {}",
        //     raw_image.bounding_box(),
        //     raw_image.bounding_box().top_left.x
        // );
        // println!("{} {}", title_width, body_width);

        LinearLayout::horizontal(
            Chain::new(
                LinearLayout::vertical(
                    Chain::new(TextBox::with_textbox_style(
                        &make_upper(title),
                        Rectangle::new(Point::zero(), Size::new(width, 0)),
                        self.title_text_style,
                        self.title_textbox_style,
                    ))
                    .append(TextBox::with_textbox_style(
                        &body,
                        Rectangle::new(Point::zero(), Size::new(width, 0)),
                        self.body_text_style,
                        self.body_textbox_style,
                    )),
                )
                .with_alignment(horizontal::Left)
                .with_spacing(self.notification_spacing)
                .arrange(),
            )
            .append(Image::new(raw_image, Point::zero())),
        )
        .with_spacing(spacing::FixedMargin(self.icon_margin.left))
        .with_alignment(vertical::Top)
        .layout_in(&frame, horizontal::Left, vertical::Center)
        .draw(target)?;

        let image = Image::new(
            raw_image,
            Point::new(
                TryInto::<i32>::try_into(target.bounding_box().size.width).unwrap()
                    - TryInto::<i32>::try_into(raw_image.size().width).unwrap(),
                0,
            ) + image_offset,
        );
        image.draw(target)?;

        // println!("bar {:?}", image.bounding_box());

        Ok(())
    }

    pub fn layout_warning<D: DrawTarget<Color = Gray8>>(
        &mut self,
        text: &str,
        target: &mut D,
    ) -> Result<(), D::Error> {
        self.layout_clear(target)?;

        let text_width = target.bounding_box().shrink(self.margin).size.width;

        LinearLayout::vertical(
            Chain::new(Image::new(&images::WARNING, Point::zero())).append(
                TextBox::with_textbox_style(
                    &text,
                    Rectangle::new(Point::zero(), Size::new(text_width, 0)),
                    self.body_text_style,
                    TextBoxStyleBuilder::from(&self.body_textbox_style)
                        .alignment(HorizontalAlignment::Center)
                        .build(),
                ),
            ),
        )
        .with_spacing(self.warning_spacing)
        .with_alignment(horizontal::Center)
        .layout_in(&target.bounding_box(), horizontal::Center, vertical::Center)
        .draw(target)?;

        Ok(())
    }

    pub fn layout_simple_message<D: DrawTarget<Color = Gray8>>(
        &mut self,
        text: &str,
        target: &mut D,
    ) -> Result<(), D::Error> {
        self.layout_clear(target)?;

        let text_width = target.bounding_box().shrink(self.margin).size.width;

        LinearLayout::vertical(Chain::new(TextBox::with_textbox_style(
            &make_upper(text),
            Rectangle::new(Point::zero(), Size::new(text_width, 0)),
            self.title_text_style,
            TextBoxStyleBuilder::from(&self.title_textbox_style)
                .alignment(HorizontalAlignment::Center)
                .build(),
        )))
        .with_alignment(horizontal::Center)
        .layout_in(&target.bounding_box(), horizontal::Center, vertical::Center)
        .draw(target)?;

        Ok(())
    }

    pub fn layout_watermark<D: DrawTarget<Color = Gray8>>(
        &mut self,
        text: &str,
        target: &mut D,
    ) -> Result<(), D::Error> {
        LinearLayout::vertical(Chain::new(Text::new(
            &text,
            Point::zero(),
            MonoTextStyleBuilder::from(&self.body_text_style)
                .text_color(self.watermark_color)
                .build(),
        )))
        .with_alignment(horizontal::Right)
        .layout_in(
            &target.bounding_box().shrink(self.margin),
            horizontal::Right,
            vertical::Bottom,
        )
        .draw(target)?;

        Ok(())
    }

    pub fn layout_progress<D: DrawTarget<Color = Gray8>>(
        &mut self,
        text: &str,
        basis_points: u32,
        target: &mut D,
    ) -> Result<(), D::Error> {
        self.layout_clear(target)?;

        let basis_points = core::cmp::min(basis_points, 10000);

        let max_progress_width = target.bounding_box().shrink(self.margin).size.width;
        let current_progress_width = max_progress_width * basis_points / 10000;

        // let progress_bar_bounding_box = ;
        let progress_bar = Chain::new(
            Rectangle::new(
                Point::zero(),
                Size::new(max_progress_width, self.progress_height),
            )
            .into_styled(PrimitiveStyle::with_stroke(
                self.progress_stroke_color,
                self.progress_stroke_width,
            )),
        )
        .append(
            Rectangle::new(
                Point::zero(),
                Size::new(current_progress_width, self.progress_height),
            )
            .shrink(self.progress_stroke_width.try_into().unwrap())
            .into_styled(PrimitiveStyle::with_fill(self.progress_fill_color)),
        );

        LinearLayout::vertical(
            Chain::new(Text::new(
                &text,
                Point::zero(),
                self.body_text_style,
            ))
            .append(progress_bar),
        )
        .with_spacing(self.notification_spacing)
        .layout_in(
            &target.bounding_box().shrink(self.margin),
            horizontal::Left,
            vertical::Center,
        )
        .draw(target)?;

        Ok(())
    }
}
