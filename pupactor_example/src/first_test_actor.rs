use pupactor::{actor_channel, run_actor, ActorMsg, ApplyCmd, AsyncHandle, Break, Cmd, Continue, Handle, InitActor, Listener};
use pupactor::{ActorCmd, ActorMsgHandle, Pupactor};
use std::time::Instant;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::time::Interval;

// This macro allows to write `handle` on each enum variant
// Keep in mind that enum variants should be only one value:
// bad: `MyEnumVariant(u32, String)`
// good: `MyEnumVariant((u32, String))`
#[derive(ActorMsgHandle)]
#[actor(kind = "MyFirstTestActor")]
pub enum Value {
    MyGreetings(String),
    U32(u32),
    U64(u64),
}

// Any command like `shutdown` or other problems with `listener`
#[derive(ActorCmd)]
pub struct MyActorShutdown;


#[derive(Pupactor)]
#[actor(cmd = "MyActorShutdown")]
struct MyFirstTestActor {
    some_data: bool,
    some_other_data: usize,
    #[listener]
    interval: Listener<Interval, Instant>,
    #[listener]
    interval2: Listener<Interval, Instant>,
    #[listener]
    channel: Listener<UnboundedReceiver<ActorMsg<Value, MyActorShutdown>>, Value, MyActorShutdown>,
}

impl InitActor<UnboundedReceiver<ActorMsg<Value, MyActorShutdown>>> for MyFirstTestActor {
    async fn init_actor(receiver: UnboundedReceiver<ActorMsg<Value, MyActorShutdown>>) -> Option<Self> {
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
    let (sender, receiver) = actor_channel::<Value, MyActorShutdown>();

    sender.send(Value::MyGreetings("Hello".to_string()));
    sender.send(Value::U32(100));
    sender.send(Value::U64(200));


    // We also can send command from outside
    // sender.command(MyActorShutdown);


    // so sender will not die before actor
    let _sender = sender;

    // actor is already spawned
    // join_handle is a result of tokio::spawn
    let join_handle = run_actor::<MyFirstTestActor>(receiver);


    // Wait join_handle, so main thread will not be killed
    // Usually we do not need it
    let _ = join_handle.await;
}

impl AsyncHandle<u32> for MyFirstTestActor {
    async fn async_handle(&mut self, value: u32) -> Continue {
        self.some_data = !self.some_data;
        println!("New msg: {value}");
        let _ = value;
    }
}

impl Handle<u64> for MyFirstTestActor {
    fn handle(&mut self, value: u64) -> Option<Break> {
        println!("New msg: {value}");
        None
    }
}

impl AsyncHandle<String> for MyFirstTestActor {
    async fn async_handle(&mut self, value: String) -> Option<Break> {
        println!("New msg: {value}");
        None
    }
}

impl AsyncHandle<Instant> for MyFirstTestActor {
    async fn async_handle(&mut self, _value: Instant) -> Option<Cmd<MyActorShutdown>> {
        self.some_other_data += 1;
        println!("New msg, counter: {}", self.some_other_data);
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
