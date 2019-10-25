use std::time::Duration;

use actix::*;
use actix::io::SinkWrite;
use actix_codec::{AsyncRead, AsyncWrite, Framed};
use awc::{
    error::WsProtocolError,
    ws::{Codec, Frame, Message},
};

use futures::stream::SplitSink;

use bytes::Bytes;

pub struct WsClient<T>(pub SinkWrite<SplitSink<Framed<T, Codec>>>)
    where
        T: AsyncRead + AsyncWrite;

impl<T: 'static> Actor for WsClient<T>
    where
        T: AsyncRead + AsyncWrite,
{
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        // start heartbeats otherwise server will disconnect after 10 seconds
        self.hb(ctx)
    }

    fn stopped(&mut self, _: &mut Context<Self>) {
        println!("Disconnected");

        // Stop application on disconnect
        System::current().stop();
    }
}

impl<T: 'static> WsClient<T>
    where
        T: AsyncRead + AsyncWrite,
{
    fn hb(&self, ctx: &mut Context<Self>) {
        ctx.run_later(Duration::new(1, 0), |act, ctx| {
            act.0.write(Message::Ping(String::new())).unwrap();
            act.hb(ctx);

            // client should also check for a timeout here, similar to the
            // server code
        });
    }
}

#[derive(Message)]
pub struct ClientCommand(pub String);

impl<T: 'static> Handler<ClientCommand> for WsClient<T>
    where
        T: AsyncRead + AsyncWrite,
{
    type Result = ();

    fn handle(&mut self, msg: ClientCommand, _ctx: &mut Context<Self>) {
        let shirt = super::models::create_large_shirt(msg.0);
        let bytes = super::models::serialize_shirt(&shirt);
        self.0.write(Message::Binary(Bytes::from(bytes))).unwrap();
    }
}

impl<T: 'static> StreamHandler<Frame, WsProtocolError> for WsClient<T>
    where
        T: AsyncRead + AsyncWrite,
{
    fn handle(&mut self, msg: Frame, _ctx: &mut Context<Self>) {
        match msg {
            Frame::Text(txt) => println!("Server text: {:?}", txt),
            Frame::Binary(bin) => {
                let bytes = bin.unwrap().to_vec();
                let shirt = super::models::deserialize_shirt(&bytes);
                println!("Server binary: {:?}", shirt);
            },
            _ => () // do nothing
        }
    }

    fn started(&mut self, _ctx: &mut Context<Self>) {
        println!("Connected");
    }

    fn finished(&mut self, ctx: &mut Context<Self>) {
        println!("Server disconnected");
        ctx.stop()
    }
}

impl<T: 'static> actix::io::WriteHandler<WsProtocolError> for WsClient<T> where
    T: AsyncRead + AsyncWrite
{
}

