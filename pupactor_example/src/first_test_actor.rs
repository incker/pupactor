use pupactor::{
    run_actor, ActorMsg, ApplyCmd, AsyncHandle, Break, Cmd, Continue, Handle, InitActor, Listener,
};
use pupactor::{ActorMsgHandle, ActorShutdown, Pupactor};
use std::time::Instant;
use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::time::Interval;

#[derive(ActorMsgHandle)]
#[actor(kind = "MyFirstTestActor")]
pub enum Value {
    U32(u32),
    U64(u64),
    String(String),
}

#[derive(ActorShutdown)]
pub struct MyActorShutdown;

// generated
// impl AsyncHandle<Value> for FirstTestActor {
//     async fn async_handle(&mut self, value: Value) -> ActorCommand<Self::ShutDown> {
//         match value {
//             Value::U32(val) => self.async_handle(val).await.into(),
//             Value::U64(val) => self.async_handle(val).await.into(),
//             Value::String(val) => self.async_handle(val).await.into(),
//         }
//     }
// }

#[derive(Pupactor)]
#[actor(shutdown = "MyActorShutdown")]
struct MyFirstTestActor {
    some_data: bool,
    some_other_data: usize,
    #[listener]
    interval: Listener<Interval, Instant>,
    #[listener]
    interval2: Listener<Interval, Instant>,
    #[listener]
    channel: Listener<UnboundedReceiver<ActorMsg<Instant>>, Instant>,
}

impl InitActor<UnboundedReceiver<ActorMsg<Instant>>> for MyFirstTestActor {
    async fn init_actor(receiver: UnboundedReceiver<ActorMsg<Instant>>) -> Option<Self> {
        Some(MyFirstTestActor {
            some_data: true,
            some_other_data: 0,
            interval: Listener::new(tokio::time::interval(tokio::time::Duration::from_secs(1))),
            interval2: Listener::new(tokio::time::interval(tokio::time::Duration::from_secs(2))),
            channel: Listener::new(receiver),
        })
    }
}

pub async fn test_function() {
    let (_sender, receiver) = mpsc::unbounded_channel();

    let _ = run_actor::<MyFirstTestActor>(receiver).await;

    // actor.infinite_loop().await;
}

/*
impl Actor for FirstTestActor {
    type ShutDown = MyActorShutdown;

    async fn infinite_loop(&mut self) -> Result<Break, Self::ShutDown> {
        loop {
            select! {
                msg = Listener::next_msg(&mut self.interval) => {
                    if let Some(msg) = msg {
                        match msg {
                            ActorMsg::Msg(msg) => {
                                let command: ActorCommand<Self::ShutDown> = <Self as AsyncHandle<_>>::async_handle(self, msg).await.into();
                                if let Err(err) = command.0 {
                                    let _ = err?;
                                    break;
                                } else {
                                    continue;
                                }
                            }
                            ActorMsg::Shutdown(shutdown) => {
                                return Err(Self::ShutDown::from(shutdown));
                            }
                        }
                    } else {
                        break;
                    }
                }
                msg = Listener::next_msg(&mut self.interval2) => {
                    if let Some(msg) = msg {
                        match msg {
                            ActorMsg::Msg(msg) => {
                                let command: ActorCommand<Self::ShutDown> = <Self as AsyncHandle<_>>::async_handle(self, msg).await.into();
                                if let Err(err) = command.0 {
                                    let _ = err?;
                                    break;
                                } else {
                                    continue;
                                }
                            }
                            ActorMsg::Shutdown(shutdown) => {
                                return Err(Self::ShutDown::from(shutdown));
                            }
                        }
                    } else {
                        break;
                    }
                }
                msg = Listener::next_msg(&mut self.channel) => {
                    if let Some(msg) = msg {
                        match msg {
                            ActorMsg::Msg(msg) => {
                                let command: ActorCommand<Self::ShutDown> = <Self as AsyncHandle<_>>::async_handle(self, msg).await.into();
                                if let Err(err) = command.0 {
                                    let _ = err?;
                                    break;
                                } else {
                                    continue;
                                }
                            }
                            ActorMsg::Shutdown(shutdown) => {
                                return Err(Self::ShutDown::from(shutdown));
                            }
                        }
                    } else {
                        break;
                    }
                }
            }
        }
        Ok(Break)
    }
}
*/

impl AsyncHandle<u32> for MyFirstTestActor {
    async fn async_handle(&mut self, value: u32) -> Continue {
        // some code
        self.some_data = !self.some_data;
        let _ = value;
    }
}

impl Handle<u64> for MyFirstTestActor {
    fn handle(&mut self, value: u64) -> Cmd<MyActorShutdown> {
        let _ = value;
        Cmd(MyActorShutdown)
    }
}

impl AsyncHandle<String> for MyFirstTestActor {
    async fn async_handle(&mut self, value: String) -> Option<Break> {
        let _ = value;
        None
    }
}

impl AsyncHandle<Instant> for MyFirstTestActor {
    async fn async_handle(&mut self, _value: Instant) -> Option<Cmd<MyActorShutdown>> {
        self.some_other_data += 1;
        println!("New msg, couner: {}", self.some_other_data);

        if self.some_other_data > 5 {
            Some(Cmd(MyActorShutdown))
        } else {
            None
        }
    }
}

impl ApplyCmd<MyActorShutdown> for MyFirstTestActor {
    async fn apply_cmd(self, shut_down: MyActorShutdown) -> Option<Self> {
        println!("Called Shutdown");
        let _ = shut_down;
        None
    }
}

impl ApplyCmd<Break> for MyFirstTestActor {
    async fn apply_cmd(self, shut_down: Break) -> Option<Self> {
        println!("Called Break");
        let _ = shut_down;
        None
    }
}
