use crate::{ActorListener, ActorMsg};
use std::time::Duration;

pub struct SleepDuration(Duration);

impl ActorListener<SleepDone> for SleepDuration {
    #[inline]
    async fn next_msg(&mut self) -> Option<ActorMsg<SleepDone>> {
        tokio::time::sleep(self.0).await;
        Some(ActorMsg::Msg(SleepDone))
    }
}

impl SleepDuration {
    #[inline]
    pub fn new(dur: Duration) -> Self {
        Self(dur)
    }
}

pub struct SleepDone;
