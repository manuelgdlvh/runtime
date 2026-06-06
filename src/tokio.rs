use futures::FutureExt;
use tokio::runtime::LocalOptions;

use crate::{
    JoinError, JoinHandle, Runtime,
    tokio::{mpsc::TokioMpsc, oneshot::TokioOneshot, tcp::TokioTcp},
};

pub mod mpsc;
pub mod oneshot;
pub mod tcp;

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
    type Oneshot = TokioOneshot;
    type Tcp = TokioTcp;

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
