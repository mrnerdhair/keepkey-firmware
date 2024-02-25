use core::{
    convert::{From, TryFrom},
    num::TryFromIntError,
};
use embedded_graphics::{
    geometry::{Point, Size},
    primitives::Rectangle,
};

#[derive(Debug, Clone, Copy)]
pub struct BoxOffset {
    pub top: i32,
    pub left: i32,
    pub bottom: i32,
    pub right: i32,
}

impl BoxOffset {
    pub fn zero() -> Self {
        (0, 0, 0, 0).into()
    }

    pub fn top(x: i32) -> Self {
        (x, 0, 0, 0).into()
    }

    pub fn left(x: i32) -> Self {
        (0, x, 0, 0).into()
    }

    pub fn bottom(x: i32) -> Self {
        (0, 0, x, 0).into()
    }

    pub fn right(x: i32) -> Self {
        (0, 0, 0, x).into()
    }

    pub fn translation(&self) -> Point {
        Point::new(self.left, self.top)
    }
}

impl core::ops::Neg for BoxOffset {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            top: -self.top,
            left: -self.left,
            bottom: -self.bottom,
            right: -self.right,
        }
    }
}

impl core::ops::AddAssign for BoxOffset {
    fn add_assign(&mut self, rhs: Self) {
        self.top += rhs.top;
        self.left += rhs.left;
        self.bottom += rhs.bottom;
        self.right += rhs.right;
    }
}

impl core::ops::Add for BoxOffset {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self {
        self += rhs;
        self
    }
}

impl core::ops::SubAssign for BoxOffset {
    fn sub_assign(&mut self, rhs: Self) {
        *self += -rhs;
    }
}

impl core::ops::Sub for BoxOffset {
    type Output = Self;
    fn sub(mut self, rhs: Self) -> Self {
        self -= rhs;
        self
    }
}

impl Default for BoxOffset {
    fn default() -> Self {
        Self::zero()
    }
}

impl From<(i32, i32, i32, i32)> for BoxOffset {
    fn from(val: (i32, i32, i32, i32)) -> Self {
        Self {
            top: val.0,
            left: val.1,
            bottom: val.2,
            right: val.3,
        }
    }
}

impl From<(i32, i32, i32)> for BoxOffset {
    fn from(val: (i32, i32, i32)) -> Self {
        Self {
            top: val.0,
            left: val.1,
            bottom: val.2,
            right: val.1,
        }
    }
}

impl From<(i32, i32)> for BoxOffset {
    fn from(val: (i32, i32)) -> Self {
        Self {
            top: val.0,
            left: val.1,
            bottom: val.0,
            right: val.1,
        }
    }
}

impl From<(i32,)> for BoxOffset {
    fn from(val: (i32,)) -> Self {
        Self {
            top: val.0,
            left: val.0,
            bottom: val.0,
            right: val.0,
        }
    }
}

impl From<i32> for BoxOffset {
    fn from(val: i32) -> Self {
        Self {
            top: val,
            left: val,
            bottom: val,
            right: val,
        }
    }
}

impl<T, U, V> TryFrom<(u32, T, U, V)> for BoxOffset
where
    T: TryInto<i32>,
    U: TryInto<i32>,
    V: TryInto<i32>,
{
    type Error = ();
    fn try_from(val: (u32, T, U, V)) -> Result<Self, Self::Error> {
        let val: (i32, i32, i32, i32) = (
            val.0.try_into().map_err(|_x| ())?,
            val.1.try_into().map_err(|_x| ())?,
            val.2.try_into().map_err(|_x| ())?,
            val.3.try_into().map_err(|_x| ())?,
        );
        Ok(Self::from(val))
    }
}

impl<T, U> TryFrom<(i32, u32, T, U)> for BoxOffset
where
    T: TryInto<i32>,
    U: TryInto<i32>,
{
    type Error = ();
    fn try_from(val: (i32, u32, T, U)) -> Result<Self, Self::Error> {
        let val: (i32, i32, i32, i32) = (
            val.0.try_into().map_err(|_x| ())?,
            val.1.try_into().map_err(|_x| ())?,
            val.2.try_into().map_err(|_x| ())?,
            val.3.try_into().map_err(|_x| ())?,
        );
        Ok(Self::from(val))
    }
}

impl<T> TryFrom<(i32, i32, u32, T)> for BoxOffset
where
    T: TryInto<i32>,
{
    type Error = ();
    fn try_from(val: (i32, i32, u32, T)) -> Result<Self, Self::Error> {
        let val: (i32, i32, i32, i32) = (
            val.0.try_into().map_err(|_x| ())?,
            val.1.try_into().map_err(|_x| ())?,
            val.2.try_into().map_err(|_x| ())?,
            val.3.try_into().map_err(|_x| ())?,
        );
        Ok(Self::from(val))
    }
}

