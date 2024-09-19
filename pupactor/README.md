Pupactor is actor model library built with tokio

Macros and traits could help you to organise actors to look same


Actor body looks like:

```rust
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
```

where `Pupactor` macro generates `select!` macro that listens `interval`, `interval2` and `channel`

On each income event (`Interval`, `Value`) we need create a Handle like this:

```rust
impl AsyncHandle<Instant> for MyFirstTestActor {
    async fn async_handle(&mut self, _value: Instant) {
        self.some_other_data += 1;
        println!("New msg, counter: {}", self.some_other_data);
    }
}
```

Or sync handle if you do not need async 


```rust
impl Handle<Instant> for MyFirstTestActor {
    fn handle(&mut self, _value: Instant) {
        self.some_other_data += 1;
        println!("New msg, counter: {}", self.some_other_data);
    }
}
```


For more complex values like enums, there is other macro `ActorMsgHandle`:

```rust
#[derive(ActorMsgHandle)]
#[actor(kind = "MyFirstTestActor")]
pub enum Value {
    MyGreetings(String),
    U32(u32),
    U64(u64),
}
```

So you will need describe handles only for `String`, `u32` and `u64`

```rust
impl AsyncHandle<u32> for MyFirstTestActor {
    async fn async_handle(&mut self, value: u32) -> Continue {
        println!("New msg: {value}");
    }
}
```


All handles can return something. All return types:
```
Continue - actor continue live
Cmd<T> where T is MyActorShutdown - this is your custom command
Break - this command will break the infinite loop { select!{ ... } }
Resut<Continue, Cmd<T>>
Resut<Continue, Break>
Resut<Continue, Result<Break, Cmd<T>>>
```


