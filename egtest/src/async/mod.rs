pub mod deferred;
pub mod executor;
pub mod queue;
pub mod waker_chain;

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
