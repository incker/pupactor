mod tcp_listener;
mod sleep;
mod unbounded_receiver;
mod interval;

pub use sleep::*;

use crate::ActorMsg;
use std::convert::Infallible;
use std::future::Future;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

pub trait ActorListener<Msg, Command = Infallible> {
    fn next_msg(&mut self) -> impl Future<Output=Option<ActorMsg<Msg, Command>>>;
}

pub struct Listener<T, Msg, Command = Infallible>(T, PhantomData<(Msg, Command)>)
where
    T: ActorListener<Msg, Command>;

impl<T, Msg, Command> Deref for Listener<T, Msg, Command>
where
    T: ActorListener<Msg, Command>,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, Msg, Command> DerefMut for Listener<T, Msg, Command>
where
    T: ActorListener<Msg, Command>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T, Msg, Command> Listener<T, Msg, Command>
where
    T: ActorListener<Msg, Command>,
{
    #[inline(always)]
    pub fn new(listener: T) -> Self {
        Listener(listener, PhantomData)
    }

    #[inline(always)]
    pub async fn next_msg(&mut self) -> Option<ActorMsg<Msg, Command>> {
        self.0.next_msg().await
    }
}

