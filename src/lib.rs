//! A simple future that reports the duration of its inner future.

use std::{
    future::Future,
    time::{Duration, Instant},
    task::{Poll, Context},
    pin::Pin,
};
use futures::ready;
use pin_project::{pin_project, project};

pub async fn stopwatch<F: Future>(inner: F) -> (F::Output, Duration)
    where F: Future
{
    Stopwatch::new(inner).await
}
pub async fn try_stopwatch<F, T, E>(inner: F) -> Result<(T, Duration), E>
    where F: Future<Output=Result<T, E>>
{
    let (result, duration): (Result<T, E>, Duration) = Stopwatch::new(inner).await;
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
mod test {
    use super::*;
    use std::time::Duration;
    use tokio::time::delay_for;
    #[tokio::test]
    async fn test_stopwatch() {
        let ((), time) = stopwatch(delay_for(Duration::from_secs(2))).await;
        println!("Timer duration: {:?}", time);
        assert!(time >= Duration::from_secs(2));
    }

    #[tokio::test]
    async fn test_try_stopwatch() {
        let future = async {
            Result::<(), u8>::Ok(delay_for(Duration::from_secs(2)).await)
        };
        let ((), time) = try_stopwatch(future).await.unwrap();
        println!("Timer duration: {:?}", time);
        assert!(time >= Duration::from_secs(2));
    }
}