impl TryFrom<(i32, i32, i32, u32)> for BoxOffset {
    type Error = ();
    fn try_from(val: (i32, i32, i32, u32)) -> Result<Self, Self::Error> {
        let val: (i32, i32, i32, i32) = (
            val.0.try_into().map_err(|_x| ())?,
            val.1.try_into().map_err(|_x| ())?,
            val.2.try_into().map_err(|_x| ())?,
            val.3.try_into().map_err(|_x| ())?,
        );
        Ok(Self::from(val))
    }
}

impl<T, U> TryFrom<(u32, T, U)> for BoxOffset
where
    T: TryInto<i32>,
    U: TryInto<i32>,
{
    type Error = ();
    fn try_from(val: (u32, T, U)) -> Result<Self, Self::Error> {
        let val: (i32, i32, i32) = (
            val.0.try_into().map_err(|_x| ())?,
            val.1.try_into().map_err(|_x| ())?,
            val.2.try_into().map_err(|_x| ())?,
        );
        Ok(Self::from(val))
    }
}

impl<T> TryFrom<(i32, u32, T)> for BoxOffset
where
    T: TryInto<i32>,
{
    type Error = ();
    fn try_from(val: (i32, u32, T)) -> Result<Self, Self::Error> {
        let val: (i32, i32, i32) = (
            val.0.try_into().map_err(|_x| ())?,
            val.1.try_into().map_err(|_x| ())?,
            val.2.try_into().map_err(|_x| ())?,
        );
        Ok(Self::from(val))
    }
}

impl TryFrom<(i32, i32, u32)> for BoxOffset {
    type Error = ();
    fn try_from(val: (i32, i32, u32)) -> Result<Self, Self::Error> {
        let val: (i32, i32, i32) = (
            val.0.try_into().map_err(|_x| ())?,
            val.1.try_into().map_err(|_x| ())?,
            val.2.try_into().map_err(|_x| ())?,
        );
        Ok(Self::from(val))
    }
}

impl<T> TryFrom<(u32, T)> for BoxOffset
where
    T: TryInto<i32>,
{
    type Error = ();
    fn try_from(val: (u32, T)) -> Result<Self, Self::Error> {
        let val: (i32, i32) = (
            val.0.try_into().map_err(|_x| ())?,
            val.1.try_into().map_err(|_x| ())?,
        );
        Ok(Self::from(val))
    }
}

impl TryFrom<(i32, u32)> for BoxOffset {
    type Error = ();
    fn try_from(val: (i32, u32)) -> Result<Self, Self::Error> {
        let val: (i32, i32) = (
            val.0.try_into().map_err(|_x| ())?,
            val.1.try_into().map_err(|_x| ())?,
        );
        Ok(Self::from(val))
    }
}

impl TryFrom<(u32,)> for BoxOffset {
    type Error = ();
    fn try_from(val: (u32,)) -> Result<Self, Self::Error> {
        let val: (i32,) = (val.0.try_into().map_err(|_x| ())?,);
        Ok(Self::from(val))
    }
}

impl TryFrom<u32> for BoxOffset {
    type Error = ();
    fn try_from(val: u32) -> Result<Self, Self::Error> {
        let val: i32 = val.try_into().map_err(|_x| ())?;
        Ok(Self::from(val))
    }
}

fn saturating_add_inverse_if_negative(x: u32, y: i32) -> u32 {
    if y < 0 {
        x.saturating_add((-y).try_into().unwrap())
    } else {
        x
    }
}

fn saturating_sub_if_not_negative(x: u32, y: i32) -> u32 {
    if y > 0 {
        x.saturating_sub(y.try_into().unwrap())
    } else {
        x
    }
}

pub trait RectangleExt: Sized {
    fn shrink(self, offset: BoxOffset) -> Self;
    fn expand(self, offset: BoxOffset) -> Self {
        self.shrink(-offset)
    }
}

impl RectangleExt for Rectangle {
    fn shrink(self, offset: BoxOffset) -> Self {
        Self::new(
            self.top_left + offset.translation(),
            self.size.shrink(offset),
        )
    }
}

pub trait SizeExt: Sized {
    fn shrink(self, offset: BoxOffset) -> Self;
    fn expand(self, offset: BoxOffset) -> Self {
        self.shrink(-offset)
    }
}

