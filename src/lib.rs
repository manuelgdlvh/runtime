use std::{
    pin::Pin,
    thread::{self},
};

use crate::mpsc::{Mpsc, MpscSenderOf, Receiver, Sender};

#[cfg(feature = "tokio")]
pub mod tokio;

pub mod mpsc;

type Callback = Pin<Box<dyn Future<Output = ()> + Send>>;

type JoinHandleOf<R, T> = <R as Runtime>::JoinHandle<T>;

pub enum JoinError {
    Panicked,
    Cancelled,
}

pub trait JoinHandle<T>: Send {
    fn cancel(&self);
    fn is_finished(&self) -> bool;
    fn join(&mut self) -> impl Future<Output = Result<T, JoinError>>;
}

pub trait Runtime: 'static {
    type JoinHandle<T: Send>: JoinHandle<T>;
    type Mpsc: Mpsc;

    fn new(threads: usize) -> Self;

    fn spawn<F>(fut: F) -> Self::JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static;

    fn spawn_local<F>(fut: F) -> Self::JoinHandle<F::Output>
    where
        F: Future + 'static,
        F::Output: Send + 'static;

    fn block_on<Fut>(&self, fut: Fut) -> Fut::Output
    where
        Fut: Future;

    fn defer(threads: usize, capacity: usize) -> Handle<Self>
    where
        Self: Sized,
    {
        let (tx, mut rx) = Self::Mpsc::channel::<Callback>(capacity);
        thread::spawn(move || {
            let runtime = Self::new(threads);
            runtime.block_on(async {
                while let Ok(callback) = rx.recv().await {
                    callback.await;
                }
            });
        });

        Handle::new(tx)
    }
}

pub struct Handle<R>
where
    R: Runtime,
{
    tx: MpscSenderOf<R, Callback>,
}

impl<R> Handle<R>
where
    R: Runtime,
{
    fn new(tx: MpscSenderOf<R, Callback>) -> Self {
        Self { tx }
    }

    async fn send<F>(&self, f: F) -> JoinHandleOf<R, F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        let (tx, mut rx) = R::Mpsc::channel::<JoinHandleOf<R, F::Output>>(1);
        let callback: Callback = Box::pin(async move {
            let handle = R::spawn(f);
            if let Err(err) = tx.send(handle).await {
                match err {
                    mpsc::TrySendError::Full(_) => todo!(),
                    mpsc::TrySendError::Closed(_) => todo!(),
                }
            }
        });

        self.tx
            .send(callback)
            .await
            .expect("lifetime of runtime depends on handle");

        rx.recv()
            .await
            .expect("lifetime of runtime depends on handle")
    }
}
