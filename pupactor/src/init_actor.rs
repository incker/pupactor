use crate::Actor;
use std::future::Future;

pub trait InitActor<Init>: Sized
where
    Self: Actor + Send + 'static,
    Init: Send + 'static,
{
    fn init_actor(init: Init) -> impl Future<Output=Option<Self>> + Send;
}

/// When Init == Act
impl<Act: Send> InitActor<Act> for Act
where
    Act: Actor + Send + 'static,
{
    #[inline(always)]
    async fn init_actor(init: Act) -> Option<Act> {
        Some(init)
    }
}

pub trait WithInitActor<Act>
where
    Act: Actor + 'static,
    Self: Send + 'static + Sized,
{
    fn init_actor(self) -> impl Future<Output=Option<Act>> + Send;
}

impl<Act, Init> WithInitActor<Act> for Init
where
    Act: Actor + InitActor<Init> + Send + 'static + Sized,
    Init: Send + 'static,
{
    #[inline(always)]
    fn init_actor(self) -> impl Future<Output=Option<Act>> + Send {
        <Act as InitActor<Init>>::init_actor(self)
    }
}
