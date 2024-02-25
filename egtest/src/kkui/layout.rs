use super::resizable::Resizable;
use embedded_graphics::{
  geometry::{Point, Size},
  Drawable,
};
use embedded_layout::{
  align::{Align, HorizontalAlignment, VerticalAlignment},
  layout::linear::{LinearLayout, Orientation},
  view_group::ViewGroup,
  View,
};

pub trait Layout: Align + View {
  type ViewGroup: ViewGroup;
  fn layout_in<H: HorizontalAlignment, V: VerticalAlignment>(
    self,
    reference: &impl View,
    horizontal: H,
    vertical: V,
  ) -> Self;
  fn into_inner(self) -> Self::ViewGroup;
}

impl<LD, VG> Layout for LinearLayout<LD, VG>
where
  LD: Orientation,
  VG: ViewGroup,
{
  type ViewGroup = VG;
  fn layout_in<H: HorizontalAlignment, V: VerticalAlignment>(
    self,
    reference: &impl View,
    horizontal: H,
    vertical: V,
  ) -> Self {
    self.arrange().align_to(reference, horizontal, vertical)
  }
  fn into_inner(self) -> Self::ViewGroup {
    self.into_inner()
  }
}

// pub struct LinearLayout<LD, VG>
// where
//     LD: Orientation,
//     VG: ViewGroup,
// {
//     position: Point,
//     direction: LD,
//     views: VG,
// }
