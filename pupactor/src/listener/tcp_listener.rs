use crate::{ActorListener, ActorMsg};
use std::io;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};

impl ActorListener<io::Result<(TcpStream, SocketAddr)>> for TcpListener {
    #[inline]
    async fn next_msg(&mut self) -> Option<ActorMsg<io::Result<(TcpStream, SocketAddr)>>> {
        Some(ActorMsg::Msg(self.accept().await))
    }
}
