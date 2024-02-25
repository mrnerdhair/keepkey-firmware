use super::{
  box_offset::{BoxOffset, RectangleExt}
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

pub struct AutoSizable<V: View + Resizable> {
  view: V
}

impl<V: View + Resizable> Dimensions for AutoSizable<V> {
  fn bounding_box(&self) -> Rectangle {
      self.view.bounding_box()
  }
}

impl<V: View + Resizable> View for AutoSizable<V> {
  fn translate_impl(&mut self, by: Point) {
      self.view.translate_mut(by + Point::new(self.padding.left, self.padding.right));
  }
  fn bounds(&self) -> Rectangle {
      self.view.bounds()
  }
}

impl<C: PixelColor, V: View + Resizable + Drawable<Color = C>> Drawable for AutoSizable<V> {
  type Color = C;
  type Output = V::Output;
  fn draw<D: DrawTarget<Color = C>>(&self, display: &mut D) -> Result<Self::Output, D::Error> {
      self.view.draw(display)
  }
}

impl<V: View + Resizable> Resizable for AutoSizable<V> {
  fn resize_mut(&mut self, size: Size, anchor_point: AnchorPoint) {
    self.view.resize_mut(size, anchor_point)
  }
}
