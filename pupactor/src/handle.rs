use crate::{Actor, ActorCmdRes};
use std::convert::Infallible;
use std::future::Future;
pub trait AsyncHandle<T>
where
    Self: Actor + Send + 'static,
    T: Send + 'static,
{
    fn async_handle(
        &mut self,
        value: T,
    ) -> impl Future<Output=impl Into<ActorCmdRes<Self::ShutDown>>> + Send;
}

pub trait Handle<T>
where
    Self: Actor,
{
    fn handle(&mut self, value: T) -> impl Into<ActorCmdRes<Self::ShutDown>>;
}

impl<T, Act> AsyncHandle<T> for Act
where
    Self: Actor + Handle<T> + Send + 'static,
    T: Send + 'static,
{
    #[inline(always)]
    fn async_handle(
        &mut self,
        value: T,
    ) -> impl Future<Output=impl Into<ActorCmdRes<Self::ShutDown>>> + Send {
        async { self.handle(value) }
    }
}

pub trait WithHandle<Act>
where
    Self: Send + 'static,
    Act: Actor + Send + 'static,
{
    fn with_handle(
        self,
        actor: &mut Act,
    ) -> impl Future<Output=impl Into<ActorCmdRes<Act::ShutDown>>> + Send;
}

impl<Act, T> WithHandle<Act> for T
where
    Act: Actor + Send + 'static + AsyncHandle<T>,
    T: Send + 'static,
{
    #[inline(always)]
    fn with_handle(
        self,
        actor: &mut Act,
    ) -> impl Future<Output=impl Into<ActorCmdRes<Act::ShutDown>>> + Send {
        actor.async_handle(self)
    }
}

/// Some Listeners have no msg, only shutdown
impl<Act> Handle<Infallible> for Act
where
    Self: Actor + Send + 'static,
{
    #[inline(always)]
    fn handle(&mut self, _: Infallible) -> impl Into<ActorCmdRes<Self::ShutDown>> {
        // unreachable!()
    }
}
