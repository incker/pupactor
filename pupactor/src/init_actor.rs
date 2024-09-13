use crate::Actor;
use std::future::Future;

pub trait InitActor<Init>: Sized
where
    Self: Actor + Send + Sync + 'static,
    Init: Send + Sync + 'static,
{
    fn init_actor(init: Init) -> impl Future<Output=Self> + Send;
}

/// When Init == Act
// impl<Act: Send> InitActor<Act> for Act
// where
//     Act: Actor + Send + Sync + 'static,
// {
//     #[inline(always)]
//     async fn init_actor(init: Act) -> Act {
//         init
//     }
// }

pub trait WithInitActor<Act>
where
    Act: Actor + Send + Sync + 'static,
    Self: Send + Sync + 'static + Sized,
{
    fn init_actor(self) -> impl Future<Output=Act> + Send;
}


impl<Act, Init> WithInitActor<Act> for Init
where
    Act: Actor + InitActor<Init> + Send + Sync + 'static + Sized,
    Init: Send + Sync + 'static,
{
    #[inline(always)]
    fn init_actor(self) -> impl Future<Output=Act> + Send {
        <Act as InitActor<Init>>::init_actor(self)
    }
}
