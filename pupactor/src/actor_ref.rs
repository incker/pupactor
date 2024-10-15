use std::convert::Infallible;
use std::future::Future;
use std::pin::Pin;

use std::task::{Context, Poll};
use tokio::sync::mpsc::error::SendError;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender, WeakUnboundedSender};
use tokio::sync::{mpsc, oneshot};

pub fn actor_channel<Msg, Command>() -> (
    ActorRef<Msg, Command>,
    UnboundedReceiver<ActorMsg<Msg, Command>>,
) {
    let (sender, receiver) = mpsc::unbounded_channel();
    (ActorRef::new(sender), receiver)
}

pub struct ActorRef<Msg, Command = Infallible> {
    inner: UnboundedSender<ActorMsg<Msg, Command>>,
}

impl<Msg, Command> Clone for ActorRef<Msg, Command> {
    fn clone(&self) -> Self {
        Self::new(self.inner.clone())
    }
}

pub enum ActorMsg<Msg, Command = Infallible> {
    Msg(Msg),
    Shutdown(Command),
}

impl<Msg, Command> From<Msg> for ActorMsg<Msg, Command> {
    #[inline(always)]
    fn from(msg: Msg) -> Self {
        ActorMsg::Msg(msg)
    }
}

impl<Msg, Command> ActorRef<Msg, Command> {
    #[inline]
    pub fn new(inner: UnboundedSender<ActorMsg<Msg, Command>>) -> Self {
        Self { inner }
    }

    #[inline]
    pub fn try_send<M>(&self, msg: M) -> Result<(), SendError<ActorMsg<Msg, Command>>>
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

    pub fn try_command<Resp>(
        &self,
        msg: Command,
    ) -> Result<(), SendError<ActorMsg<Msg, Command>>>
    where
        Msg: From<oneshot::Sender<Resp>>,
        Resp: Default,
    {
        self.inner.send(ActorMsg::Shutdown(msg))
    }

    pub fn command<IntoCommand>(&self, shutdown: IntoCommand)
    where
        Command: From<IntoCommand>,
    {
        let _ = self.inner.send(ActorMsg::Shutdown(shutdown.into()));
    }

    #[inline]
    pub fn downgrade(&self) -> WeakActorRef<Msg, Command> {
        WeakActorRef {
            inner: self.inner.downgrade(),
        }
    }

    #[inline]
    pub fn strong_count(&self) -> usize {
        self.inner.strong_count()
    }

    #[inline]
    pub fn weak_count(&self) -> usize {
        self.inner.weak_count()
    }

    #[inline]
    pub fn is_closed(&self) -> bool {
        self.inner.is_closed()
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
            Poll::Ready(res) => match res {
                Ok(res) => Poll::Ready(res),
                Err(_) => Poll::Ready(T::default()),
            },
            Poll::Pending => Poll::Pending,
        }
    }
}

impl<T> PendingRespOrDefault<T>
where
    T: Default,
{
    pub fn blocking_recv(self) -> T {
        self.0.blocking_recv().unwrap_or_default()
    }
}

pub struct WeakActorRef<Msg, Shutdown = Infallible> {
    inner: WeakUnboundedSender<ActorMsg<Msg, Shutdown>>,
}

impl<Msg, Shutdown> WeakActorRef<Msg, Shutdown> {
    #[inline]
    pub fn upgrade(&self) -> Option<ActorRef<Msg, Shutdown>> {
        self.inner.upgrade().map(ActorRef::new)
    }

    #[inline]
    pub fn strong_count(&self) -> usize {
        self.inner.strong_count()
    }

    #[inline]
    pub fn weak_count(&self) -> usize {
        self.inner.weak_count()
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
