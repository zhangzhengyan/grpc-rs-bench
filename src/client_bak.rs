use std::sync::Arc;

// use futures::TryFutureExt;
use grpcio::*;
use proto::pingpong_grpc::*;
use proto::pingpong::*;

use std::thread::{self, JoinHandle};
use futures_executor::block_on;
// use futures_util::future;

use clap::Parser;

#[derive(Parser)]
#[clap(name = "server")]
#[clap(version = "1.0")]
struct Argument {
    /// grpc ip
    #[clap(long = "--rip")]
    rpc_ip: String,

    /// grpc port
    #[clap(long = "--rport")]
    rpc_port: u16,

    /// threads
    #[clap(long = "--threads")]
    thread_count: usize,

    /// loop times
    #[clap(long = "--times")]
    times: usize,

    /// data size byte
    #[clap(long = "--size")]
    size: usize,

    /// data size byte
    #[clap(long = "--resp_size")]
    r_size: usize,
}

#[allow(unused)]
fn main() {
      let arguments = Argument::parse();

      let ip = arguments.rpc_ip;
      let port = arguments.rpc_port;
      let times = arguments.times;
      let thread_count = arguments.thread_count;
      let size = arguments.size;
      let r_size = arguments.r_size;

      // let env = Arc::new(EnvBuilder::new().build());
      // let env = Arc::new(EnvBuilder::new().cq_count(thread_count).build());
      let addr = format!("{}:{}", ip, port);
      println!("remote addr:{}", addr);
      let mut j_vec = Vec::new();
      let data: Vec<u8> = vec![0; size];
      let mut req = PingRequest::default();
      req.set_data(data);
      req.set_resp_length(r_size as i32);

      let spawn_reqs = |req: PingRequest| -> JoinHandle<()> {
            let env = Arc::new(EnvBuilder::new().build());
            let ch = ChannelBuilder::new(env).connect(&addr);
            let mut client = GreeterClient::new(ch);
            // let mut resps = Vec::with_capacity(times);
            thread::spawn(move || {
                  for _i in 0..times {
                        // resps.push(client.say_hello_async(&req).unwrap());
                        // let cli_c = client.clone();
                        // let req_c = req.clone();
                        // client.spawn(async move {
                        //       let mut reply = cli_c.say_hello_async(&req_c).unwrap();
                        //       let res = reply.message().await.unwrap(); 
                        //       println!("res {:?}", res.data.len())
                        // });
                        
                        // let mut reply = client.say_hello_async(&req).unwrap();
                        let mut reply = client.say_hello(&req).unwrap();
                        println!("res {:?}", reply.data.len());
                        // client.spawn(async move { 
                        //       let res = reply.message().await.unwrap(); 
                        //       println!("res {:?}", res.data.len());
                        // });
                        // println!("index:{}", _i);
                        // block_on(async { 
                        //       let res = reply.message().await.unwrap(); 
                        //       println!("res {:?}", res.data.len()) 
                        // });
                  }
                  // block_on(async { future::try_join_all(resps).await }).unwrap();
            })
      };

      for _i in 0..=thread_count {
            j_vec.push(spawn_reqs(req.clone()));
            println!("thread_{} start...", _i);
      }

      for _job in j_vec {
            _job.join().unwrap();
            println!("thread end...");
      }
}
