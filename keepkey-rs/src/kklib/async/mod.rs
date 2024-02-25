pub mod deferred;
pub mod executor;
pub mod waker_chain;

use deferred::Deferred;

/*trait Generator<T, TNext> {
  type Future: Future<Output = Option<T>>;
  fn next(&mut self, x: TNext) -> Self::Future;
}

trait GeneratorContext<T, TNext> {
  type Future: Future<Output = TNext>;
  fn r#yield(&mut self, x: T) -> Self::Future;
}

struct Foo<T, TNext> {
  ctx_to_gen: heapless::spsc::Queue<T, 2>
  gen_to_ctx: heapless::spsc::Queue<TNext, 2>,
}

impl<T, TNext> GeneratorContext<T, TNext> for Foo<T, TNext>
{
  type Future = ;
  fn r#yield(&mut self, x: T) -> Self::Future {
    self.ctx_to_gen.enqueue(x).unwrap();
    self.gen_to_ctx.dequeue()
  }
}

impl<T, TNext> Generator<T, TNext> for Foo<T, TNext>
{
  type Future = ;
  fn next(&mut self, x: TNext) -> Self::Future {
    self.gen_to_ctx.enqueue(x).unwrap();
    self.ctx_to_gen.dequeue()
  }
}

enum FooGeneratorState<T> {
  BeforeFirstYield,
  Yielded(T),
  Complete(T),
}*/

// ---

pub struct AsyncQueue<T, const N: usize> {
  queue: heapless::spsc::Queue<T, N>,
  next_enqueue_event: Option<Deferred<()>>,
  next_dequeue_event: Option<Deferred<()>>,
}

impl<'a, T, const N: usize> AsyncQueue<T, N> {
  pub const fn new() -> Self {
    let mut out = AsyncQueue {
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

#[test]
fn foo() {
  spin_on::spin_on(foo_inner());
}

#[cfg(test)]
async fn foo_inner() {
  let mut blah = AsyncQueue::<u32, 2>::new();
  blah.enqueue(0xdeadbeef).await;
  assert_eq!(blah.dequeue().await, 0xdeadbeef);
}
