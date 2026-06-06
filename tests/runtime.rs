use std::time::Duration;

use runtime::{JoinHandle, Runtime, oneshot};

use runtime::time::Time;

pub fn test_block_on_when_finish_then_returns_output<R: Runtime>() {
    let result = R::new(1).block_on(async move { "hello world!" });
    assert_eq!(result, "hello world!");
}

pub fn test_defer_when_spawn_then_receive_result<R: Runtime>() {
    R::new(1).block_on(async move {
        let handle = R::defer(1, 1024, async move {
            R::Time::sleep(Duration::from_millis(250)).await;
        });
        let mut result = handle.spawn(async move { "hello world!" }).await.unwrap();
        assert!(matches!(result.join().await, Ok("hello world!")));
    })
}

pub fn test_defer_when_spawn_and_block_on_returns_then_disconnected_error<R: Runtime>() {
    R::new(1).block_on(async move {
        let handle = R::defer(1, 1024, async move {});

        R::Time::sleep(Duration::from_millis(250)).await;
        let result = handle.spawn(async move { "hello world!" }).await;

        assert!(matches!(result, Err(oneshot::TryRecvError::Disconnected)));
    })
}

pub fn test_defer_when_join_then_receive_future_output<R: Runtime>() {
    R::new(1).block_on(async move {
        let mut handle = R::defer(1, 1024, async move {
            R::Time::sleep(Duration::from_millis(250)).await;
            "hello world!"
        });
        assert!(matches!(handle.join().await, Ok("hello world!")));
    })
}

pub fn test_spawn_local_when_multi_threaded_then_panic<R: Runtime>() {
    R::new(2).block_on(async move { R::spawn_local(async {}) });
}

pub fn test_spawn_local_when_single_threaded_then_returns_result<R: Runtime>() {
    R::new(1).block_on(async move {
        let result = R::spawn_local(async move { "hello world!" }).join().await;
        assert!(matches!(result, Ok("hello world!")))
    });
}
