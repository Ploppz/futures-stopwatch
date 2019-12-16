//! A simple future that reports the duration of its inner future.

use futures::{Async, Future, Poll};
use std::time::{Duration, Instant};

pub struct Stopwatch<F> {
    start_time: Instant,
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
impl<F> Future for Stopwatch<F>
where
    F: Future,
{
    type Item = (F::Item, Duration);
    type Error = F::Error;
    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let poll = self.inner.poll();
        match poll {
            Ok(Async::Ready(x)) => Ok(Async::Ready((x, self.start_time.elapsed()))),
            Ok(Async::NotReady) => Ok(Async::NotReady),
            Err(e) => Err(e),
        }
    }
}
#[cfg(test)]
#[test]
fn timer_future() {
    use std::time::{Duration, Instant};
    use tokio::timer::Delay;
    let future = Stopwatch::new(Delay::new(Instant::now() + Duration::from_secs(2)));
    let mut runtime = tokio::runtime::Runtime::new().unwrap();
    let ((), time) = runtime.block_on(future).unwrap();
    println!("Timer duration: {:?}", time);
    assert!(time >= Duration::from_secs(2));
}
