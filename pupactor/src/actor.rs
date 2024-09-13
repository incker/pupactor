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


pub fn run_actor<Act>(init_data: impl WithInitActor<Act>) -> JoinHandle<()>
where
    Act: Actor,
    Break: WithStopActor<Act>,
    Act::ShutDown: WithStopActor<Act>,
{
    tokio::spawn(async move {
        let mut actor: Act = init_data.init_actor().await;
        let shutdown: Result<Break, Act::ShutDown> = actor.infinite_loop().await;
        match shutdown {
            Ok(shutdown) => shutdown.stop_actor(actor).await,
            Err(shutdown) => shutdown.stop_actor(actor).await,
        }
    })
}
