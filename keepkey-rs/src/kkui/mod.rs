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

pub mod fonts;
pub mod images;

pub(crate) fn padding<C: GrayColor>(
    width: u32,
    height: u32,
) -> Styled<Rectangle, PrimitiveStyle<C>> {
    Rectangle::new(Point::zero(), Size::new(width, height))
        .into_styled(PrimitiveStyle::with_stroke(C::BLACK, 0))
}

fn image_anchored_at_point<'a, 'b, C, D>(image: &'a ImageRaw<'a, C>, target: &'b D, anchor_point: AnchorPoint, offset: Point) -> Image<'a, ImageRaw<'a, C>>
where
  ImageRaw<'a, C>: ImageDrawable,
  C: PixelColor + From<<C as PixelColor>::Raw>,
  D: DrawTarget<Color = C>,
{
    let image_width: i32 = image.size().width.try_into().unwrap();
    let image_height: i32 = image.size().height.try_into().unwrap();

    let anchor_point_correction = Point::new(match anchor_point {
        AnchorPoint::TopLeft | AnchorPoint::CenterLeft | AnchorPoint::BottomLeft => 0,
        AnchorPoint::TopCenter | AnchorPoint::Center | AnchorPoint::BottomCenter => 1,
        AnchorPoint::TopRight | AnchorPoint::CenterRight | AnchorPoint::BottomRight => 1,
    }, match anchor_point {
        AnchorPoint::TopLeft | AnchorPoint::TopCenter | AnchorPoint::TopRight => 0,
        AnchorPoint::CenterLeft | AnchorPoint::Center | AnchorPoint::CenterRight => 1,
        AnchorPoint::BottomLeft | AnchorPoint::BottomCenter | AnchorPoint::BottomRight => 1,
    });
    let corrected_anchor_point = target.bounding_box().anchor_point(anchor_point) + anchor_point_correction;

    let image_origin_offset = Point::new(match anchor_point {
        AnchorPoint::TopLeft | AnchorPoint::CenterLeft | AnchorPoint::BottomLeft => 0,
        AnchorPoint::TopCenter | AnchorPoint::Center | AnchorPoint::BottomCenter => -(image_width / 2),
        AnchorPoint::TopRight | AnchorPoint::CenterRight | AnchorPoint::BottomRight => -image_width,
    }, match anchor_point {
        AnchorPoint::TopLeft | AnchorPoint::TopCenter | AnchorPoint::TopRight => 0,
        AnchorPoint::CenterLeft | AnchorPoint::Center | AnchorPoint::CenterRight => -(image_height / 2),
        AnchorPoint::BottomLeft | AnchorPoint::BottomCenter | AnchorPoint::BottomRight => -image_height,
    });
    let image_origin = corrected_anchor_point + image_origin_offset + offset;

    Image::new(image, image_origin)
}

pub enum NotificationType {
    Logo,
    Info,
    Request,
    // ConfirmAnimation,
    Confirmed,
    Unplug,
}

#[non_exhaustive]
pub struct KeepKeyUI<'a>
{
    pub title_text_style: MonoTextStyle<'a, Gray8, VariableFont<'a>>,
    pub title_textbox_style: TextBoxStyle,
    pub body_text_style: MonoTextStyle<'a, Gray8, VariableFont<'a>>,
    pub body_textbox_style: TextBoxStyle,
    pub left_margin: u32,
    pub warning_top_margin: u32,
    pub warning_body_top_margin: u32,
}

impl<'a> KeepKeyUI<'a> {
    pub fn new() -> Self {
        // let body_top_margin = TryInto::<i32>::try_into(self.body_text_style.line_height()).unwrap() * 70 / 100;
        Self {
            title_text_style: MonoTextStyle::new(&fonts::TITLE_FONT, Gray8::WHITE),
            title_textbox_style: TextBoxStyleBuilder::new()
                .alignment(HorizontalAlignment::Left)
                .vertical_alignment(VerticalAlignment::Top)
                .line_height(LineHeight::Pixels(fonts::TITLE_FONT.character_height()))
                .height_mode(HeightMode::FitToText)
                .build(),
            body_text_style: MonoTextStyle::new(&fonts::BODY_FONT, Gray8::WHITE),
            body_textbox_style: TextBoxStyleBuilder::new()
                .alignment(HorizontalAlignment::Left)
                .vertical_alignment(VerticalAlignment::Top)
                .line_height(LineHeight::Pixels(fonts::BODY_FONT.character_height() + 4))
                .height_mode(HeightMode::FitToText)
                .build(),
            left_margin: 4,
            body_top_margin: 7,
            warning_top_margin: 7,
            warning_body_top_margin: 7,
        }
    }

    pub fn layout_standard_notification<D: DrawTarget<Color = Gray8>>(
        &mut self,
        title: &str,
        body: &str,
        notification_type: NotificationType,
        target: &mut D,
    ) -> Result<(), D::Error> {
        target.clear(D::Color::BLACK)?;

        let top_right = target.bounding_box().anchor_point(AnchorPoint::TopRight);
        let null_image = ImageRaw::default();
        let image = match notification_type {
            NotificationType::Info => image_anchored_at_point(&null_image, target, AnchorPoint::TopRight, Point::zero()),
            NotificationType::Request => image_anchored_at_point(&images::CONFIRM_ICON, target, AnchorPoint::TopRight, Point::new(-3, 2)),
            NotificationType::Confirmed => image_anchored_at_point(&images::CONFIRMING[63], target, AnchorPoint::TopRight, Point::new(-1, 4)),
            NotificationType::Unplug => image_anchored_at_point(&images::UNPLUG, target, AnchorPoint::CenterRight, Point::new(-3, 0)),
            NotificationType::Logo => image_anchored_at_point(&images::SWORD_LOGO, target, AnchorPoint::CenterRight, Point::new(-2, 0)),
        };

        let mut title_upper: heapless::String<128> =
            heapless::String::from_str(&title[..core::cmp::min(title.len(), 128)]).unwrap();
        title_upper.make_ascii_uppercase();

        let title_size = target.bounding_box().size - Size::new(50, 0);
        let body_size = target.bounding_box().size - Size::new(31, 0);

        LinearLayout::horizontal(
            Chain::new(padding(self.left_margin, 0)).append(
                LinearLayout::vertical(
                    Chain::new(TextBox::with_textbox_style(
                        &title_upper,
                        Rectangle::new(Point::zero(), title_size),
                        self.title_text_style,
                        self.title_textbox_style,
                    ))
                    .append(TextBox::with_textbox_style(
                        &body,
                        Rectangle::new(Point::zero(), body_size),
                        self.body_text_style,
                        self.body_textbox_style,
                    )),
                )
                .with_spacing(spacing::FixedMargin(
                    self.body_top_margin.try_into().unwrap(),
                ))
                .arrange(),
            ),
        )
        .arrange()
        .align_to(
            &target.bounding_box(),
            horizontal::Left,
            vertical::Center,
        )
        .draw(target)?;
        
        image.draw(target)?;

        Ok(())
    }

    pub fn layout_warning<D: DrawTarget<Color = Gray8>>(&mut self, text: &str, target: &mut D) -> Result<(), D::Error> {
        target.clear(D::Color::BLACK)?;

        LinearLayout::vertical(
            Chain::new(padding(0, self.warning_top_margin)).append(
                LinearLayout::vertical(
                    Chain::new(TextBox::with_textbox_style(
                        &title_upper,
                        Rectangle::new(Point::zero(), title_size),
                        self.title_text_style,
                        self.title_textbox_style,
                    ))
                    .append(TextBox::with_textbox_style(
                        &body,
                        Rectangle::new(Point::zero(), body_size),
                        self.body_text_style,
                        self.body_textbox_style,
                    )),
                )
                .with_spacing(spacing::FixedMargin(
                    self.body_top_margin.try_into().unwrap(),
                ))
                .arrange(),
            ),
        )
        .arrange()
        .align_to(
            &target.bounding_box(),
            horizontal::Left,
            vertical::Center,
        )
        .draw(target)?;
        
        image.draw(target)?;

        Ok(())
    }
}
