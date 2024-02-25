use super::{
    box_offset::{BoxOffset, RectangleExt},
    resizable::Resizable,
};
use embedded_graphics::{
    Drawable,
    Pixel,
    pixelcolor::PixelColor,
    draw_target::DrawTarget,
    geometry::{AnchorPoint, Dimensions, Point, Size},
    primitives::Rectangle,
    transform::Transform,
};
use embedded_layout::View;

// enum Position {
//     Static,
//     Relative(BoxOffset),
//     Absolute(BoxOffset),
// }

pub struct Box<V: View> {
    view: V,
    margin: BoxOffset,
    padding: BoxOffset,
    // height: Option<u32>,
    // max_height: Option(u32),
    // width: Option<u32>,
    // max_width: Option(u32),
}

impl<V: View> Dimensions for Box<V> {
    fn bounding_box(&self) -> Rectangle {
        self.bounds()
    }
}

impl<V: View> View for Box<V> {
    fn translate_impl(&mut self, by: Point) {
        self.view.translate_mut(by);
    }
    fn bounds(&self) -> Rectangle {
        self.view.bounds().expand(self.padding)
        // let source = self.view.bounds().box_offset(self.padding);
        // source.resized(Size::new(
        //     self.width.unwrap_or(source.size.width),
        //     self.height.unwrap_or(source.size.height),
        // ), AnchorPoint::TopLeft)
    }
}

impl<C: PixelColor, V: View + Drawable<Color = C>> Drawable for Box<V> {
    type Color = C;
    type Output = V::Output;
    fn draw<D: DrawTarget<Color = C>>(&self, display: &mut D) -> Result<Self::Output, D::Error> {
        self.view.draw(display)
    }
}

impl<V: View + Resizable> Resizable for Box<V> {
    fn resize_mut(&mut self, size: Size, anchor_point: AnchorPoint) {
        self.view.resize_mut(size, anchor_point)
    }
}
