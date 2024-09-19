use crate::Actor;
use std::convert::Infallible;
use std::future::Future;

pub trait ApplyCmd<Cmd>
where
    Self: Actor,
{
    fn apply_cmd(self, cmd: Cmd) -> impl Future<Output=Option<Self>> + Send;
}

pub trait WithApplyCmd<Act: Actor> {
    fn apply_cmd(self, actor: Act) -> impl Future<Output=Option<Act>> + Send;
}

impl<Act, Cmd> WithApplyCmd<Act> for Cmd
where
    Act: Actor + ApplyCmd<Cmd>,
{
    #[inline(always)]
    fn apply_cmd(self, actor: Act) -> impl Future<Output=Option<Act>> + Send {
        actor.apply_cmd(self)
    }
}

impl<Act> ApplyCmd<Infallible> for Act
where
    Act: Actor + Send,
{
    #[inline(always)]
    async fn apply_cmd(self, _: Infallible) -> Option<Act> {
        None
    }
}
