use tokio::net::TcpStream;
use crate::Message::Message;
use std::io::Cursor;
use bytes::BytesMut;
use anyhow::Result;

pub struct Connection{
    socket: TcpStream,
    buffer: BytesMut,
}

impl Connection{
    pub fn new(socket: TcpStream) -> Connection{
        Connection{
            socket,
            buffer: BytesMut::with_capacity(4096),
        }
    }
    pub async fn read_message(&mut self) -> Result<Option<Message>>{
        loop{
            if let Some(message) = self.parse_message()? {
                return Ok(Some(message));
            }
        }

    }

    fn parse_message(&mut self) -> Result<Option<Message>> {
        let mut buf = Cursor::new(&self.buffer[..]);
        match  Message::check(&mut  buf) {
            Ok(_) => {
                Ok(None)
            },
            Err(Incomplete) => Ok(None),
            Err(e) => Err(e.into())

        }
    }
}