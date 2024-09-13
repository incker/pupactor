use crate::{Break, WithInitActor, WithStopActor};
use std::future::Future;
use tokio::task::JoinHandle;

pub trait Actor
where
    Self: Sized + Send + Sync + 'static,
{
    type ShutDown: Send + Sync + 'static + WithStopActor<Self>;

    fn infinite_loop(&mut self) -> impl Future<Output=Result<Break, Self::ShutDown>> + Send;
}

#[cfg(not(tokio_unstable))]
#[inline(always)]
pub fn run_named_actor<Act>(init_data: impl WithInitActor<Act>, _: &str) -> JoinHandle<()>
where
    Act: Actor,
    Break: WithStopActor<Act>,
    Act::ShutDown: WithStopActor<Act>,
{
    run_actor(init_data)
}

#[cfg(tokio_unstable)]
pub fn run_named_actor<Act>(init_data: impl WithInitActor<Act>, name: &str) -> JoinHandle<()>
where
    Act: Actor,
    Break: WithStopActor<Act>,
    Act::ShutDown: WithStopActor<Act>,
{
    let fut = run_actor_internal::<Act>(init_data);
    tokio::task::Builder::new().name(name).spawn(fut).unwrap()
}

pub fn run_actor<Act>(init_data: impl WithInitActor<Act>) -> JoinHandle<()>
where
    Act: Actor,
    Break: WithStopActor<Act>,
    Act::ShutDown: WithStopActor<Act>,
{
    let fut = run_actor_internal::<Act>(init_data);
    tokio::spawn(fut)
}

async fn run_actor_internal<Act>(init_data: impl WithInitActor<Act>)
where
    Act: Actor,
    Break: WithStopActor<Act>,
    Act::ShutDown: WithStopActor<Act>,
{
    if let Some(mut actor) = init_data.init_actor().await {
        let shutdown: Result<Break, Act::ShutDown> = actor.infinite_loop().await;
        match shutdown {
            Ok(shutdown) => shutdown.stop_actor(actor).await,
            Err(shutdown) => shutdown.stop_actor(actor).await,
        }
    }
}
