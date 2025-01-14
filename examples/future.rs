#![allow(unused)]

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

struct RandFuture;

impl Future for RandFuture {
  type Output = u16;

  fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
    Poll::Ready(rand::random())
  }
}

/// Time Wrapper
struct TimeWrapper<Fut: Future> {
  start: Option<Instant>,
  future: Fut,
}

impl<Fut: Future> TimeWrapper<Fut> {
  fn new(future: Fut) -> Self {
    Self {
      start: None,
      future,
    }
  }
}

impl<Fut: Future> Future for TimeWrapper<Fut> {
  type Output = (Fut::Output, Duration);

  fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
    unsafe {
      let instance = self.get_unchecked_mut();
      instance.start.get_or_insert_with(Instant::now);

      match Pin::new_unchecked(&mut instance.future).poll(cx) {
        Poll::Pending => Poll::Pending,
        Poll::Ready(output) => {
          let duration = instance.start.unwrap().elapsed();
          Poll::Ready((output, duration))
        }
      }
    }
  }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let my_fut = async {
    tokio::time::sleep(Duration::from_secs(1)).await;
    42
  };

  let timed_wrapper = TimeWrapper::new(my_fut);
  let (result, duration) = timed_wrapper.await;

  println!("Result: {result}, Duration: {duration:?}");

  Ok(())
}
