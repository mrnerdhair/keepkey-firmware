#![allow(dead_code)]

use super::deferred::Deferred;

pub struct AsyncQueue<T, const N: usize> {
  queue: heapless::spsc::Queue<T, N>,
  next_enqueue_event: Option<Deferred<()>>,
  next_dequeue_event: Option<Deferred<()>>,
}

impl<'a, T, const N: usize> AsyncQueue<T, N> {
  pub const fn new() -> Self {
    let out = AsyncQueue {
      queue: heapless::spsc::Queue::new(),
      next_enqueue_event: None,
      next_dequeue_event: None,
    };
    out
  }
  pub const fn capacity(&self) -> usize {
    self.queue.capacity()
  }
  pub fn len(&self) -> usize {
    self.queue.len()
  }
  pub fn is_empty(&self) -> bool {
    self.queue.is_empty()
  }
  pub fn is_full(&self) -> bool {
    self.queue.is_full()
  }
  pub fn iter(&self) -> heapless::spsc::Iter<'_, T, N> {
    self.queue.iter()
  }
  pub fn iter_mut(&mut self) -> heapless::spsc::IterMut<'_, T, N> {
    self.queue.iter_mut()
  }
  pub async fn enqueue(&mut self, val: T) {
    let mut val = Some(val);
    loop {
      match self.queue.enqueue(val.take().unwrap()) {
        Ok(()) => return,
        Err(x) => val = Some(x),
      }
      self.next_dequeue_event = Some(Deferred::new());
      self.next_dequeue_event.as_mut().unwrap().await;
    }
  }
  pub async fn dequeue(&mut self) -> T {
    loop {
      match self.queue.dequeue() {
        Some(x) => return x,
        None => (),
      }
      self.next_enqueue_event = Some(Deferred::new());
      self.next_enqueue_event.as_mut().unwrap().await;
    }
  }
  pub fn peek(&self) -> Option<&T> {
    self.queue.peek()
  }
}

// #[test]
// fn foo() {
//   spin_on::spin_on(foo_inner());
// }

// #[cfg(test)]
// async fn foo_inner() {
//   let mut blah = AsyncQueue::<u32, 2>::new();
//   futures::join!(blah.enqueue(0xdeadbeef), blah.enqueue(0xfeedface), assert_eq!(blah.dequeue().await, 0xdeadbeef));
//   blah.enqueue(0xfeedface).await;
//   assert_eq!(blah.dequeue().await, 0xfeedface);
// }
