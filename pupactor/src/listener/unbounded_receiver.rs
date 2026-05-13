use crate::{ActorListener, ActorMsg};
use tokio::sync::mpsc;

/// UnboundedReceiver
impl<Msg, Command> ActorListener<Msg, Command> for mpsc::UnboundedReceiver<ActorMsg<Msg, Command>> {
    #[inline(always)]
    async fn next_msg(&mut self) -> Option<ActorMsg<Msg, Command>> {
        self.recv().await
    }
}

impl<Msg, Command> ActorListener<Msg, Command> for mpsc::Receiver<ActorMsg<Msg, Command>> {
    #[inline(always)]
    async fn next_msg(&mut self) -> Option<ActorMsg<Msg, Command>> {
        self.recv().await
    }
}
