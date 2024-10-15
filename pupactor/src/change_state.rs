use crate::Actor;
use std::convert::Infallible;
use std::future::Future;

pub trait ApplyCmd<Cmd>
where
    Self: Actor,
{
    fn change_state(self, cmd: Cmd) -> impl Future<Output=Option<Self::States>> + Send;
}

pub trait WithApplyCmd<Akt, States = Akt> {
    fn change_state(self, actor: Akt) -> impl Future<Output=Option<States>> + Send;
}

impl<Act, States, Cmd> WithApplyCmd<Act, States> for Cmd
where
    Act: Actor<States=States> + ApplyCmd<Cmd>,
{
    fn change_state(self, actor: Act) -> impl Future<Output=Option<States>> + Send {
        actor.change_state(self)
    }
}

impl<Act> ApplyCmd<Infallible> for Act
where
    Act: Actor + Send,
{
    #[inline(always)]
    async fn change_state(self, _: Infallible) -> Option<Act::States> {
        // this code is actually unreachable
        None
    }
}
