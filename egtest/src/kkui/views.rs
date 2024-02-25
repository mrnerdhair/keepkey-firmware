pub trait SizedView: View + Sized {}

impl<T: View + Sized> SizedView for T {}
