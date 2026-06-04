use std::{
    pin::{Pin, pin},
    thread::{self},
};

use futures::FutureExt;

use crate::{
    mpsc::{Mpsc, MpscSenderOf, Receiver as MpscReceiver, Sender as MpscSender},
    oneshot::{
        Oneshot, OneshotReceiverOf, Receiver as OneshotReceiver, Sender as OneshotSender,
        TryRecvError,
    },
    tcp::Tcp,
};

#[cfg(feature = "tokio")]
pub mod tokio;

pub mod mpsc;
pub mod oneshot;
pub mod tcp;

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
    type Oneshot: Oneshot;
    type Tcp: Tcp;

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

    fn defer<Fut>(threads: usize, capacity: usize, fut: Fut) -> Handle<Self, Fut>
    where
        Self: Sized,
        Fut: Future + Send + 'static,
        Fut::Output: Send,
    {
        let (tx, mut rx) = Self::Mpsc::channel::<Callback>(capacity);
        let (tx_fut, rx_fut) = Self::Oneshot::channel::<Option<Fut::Output>>();
        thread::spawn(move || {
            let runtime = Self::new(threads);
            runtime.block_on(async move {
                let mut fut = pin!(fut.fuse());
                let output = loop {
                    futures::select! {
                        callback = rx.recv().fuse() => {
                            match callback {
                                Ok(callback) => {
                                    callback.await;
                                }
                                // Handle dropped
                                Err(_) => {
                                    break None;
                                }
                            }
                        }
                       result = fut => {
                           // Block on future returned
                           break Some(result);
                        }
                    }
                };

                if let Err(_err) = tx_fut.send(output) {}
            });
        });

        Handle::new(tx, rx_fut)
    }
}

pub struct Handle<R, Fut>
where
    Fut: Future,
    Fut::Output: Send,
    R: Runtime,
{
    tx: MpscSenderOf<R, Callback>,
    rx: OneshotReceiverOf<R, Option<Fut::Output>>,
}

impl<R, Fut> Handle<R, Fut>
where
    Fut: Future,
    Fut::Output: Send,
    R: Runtime,
{
    fn new(tx: MpscSenderOf<R, Callback>, rx: OneshotReceiverOf<R, Option<Fut::Output>>) -> Self {
        Self { tx, rx }
    }

    async fn send<F>(&self, f: F) -> JoinHandleOf<R, F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send,
    {
        let (tx, mut rx) = R::Oneshot::channel::<JoinHandleOf<R, F::Output>>();
        let callback: Callback = Box::pin(async move {
            let handle = R::spawn(f);
            if let Err(_) = tx.send(handle) {}
        });

        self.tx
            .send(callback)
            .await
            .expect("lifetime of runtime depends on handle");

        rx.recv()
            .await
            .expect("lifetime of runtime depends on handle")
    }

    fn join(&mut self) -> impl Future<Output = Result<Fut::Output, TryRecvError>> {
        self.rx.recv().map(|res| match res {
            Ok(Some(val)) => Ok(val),
            Ok(None) => Err(TryRecvError::Empty),
            Err(err) => Err(err),
        })
    }
}
