use crate::{Break, WithApplyCmd, WithInitActor};
use std::future::Future;
use std::marker::PhantomData;
use tokio::task::JoinHandle;

pub struct ActorWrap<Act, ActorVariants>(Act, PhantomData<ActorVariants>);

impl<Act, ActorVariants> ActorWrap<Act, ActorVariants>
where
    Act: Actor<States=ActorVariants>,
    Break: WithApplyCmd<Act, ActorVariants>,
    Act::Cmd: WithApplyCmd<Act, ActorVariants>,
{
    async fn infinite_loop(self) -> Option<ActorVariants> {
        let mut actor = self.0;
        let cmd: Result<Break, Act::Cmd> = actor.infinite_loop().await;
        match cmd {
            Ok(cmd) => cmd.change_state(actor).await,
            Err(cmd) => cmd.change_state(actor).await,
        }
    }
}


pub trait Actor
where
    Self: Sized + Send + Sync + 'static,
{
    type States;
    type Cmd: Send + Sync + 'static + WithApplyCmd<Self, Self::States>;

    fn infinite_loop(&mut self) -> impl Future<Output=Result<Break, Self::Cmd>> + Send;
}

#[cfg(not(tokio_unstable))]
#[inline(always)]
pub fn run_named_actor<Act>(init_data: impl WithInitActor<Act>, _: &str) -> JoinHandle<()>
where
    Act: Actor<States=Act>,
    Break: WithApplyCmd<Act>,
    Act::Cmd: WithApplyCmd<Act>,
{
    run_actor(init_data)
}

#[cfg(tokio_unstable)]
pub fn run_named_actor<Act>(init_data: impl WithInitActor<Act>, name: &str) -> JoinHandle<()>
where
    Act: Actor<States=Act>,
    Break: WithApplyCmd<Act>,
    Act::Cmd: WithApplyCmd<Act>,
{
    let fut = run_actor_internal::<Act>(init_data);
    tokio::task::Builder::new().name(name).spawn(fut).unwrap()
}

pub fn run_actor<Act>(init_data: impl WithInitActor<Act>) -> JoinHandle<()>
where
    Act: Actor<States=Act>,
    Break: WithApplyCmd<Act>,
    Act::Cmd: WithApplyCmd<Act>,
{
    let fut = run_actor_internal::<Act>(init_data);
    tokio::spawn(fut)
}

async fn run_actor_internal<Act>(init_data: impl WithInitActor<Act>)
where
    Act: Actor<States=Act>,
    Break: WithApplyCmd<Act>,
    Act::Cmd: WithApplyCmd<Act>,
{
    let mut opt_actor = init_data.init_actor().await;
    while let Some(mut actor) = opt_actor {
        let cmd: Result<Break, Act::Cmd> = actor.infinite_loop().await;
        opt_actor = match cmd {
            Ok(cmd) => cmd.change_state(actor).await,
            Err(cmd) => cmd.change_state(actor).await,
        };
    }
}
