use crate::Actor;
use std::convert::Infallible;
use std::future::Future;

pub trait StopActor<ShutDown>
where
    Self: Actor,
{
    fn stop_actor(self, shut_down: ShutDown) -> impl Future<Output=()> + Send;
}

pub trait WithStopActor<Act: Actor> {
    fn stop_actor(self, actor: Act) -> impl Future<Output=()> + Send;
}

impl<Act, ShutDown> WithStopActor<Act> for ShutDown
where
    Act: Actor + StopActor<ShutDown>,
{
    fn stop_actor(self, actor: Act) -> impl Future<Output=()> + Send {
        actor.stop_actor(self)
    }
}

impl<Act> StopActor<Infallible> for Act
where
    Act: Actor + Send,
{
    async fn stop_actor(self, _: Infallible) {
        unreachable!()
    }
}
