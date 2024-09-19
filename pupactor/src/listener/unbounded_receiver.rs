use crate::{ActorListener, ActorMsg};
use tokio::sync::mpsc::UnboundedReceiver;

/// UnboundedReceiver
impl<Msg, Command> ActorListener<Msg, Command> for UnboundedReceiver<ActorMsg<Msg, Command>> {
    #[inline(always)]
    async fn next_msg(&mut self) -> Option<ActorMsg<Msg, Command>> {
        self.recv().await
    }
}
