use crate::{ActorListener, ActorMsg};
use tokio::time::{Instant, Interval};

/// Interval
impl<Resp> ActorListener<Resp> for Interval
where
    Resp: From<Instant>,
{
    #[inline(always)]
    async fn next_msg(&mut self) -> Option<ActorMsg<Resp>> {
        Some(ActorMsg::Msg(self.tick().await.into()))
    }
}
