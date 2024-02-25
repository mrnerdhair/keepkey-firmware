use core::{
  future::Future,
  pin::Pin,
  task::{Context, Poll, Waker},
};

pub struct Deferred<T> {
  value: Option<Option<T>>,
  waker: Option<Waker>,
}

impl<T> Deferred<T> {
  pub fn new() -> Self {
    Self {
      value: None,
      waker: None,
    }
  }

  pub fn resolve(&mut self, x: T) {
    match self.value {
      Some(_) => panic!("tried to resolve a resolved Deferred"),
      None => self.value = Some(Some(x)),
    }
    if let Some(waker) = self.waker.take() {
      waker.wake();
    }
  }
}

impl<T> Default for Deferred<T> {
  fn default() -> Self {
    Self::new()
  }
}

impl<T> Future for Deferred<T> where
  T: Unpin
{
  type Output = T;
  fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
    let self_unpinned = Pin::into_inner(self);
    match self_unpinned.value.take() {
      Some(mut x) => match x.take() {
        Some(y) => Poll::Ready(y),
        None => panic!("tried to poll a ready Deferred"),
      },
      None => {
        self_unpinned.waker = Some(cx.waker().clone());
        Poll::Pending
      },
    }
  }
}

trait Generator<T, TNext> {
  type Future: Future<Output = Option<T>>;
  fn next(&mut self, x: TNext) -> Self::Future;
}

impl<T, TNext, F, FReturn> Generator<T, TNext> for F
where
  F: FnMut(TNext) -> FReturn,
  FReturn: Future<Output = Option<T>>
{
  type Future = FReturn;
  fn next(&mut self, x: TNext) -> Self::Future {
    self(x)
  }
}

trait GeneratorContext<T, TNext> {
  type Future: Future<Output = T>;
  fn r#yield(&mut self, x: TNext) -> Self::Future;
}

enum FooGeneratorState<T> {
  BeforeFirstYield,
  Yielded(T),
  Complete(T),
}

// struct FooGeneratorContext<T, TNext> {
//   next_output: Deferred<T>,
//   next_type: core::marker::PhantomData<*const TNext>,
// }

// impl<T, TNext> GeneratorContext<T, TNext>  for FooGeneratorContext<T, TNext> {
//   type Future = Deferred<T>;
//   fn r#yield(&mut self, x: TNext) -> Self::Future {
//     //TODO: fix this
//     self.next_output
//   }
// }

// struct Bar<T> {
//   pool: heapless::pool::Pool<dyn Future<Output = T>>::new(),
// }

// struct FooGeneratorRunner<T, TNext, TReturnFuture>
// where
//   TReturnFuture: Future<Output = T>,
// {
//   state: FooGeneratorState<T>,
//   ctx: FooGeneratorContext<T, TNext>,
//   return_future: Option<TReturnFuture>,
// }

// impl<T, TNext, TFuture, TNextFuture> FooGeneratorRunner<T, TNext>
// where
//   TFuture: Future<Output = T>,
//   TNextFuture: Future<Output = TNext>,
// {
//   fn new<F>(f: F) -> Self
//   where
//     F: FnMut(&mut dyn GeneratorContext<T, TNext, Future = TFuture>) -> TFuture
//   {
//     let ctx = FooGeneratorContext::<T, TNext> {
//       next_output: Deferred::<T>::new(),
//       next_type: core::marker::PhantomData,
//     };
//     Self {
//       state: FooGeneratorState::BeforeFirstYield,
//       ctx,
//       return_future: Some(f(&mut ctx)),
//     }
//   }
// }

// struct GeneratorIterator<T>
// where
//   T: Generator<>
// {
//   generator: T
// }

// struct USBPacketHandler<T>
// where
//   T: Generator<heapless::Vec<u8, 64>, heapless::Vec<u8, 64>>,
// {
//   handler: T
// }

// impl USBPacketHandler {
//   fn processPacket(&mut self, x: heapless::Vec<u8, 64>) -> heapless::Vec<u8, 64> {
//     spin_on::spin_on(self.handler.next(x));
//   }
// }
