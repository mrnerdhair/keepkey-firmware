use core::{
  future::Future,
  mem::take,
  pin::Pin,
  task::{Context, Poll, Waker},
};
use futures::prelude;
use super::deferred::Deferred;

// struct USBEndpoint {
//   next_packet: Deferred<heapless::Vec<u8, 64>>,
// }

// impl USBEndpoint {
//   fn gotAThing(&mut self, x: heapless::Vec<u8, 64>) {
//     core::mem::take(&mut self.next_packet).resolve(x);
//   }
// }

// impl USBThing {
//   async fn yield(&mut self, x: heapless::String<128>) {
//     self.response = Some(x)
    
//   }
//   async fn return(&mut self, x: heapless::String<128>) -> {
//     self.response = Some(x)
//   }
// }

// async fn foo(): heapless::String<128> {
//   println!("got foo, asking for bar");
//   let bar: &str = yield(heapless::String::<128>::from_str("plz gib bar")).await;

//   println!("got bar");
//   let baz: heapless::String::<128>::from_str(bar);
//   baz.push_str("baz");

//   baz
// }
