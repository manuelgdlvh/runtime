use futures::FutureExt;
use tokio::runtime::LocalOptions;

use crate::{JoinError, JoinHandle, Runtime, tokio::mpsc::TokioMpsc};

pub mod mpsc;

pub enum Tokio {
    SingleThreaded { rt: tokio::runtime::LocalRuntime },
    MultiThreaded { rt: tokio::runtime::Runtime },
}

impl<T: Send> JoinHandle<T> for tokio::task::JoinHandle<T> {
    fn cancel(&self) {
        self.abort();
    }

    fn is_finished(&self) -> bool {
        self.is_finished()
    }

    fn join(&mut self) -> impl Future<Output = Result<T, JoinError>> {
        self.map(|res| match res {
            Ok(val) => Ok(val),
            Err(err) if err.is_panic() => Err(JoinError::Panicked),
            Err(_) => Err(JoinError::Cancelled),
        })
    }
}

impl Runtime for Tokio {
    type JoinHandle<T: Send> = tokio::task::JoinHandle<T>;
    type Mpsc = TokioMpsc;

    fn new(threads: usize) -> Self {
        if threads > 1 {
            Tokio::MultiThreaded {
                rt: tokio::runtime::Builder::new_multi_thread()
                    .enable_all()
                    .build()
                    .unwrap(),
            }
        } else {
            Tokio::SingleThreaded {
                rt: tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build_local(LocalOptions::default())
                    .unwrap(),
            }
        }
    }

    fn spawn<F>(fut: F) -> Self::JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        tokio::spawn(fut)
    }

    fn block_on<Fut>(&self, fut: Fut) -> Fut::Output
    where
        Fut: Future,
    {
        match self {
            Tokio::SingleThreaded { rt } => rt.block_on(fut),
            Tokio::MultiThreaded { rt } => rt.block_on(fut),
        }
    }

    fn spawn_local<F>(fut: F) -> Self::JoinHandle<F::Output>
    where
        F: Future + 'static,
        F::Output: Send + 'static,
    {
        tokio::task::spawn_local(fut)
    }
}

#[cfg(test)]
mod test {

    use crate::{Runtime, tokio::Tokio};

    fn async_test<F: Future>(threads: usize, f: F) -> F::Output {
        Tokio::new(threads).block_on(f)
    }

    #[test]
    fn test_block_on_when_finish_then_returns_output() {
        let result = async_test(1, async move { "hello world!" });
        assert_eq!(result, "hello world!");
    }

    #[test]
    fn test_new_when_one_thread_then_returns_single_threaded() {
        assert!(matches!(Tokio::new(1), Tokio::SingleThreaded { .. }));
    }

    #[test]
    fn test_new_when_higher_than_one_thread_then_returns_single_threaded() {
        assert!(matches!(Tokio::new(2), Tokio::MultiThreaded { .. }));
    }

    #[test]
    fn test_defer_when_send_future_then_receive_result() {
        async_test(1, async move {
            let handle = Tokio::defer(1, 1024);
            let result = handle.send(async move { "hello world!" }).await;
            assert!(matches!(result.await, Ok("hello world!")));
        })
    }

    #[test]
    #[should_panic]
    fn test_spawn_local_when_multi_threaded_then_panic() {
        async_test(2, async move { Tokio::spawn_local(async {}) });
    }

    #[test]
    fn test_spawn_local_when_single_threaded_then_returns_result() {
        async_test(1, async move {
            let result = Tokio::spawn_local(async move { "hello world!" }).await;
            assert!(matches!(result, Ok("hello world!")))
        });
    }
}
