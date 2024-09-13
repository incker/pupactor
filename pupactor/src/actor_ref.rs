use std::convert::Infallible;
use std::future::Future;
use std::pin::Pin;

use std::task::{Context, Poll};
use tokio::sync::mpsc::error::SendError;
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::oneshot;

pub struct ActorRef<Msg, Shutdown = Infallible> {
    inner: UnboundedSender<ActorMsg<Msg, Shutdown>>,
}


pub enum ActorMsg<Msg, Shutdown = Infallible> {
    Msg(Msg),
    Shutdown(Shutdown),
}

impl<Msg, Shutdown> From<Msg> for ActorMsg<Msg, Shutdown> {
    #[inline(always)]
    fn from(msg: Msg) -> Self {
        ActorMsg::Msg(msg)
    }
}


impl<Msg, Shutdown> ActorRef<Msg, Shutdown> {
    #[inline]
    pub fn new(inner: UnboundedSender<ActorMsg<Msg, Shutdown>>) -> Self {
        Self { inner }
    }

    #[inline]
    pub fn try_send<M>(&self, msg: M) -> Result<(), SendError<ActorMsg<Msg, Shutdown>>>
    where
        Msg: From<M>,
    {
        self.inner.send(ActorMsg::Msg(msg.into()))
    }

    #[inline]
    pub fn send<M>(&self, msg: M)
    where
        Msg: From<M>,
    {
        let _ = self.inner.send(ActorMsg::Msg(msg.into()));
    }

    pub fn ask<Resp>(&self) -> oneshot::Receiver<Resp>
    where
        Msg: From<oneshot::Sender<Resp>>,
    {
        let (tx, rx) = oneshot::channel::<Resp>();
        let _ = self.inner.send(ActorMsg::Msg(tx.into()));
        rx
    }

    pub fn ask_or_default<Resp>(&self) -> PendingRespOrDefault<Resp>
    where
        Msg: From<oneshot::Sender<Resp>>,
        Resp: Default,
    {
        PendingRespOrDefault(self.ask())
    }

    pub fn try_shutdown<Resp>(&self, msg: Shutdown) -> Result<(), SendError<ActorMsg<Msg, Shutdown>>>
    where
        Msg: From<oneshot::Sender<Resp>>,
        Resp: Default,
    {
        self.inner.send(ActorMsg::Shutdown(msg))
    }

    pub fn shutdown<Resp>(&self, msg: Shutdown)
    where
        Msg: From<oneshot::Sender<Resp>>,
        Resp: Default,
    {
        let _ = self.inner.send(ActorMsg::Shutdown(msg));
    }
}


pub struct PendingRespOrDefault<T>(oneshot::Receiver<T>);

impl<T> Future for PendingRespOrDefault<T>
where
    T: Default,
{
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let pin = Pin::new(&mut Pin::get_mut(self).0);
        match Future::poll(pin, cx) {
            Poll::Ready(res) => {
                match res {
                    Ok(res) => Poll::Ready(res),
                    Err(_) => Poll::Ready(T::default()),
                }
            }
            Poll::Pending => Poll::Pending
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(1, size_of::<Result<(), SendError<()>>>());
    }
}
