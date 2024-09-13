use crate::ActorMsg;
use std::convert::Infallible;
use std::future::Future;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::time::{Instant, Interval};

pub trait ActorListener<Msg, Shutdown = Infallible> {
    fn next_msg(&mut self) -> impl Future<Output=Option<ActorMsg<Msg, Shutdown>>>;
}

pub struct Listener<T, Msg, Shutdown = Infallible>(T, PhantomData<(Msg, Shutdown)>)
where
    T: ActorListener<Msg, Shutdown>;

impl<T, Msg, Shutdown> Deref for Listener<T, Msg, Shutdown>
where
    T: ActorListener<Msg, Shutdown>,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, Msg, Shutdown> DerefMut for Listener<T, Msg, Shutdown>
where
    T: ActorListener<Msg, Shutdown>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T, Msg, Shutdown> Listener<T, Msg, Shutdown>
where
    T: ActorListener<Msg, Shutdown>,
{
    #[inline(always)]
    pub fn new(listener: T) -> Self {
        Listener(listener, PhantomData)
    }

    #[inline(always)]
    pub async fn next_msg(&mut self) -> Option<ActorMsg<Msg, Shutdown>> {
        self.0.next_msg().await
    }
}

/// Interval
impl<Resp> ActorListener<Resp> for Interval
where
    Resp: From<Instant>,
{
    #[inline(always)]
    async fn next_msg(&mut self) -> Option<ActorMsg<Resp>> {
        Some(ActorMsg::Msg(self.tick().await.into()))
    }
}

/// UnboundedReceiver
impl<Msg, Shutdown> ActorListener<Msg, Shutdown> for UnboundedReceiver<ActorMsg<Msg, Shutdown>> {
    #[inline(always)]
    async fn next_msg(&mut self) -> Option<ActorMsg<Msg, Shutdown>> {
        self.recv().await
    }
}
