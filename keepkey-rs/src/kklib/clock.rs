use super::board::KeepKeyBoard;
use core::{
  future::Future,
  pin::Pin,
  task::{Context, Poll, Waker},
};
use futures_intrusive::timer::GenericTimerService;

pub struct Clock {
  offset: u64,
}

impl Clock {
  pub fn new() -> Self {
    Self {
      offset: KeepKeyBoard::clock_ms(),
    }
  }
  const fn default() -> Self {
    Self { offset: 0 }
  }
  // fn deadline
}

impl Default for Clock {
  fn default() -> Self {
    Self::default()
  }
}

impl futures_intrusive::timer::Clock for Clock {
  fn now(&self) -> u64 {
    KeepKeyBoard::clock_ms() - self.offset
  }
}

pub struct TimerServiceFuture<T: lock_api::RawMutex> {
  timer_service: GenericTimerService<T>,
  waker: core::cell::Cell<Option<Waker>>,
}

impl<T: lock_api::RawMutex> Future for TimerServiceFuture<T> {
  type Output = ();

  fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
    self.timer_service.check_expirations();
    self.waker.set(Some(cx.waker().clone()));
    if let Some(x) = self.timer_service.next_expiration() {
      KeepKeyBoard::set_clock_compare_ms(x);
    }
    Poll::Pending
  }
}

pub static DEFAULT_CLOCK: Clock = Clock::default();

// lazy_static::lazy_static! {
//   pub static ref TIMER_SERVICE_FUTURE: TimerServiceFuture<spin::Mutex<()>> = TimerServiceFuture {
//     timer_service: GenericTimerService::<spin::Mutex<()>>::new(&DEFAULT_CLOCK),
//     waker: core::cell::Cell::new(None),
//   };
// }

/// This is called from within an ISR.
#[no_mangle]
pub extern "C" fn rust_clock_compare_callback(_ms: u64) {
  // if let Some(x) = TIMER_SERVICE_FUTURE.waker.take() {
  //   x.wake()
  // }
}
