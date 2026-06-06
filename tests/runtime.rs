use std::time::Duration;

use runtime::{JoinHandle, Runtime, oneshot};

use runtime::time::Time;

pub fn block_on_returns_completed_future_value<R: Runtime>() {
    let result = R::new(1).block_on(async move { "hello world!" });
    assert_eq!(result, "hello world!");
}

pub fn defer_spawn_returns_joined_task_result<R: Runtime>() {
    R::new(1).block_on(async move {
        let handle = R::defer(1, 1024, async move {
            R::Time::sleep(Duration::from_millis(250)).await;
        });
        let mut result = handle.spawn(async move { "hello world!" }).await.unwrap();
        assert!(matches!(result.join().await, Ok("hello world!")));
    })
}

pub fn defer_spawn_after_runtime_dropped_returns_disconnected<R: Runtime>() {
    R::new(1).block_on(async move {
        let handle = R::defer(1, 1024, async move {});

        R::Time::sleep(Duration::from_millis(250)).await;
        let result = handle.spawn(async move { "hello world!" }).await;

        assert!(matches!(result, Err(oneshot::TryRecvError::Disconnected)));
    })
}

pub fn defer_join_returns_deferred_future_output<R: Runtime>() {
    R::new(1).block_on(async move {
        let mut handle = R::defer(1, 1024, async move {
            R::Time::sleep(Duration::from_millis(250)).await;
            "hello world!"
        });
        assert!(matches!(handle.join().await, Ok("hello world!")));
    })
}

pub fn spawn_local_panics_on_multi_threaded_runtime<R: Runtime>() {
    R::new(2).block_on(async move { R::spawn_local(async {}) });
}

pub fn spawn_local_returns_result_on_single_threaded_runtime<R: Runtime>() {
    R::new(1).block_on(async move {
        let result = R::spawn_local(async move { "hello world!" }).join().await;
        assert!(matches!(result, Ok("hello world!")))
    });
}
