use std::sync::Arc;
use grpcio::*;
use proto::pingpong_grpc::*;
use proto::pingpong::*;

use futures_executor::block_on;
use futures_util::future::{FutureExt as _, TryFutureExt as _};
use futures_channel::oneshot;

use std::{thread, io, io::Read};
use clap::Parser;

#[derive(Parser)]
#[clap(name = "server")]
#[clap(version = "1.0")]
struct Argument {
    /// grpc ip
    #[clap(long = "--ip")]
    rpc_ip: String,

    /// grpc port
    #[clap(long = "--port")]
    rpc_port: u16,

    /// threads
    #[clap(long = "--threads")]
    thread_count: usize,
}


#[derive(Clone)]
struct GreeterService {}

impl Greeter for GreeterService {
    fn say_hello(&mut self, ctx: RpcContext<'_>, _request: PingRequest, sink: UnarySink<PongReply>) {
        let mut resp = PongReply::default();
        let data = vec![0; _request.resp_length as usize];
        resp.set_data(data);

        ctx.spawn(
            sink.success(resp)
                .map_err(|e| panic!("failed to reply {:?}", e))
                .map(|_| ()),
        );
    }
}

#[allow(unused)]
fn main() {
    let arguments = Argument::parse();

    let ip = arguments.rpc_ip;
    let port = arguments.rpc_port;
    let thread_count = arguments.thread_count;
    let addr = format!("{}:{}", ip, port);

    println!("listen addr:{}", addr);

    let env = Arc::new(EnvBuilder::new().cq_count(thread_count).build());
    let service = GreeterService {};

    let mut server = ServerBuilder::new(env.clone())
        .register_service(create_greeter(service))
        .build()
        .unwrap();
    server
        .add_listening_port(addr, ServerCredentials::insecure())
        .unwrap();
    server.start();

    let (tx, rx) = oneshot::channel();
    thread::spawn(move || {
        let _ = io::stdin().read(&mut [0]).unwrap();
        tx.send(())
    });
    let _ = block_on(rx);
    let _ = block_on(server.shutdown());
    drop(server);
    drop(env);
}
