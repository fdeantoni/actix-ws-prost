pub mod models;
pub mod client;

use actix::*;
use actix::io::SinkWrite;
use awc::Client;

use futures::{
    lazy,
    stream::Stream,
    Future,
};

use std::{io, thread};


fn main() {
    println!("hello!");
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let sys = actix::System::new("ws-client");

    Arbiter::spawn(lazy(|| {
        Client::new()
            .ws("ws://localhost:8080/echo/")
            .connect()
            .map_err(|e| {
                println!("Error: {}", e);
            })
            .map(|(response, framed)| {
                println!("{:?}", response);
                let (sink, stream) = framed.split();
                let addr = client::WsClient::create(|ctx| {
                    client::WsClient::add_stream(stream, ctx);
                    client::WsClient(SinkWrite::new(sink, ctx))
                });

                // start console loop
                thread::spawn(move || loop {
                    let mut cmd = String::new();
                    if io::stdin().read_line(&mut cmd).is_err() {
                        println!("error");
                        return;
                    }
                    addr.do_send(client::ClientCommand(cmd));
                });
            })
    }));

    let _ = sys.run();
}





