use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

use tokio::time::Instant;

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
    let start = self.start.get_or_insert_with(Instant::now);
    let inner_poll = self.future.poll(cx);
    let elapsed = self.elapsed();

    match inner_poll {
      Poll::Pending => Poll::Pending,
      Poll::Ready(output) => Poll::Ready((output, elapsed)),
    }
  }
}

fn main() {}
