use crate::{Break, WithApplyCmd, WithInitActor};
use std::future::Future;
use tokio::task::JoinHandle;

pub trait Actor
where
    Self: Sized + Send + 'static,
{
    type Cmd: Send + 'static;

    fn infinite_loop(&mut self) -> impl Future<Output = Result<Self::Cmd, Break>> + Send;
}

#[cfg(not(tokio_unstable))]
#[inline(always)]
pub fn run_named_actor<Act>(init_data: impl WithInitActor<Act>, _: &str) -> JoinHandle<()>
where
    Act: Actor,
    Break: WithApplyCmd<Act>,
    Act::Cmd: WithApplyCmd<Act>,
{
    run_actor(init_data)
}

#[cfg(tokio_unstable)]
pub fn run_named_actor<Act>(init_data: impl WithInitActor<Act>, name: &str) -> JoinHandle<()>
where
    Act: Actor,
    Break: WithApplyCmd<Act>,
    Act::Cmd: WithApplyCmd<Act>,
{
    let fut = run_actor_internal::<Act>(init_data);
    tokio::task::Builder::new().name(name).spawn(fut).unwrap()
}

pub fn run_actor<Act>(init_data: impl WithInitActor<Act>) -> JoinHandle<()>
where
    Act: Actor,
    Break: WithApplyCmd<Act>,
    Act::Cmd: WithApplyCmd<Act>,
{
    let fut = run_actor_internal::<Act>(init_data);
    tokio::spawn(fut)
}

async fn run_actor_internal<Act>(init_data: impl WithInitActor<Act>)
where
    Act: Actor,
    Break: WithApplyCmd<Act>,
    Act::Cmd: WithApplyCmd<Act>,
{
    let mut opt_actor = init_data.init_actor().await;

    while let Some(mut actor) = opt_actor {
        let cmd: Result<Act::Cmd, Break> = actor.infinite_loop().await;
        opt_actor = match cmd {
            Ok(cmd) => cmd.apply_cmd(actor).await,
            Err(cmd) => cmd.apply_cmd(actor).await,
        };
    }
}
