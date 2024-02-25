use embedded_graphics::{
    geometry::{AnchorPoint, Size},
    primitives::Rectangle,
    text::renderer::TextRenderer,
};
use embedded_text::TextBox;

pub trait Resizable {
    fn resize(mut self, size: Size, anchor_point: AnchorPoint) -> Self
    where
        Self: Sized,
    {
        self.resize_mut(size, anchor_point);
        self
    }
    fn resize_mut(&mut self, size: Size, anchor_point: AnchorPoint);
}

impl Resizable for Rectangle {
    fn resize_mut(&mut self, size: Size, anchor_point: AnchorPoint) {
        core::mem::swap(self, &mut self.resized(size, anchor_point));
    }
}

impl<S: TextRenderer, M> Resizable for TextBox<'_, S, M> {
    fn resize_mut(&mut self, size: Size, anchor_point: AnchorPoint) {
        self.bounds.resize_mut(size, anchor_point);
    }
}
