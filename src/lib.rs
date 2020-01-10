//! A simple future that reports the duration of its inner future.

// use futures::{Async, Future, Poll};

use std::{
    future::Future,
    time::{Duration, Instant},
    task::{Poll, Context},
    pin::Pin,
};
use futures::ready;
use pin_project::pin_project;

#[pin_project]
pub struct Stopwatch<F> {
    start_time: Instant,
    #[pin]
    inner: F,
}
impl<F> Stopwatch<F> {
    pub fn new(inner: F) -> Self {
        Stopwatch {
            start_time: Instant::now(),
            inner,
        }
    }
}
impl<F: Future> Future for Stopwatch<F> {
    type Output = (F::Output, Duration);

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let this = self.project();
        let x = ready!(this.inner.poll(cx));
        Poll::Ready((x, this.start_time.elapsed()))
    }
}
#[cfg(test)]
#[test]
fn timer_future() {
    use std::time::Duration;
    use tokio::time::delay_for;
    let mut runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async move {
        let ((), time) = Stopwatch::new(delay_for(Duration::from_secs(2))).await;
        println!("Timer duration: {:?}", time);
        assert!(time >= Duration::from_secs(2));
    });
}
