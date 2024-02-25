#[deny(unsafe_op_in_unsafe_fn)]

use core::{
  cell::Cell,
  clone::Clone,
  fmt,
  marker::{Copy, PhantomPinned},
  ops::Deref,
  pin::Pin,
  ptr::NonNull,
};

struct Ref<T: ?Sized> {
  value: &T
}

#[repr(transparent)]
struct Readonly<T: ?Sized> {
  value: T,
}

impl<T> Readonly<T> {
  #[inline]
  pub const fn new(value: T) -> Self {
    Self {
      value,
    }
  }
  #[inline]
  pub const fn into_inner(self) -> T {
    self.value
  }
}

impl<T: ?Sized> Readonly<T> {
  #[inline]
  pub const fn as_ref(&self) -> &T {
    &self.value
  }
  #[inline]
  pub const fn as_ptr(&self) -> *const T {
    &self.value as *const T
  }
}

impl<T> From<T> for Readonly<T> {
  #[inline]
  fn from(x: T) -> Self {
    Self::new(x)
  }
}

impl<T: Default> Default for Readonly<T> {
  #[inline]
  fn default() -> Self {
    Self::new(T::default())
  }
}

impl<T: ?Sized + core::fmt::Display> fmt::Display for Readonly<T> {
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    self.value.fmt(f)
  }
}

impl<T: ?Sized> Deref for Readonly<T> {
  type Target = T;

  #[inline]
  fn deref(&self) -> &Self::Target {
    self.as_ref()
  }
}

impl<T: ?Sized + Clone> Clone for Readonly<T> {
  #[inline]
  fn clone(&self) -> Self {
    Self {
      value: self.value.clone(),
    }
  }
}

impl<T: ?Sized + Copy> Copy for Readonly<T> {}
