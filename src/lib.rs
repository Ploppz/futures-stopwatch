//! A simple future that reports the duration of its inner future.

use std::{
    future::Future,
    time::{Duration, Instant},
    task::{Poll, Context},
    pin::Pin,
};
use futures::{future::TryFuture, ready};
use pin_project::{pin_project, project};

pub async fn stopwatch<F: Future>(inner: F) -> (F::Output, Duration) {
    Stopwatch::new(inner).await
}
pub async fn try_stopwatch<F: TryFuture>(inner: F) -> Result<(F::Ok, Duration), F::Error> {
    let (result, duration) = Stopwatch::new(inner).await;
    result.map(|x| (x, duration))
}

#[pin_project]
pub struct Stopwatch<F> {
    start_time: Instant,
    #[pin]
    inner: F,
}
impl<F> Stopwatch<F> {
    fn new(inner: F) -> Self {
        Stopwatch {
            start_time: Instant::now(),
            inner,
        }
    }
}
impl<F: Future> Future for Stopwatch<F> {
    type Output = (F::Output, Duration);

    #[project]
    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let this = self.project();
        let x = ready!(this.inner.poll(cx));
        Poll::Ready((x, this.start_time.elapsed()))
    }
}
#[cfg(test)]
#[tokio::test]
async fn timer_future() {
    use std::time::Duration;
    use tokio::time::delay_for;
    let ((), time) = Stopwatch::new(delay_for(Duration::from_secs(2))).await;
    println!("Timer duration: {:?}", time);
    assert!(time >= Duration::from_secs(2));
}