impl SizeExt for Size {
    fn shrink(self, offset: BoxOffset) -> Self {
        let mut width = self.width;
        width = saturating_add_inverse_if_negative(width, offset.left);
        width = saturating_add_inverse_if_negative(width, offset.right);
        width = saturating_sub_if_not_negative(width, offset.left);
        width = saturating_sub_if_not_negative(width, offset.right);

        let mut height = self.height;
        height = saturating_add_inverse_if_negative(height, offset.top);
        height = saturating_add_inverse_if_negative(height, offset.bottom);
        height = saturating_sub_if_not_negative(height, offset.top);
        height = saturating_sub_if_not_negative(height, offset.bottom);

        Size::new(width, height)
    }
}

#[test]
fn top_only() {
    let x = Rectangle::with_corners(Point::new(5, 15), Point::new(20, 25));
    let y = x.shrink(BoxOffset::from((5, 0, 0, 0)));
    assert_eq!(y, Rectangle::with_corners(Point::new(5, 20), Point::new(20, 25)));
}

#[test]
fn left_only() {
    let x = Rectangle::with_corners(Point::new(5, 15), Point::new(20, 25));
    let y = x.shrink(BoxOffset::from((0, 5, 0, 0)));
    assert_eq!(y, Rectangle::with_corners(Point::new(10, 15), Point::new(20, 25)));
}

#[test]
fn bottom_only() {
    let x = Rectangle::with_corners(Point::new(5, 15), Point::new(20, 25));
    let y = x.shrink(BoxOffset::from((0, 0, 5, 0)));
    assert_eq!(y, Rectangle::with_corners(Point::new(5, 15), Point::new(20, 20)));
}

#[test]
fn right_only() {
    let x = Rectangle::with_corners(Point::new(5, 15), Point::new(20, 25));
    let y = x.shrink(BoxOffset::from((0, 0, 0, 5)));
    assert_eq!(y, Rectangle::with_corners(Point::new(5, 15), Point::new(15, 25)));
}

#[test]
fn negative_top_only() {
    let x = Rectangle::with_corners(Point::new(5, 15), Point::new(20, 25));
    let y = x.shrink(BoxOffset::from((-5, 0, 0, 0)));
    assert_eq!(y, Rectangle::with_corners(Point::new(5, 10), Point::new(20, 25)));
}

#[test]
fn negative_left_only() {
    let x = Rectangle::with_corners(Point::new(5, 15), Point::new(20, 25));
    let y = x.shrink(BoxOffset::from((0, -5, 0, 0)));
    assert_eq!(y, Rectangle::with_corners(Point::new(0, 15), Point::new(20, 25)));
}

#[test]
fn negative_bottom_only() {
    let x = Rectangle::with_corners(Point::new(5, 15), Point::new(20, 25));
    let y = x.shrink(BoxOffset::from((0, 0, -5, 0)));
    assert_eq!(y, Rectangle::with_corners(Point::new(5, 15), Point::new(20, 30)));
}

#[test]
fn negative_right_only() {
    let x = Rectangle::with_corners(Point::new(5, 15), Point::new(20, 25));
    let y = x.shrink(BoxOffset::from((0, 0, 0, -5)));
    assert_eq!(y, Rectangle::with_corners(Point::new(5, 15), Point::new(25, 25)));
}

#[test]
fn remove_all_space() {
    let x = Rectangle::with_corners(Point::new(5, 15), Point::new(20, 25));
    let y = x.shrink(BoxOffset::from((11, 16, 0, 0)));
    assert_eq!(y, Rectangle::new(Point::new(21, 26), Size::zero()));
}

#[test]
fn remove_all_space_2() {
    let x = Rectangle::with_corners(Point::new(5, 15), Point::new(20, 25));
    let y = x.shrink(BoxOffset::from((0, 0, 11, 16)));
    assert_eq!(y, Rectangle::new(Point::new(5, 15), Size::zero()));
}

#[test]
fn remove_all_space_3() {
    let x = Rectangle::with_corners(Point::new(5, 15), Point::new(20, 25));
    let y = x.shrink(BoxOffset::from((5, 5, 11, 11)));
    assert_eq!(y, Rectangle::new(Point::new(10, 20), Size::zero()));
}

#[test]
fn impossible_1() {
    let x = Rectangle::with_corners(Point::new(5, 15), Point::new(20, 25));
    let y = x.shrink(BoxOffset::from((999, 999, 999, 999)));
    assert_eq!(y, Rectangle::new(Point::new(1004, 1014), Size::zero()));
}

#[test]
fn large_translation() {
    let x = Rectangle::with_corners(Point::new(5, 15), Point::new(20, 25));
    let y = x.shrink(BoxOffset::from((100, 100, -100, -100)));
    assert_eq!(y, Rectangle::with_corners(Point::new(105, 115), Point::new(120, 125)));
}
