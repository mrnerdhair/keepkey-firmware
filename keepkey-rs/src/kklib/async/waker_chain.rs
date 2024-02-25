use core::{
  cell::Cell,
  task::Waker,
};

pub struct WakerChainInner<'a> {
  next: Option<&'a mut WakerChain<'a>>,
  waker: Waker,
}

pub struct WakerChain<'a> {
  inner: Cell<Option<WakerChainInner<'a>>>,
}

impl<'a> WakerChain<'a> {
  pub fn wake(&mut self) {
    let mut inner = self.inner.take();
    match inner {
      Some(mut x) => {
        x.waker.wake();
        let next = x.next.take();
        if let Some(y) = next {
          y.wake()
        }
      }
      None => (),
    }
  }

  pub fn after(&'a mut self, waker: Waker) -> Self {
    let mut out = Self::from(waker);
    let mut inner = out.inner.take().unwrap();
    inner.next = Some(self);
    out.inner.set(Some(inner));
    out
  }
}

impl From<Waker> for WakerChain<'_> {
  fn from(waker: Waker) -> Self {
    WakerChain {
      inner: Cell::new(Some(WakerChainInner { next: None, waker })),
    }
  }
}
