use std::ops::Add;
use std::time::{Duration, Instant};

use runtime::Runtime;
use runtime::time::{Time, TimedOut};

pub fn sleep_test_when_invoked_then_wait<R: Runtime>() {
    R::new(1).block_on(async move {
        let now = Instant::now();
        let duration = Duration::from_millis(250);
        R::Time::sleep(duration).await;

        assert!(Instant::now() >= now.add(duration))
    });
}

pub fn timeout_test_when_future_returns_earlier_then_returns_result<R: Runtime>() {
    R::new(1).block_on(async move {
        let result =
            R::Time::timeout(Duration::from_millis(250), async move { "hello world!" }).await;
        assert!(matches!(result, Ok("hello world!")));
    });
}

pub fn timeout_test_when_sleep_returns_earlier_then_returns_error<R: Runtime>() {
    R::new(1).block_on(async move {
        let result = R::Time::timeout(Duration::from_millis(125), async move {
            R::Time::sleep(Duration::from_millis(250)).await;
            "hello world!"
        })
        .await;
        assert!(matches!(result, Err(TimedOut {})));
    });
}
