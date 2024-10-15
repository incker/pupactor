pub use actor::*;
pub use actor_command::*;
pub use actor_ref::*;
pub use change_state::*;
pub use handle::*;
pub use init_actor::*;
pub use listener::*;
use std::convert::Infallible;

mod actor;
mod actor_command;
mod actor_ref;
mod handle;
mod init_actor;
mod listener;
mod change_state;

// macros
pub use pupactor_macro::{ActorMsgHandle, ActorShutdown, Pupactor};


pub struct MyActorShutdown;

// #[derive(Pupactor)]
// #[actor(shutdown = "MyActorShutdown")]
struct MyFirstTestActor {
    some_data: bool,
}

impl Actor for MyFirstTestActor {
    type States = Self;
    type Cmd = Infallible;

    async fn infinite_loop(&mut self) -> Result<Break, Self::Cmd> {
        Ok(Break)
    }
}