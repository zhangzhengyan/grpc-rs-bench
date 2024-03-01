use std::sync::Arc;

// use proto::pingpong_grpc::GreeterClient;
// use proto::pingpong_grpc::GreeterClient;
// use futures::TryFutureExt;
// use grpcio::*;
// use proto::pingpong_grpc::*;
// use proto::pingpong::*;
use proto::tonicPingpong::{greeter_client, PingRequest};

use std::thread::{self, JoinHandle};
// use futures_executor::block_on;
// use futures_util::future;

use clap::Parser;
use tokio;
use tokio::task;
use tokio::runtime::Builder;
use tokio::sync::Barrier;

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

    /// tasks
    #[clap(long = "--tasks")]
    tasks: usize,

    /// data size byte
    #[clap(long = "--size")]
    size: usize,

    /// data size byte
    #[clap(long = "--resp_size")]
    r_size: usize,
}

#[allow(unused)]
#[tokio::main]
async fn main() {
      let arguments = Argument::parse();

      let ip = arguments.rpc_ip;
      let port = arguments.rpc_port;
      let times = arguments.times;
      let tasks = arguments.tasks;
      let thread_count = arguments.thread_count;
      let size = arguments.size;
      let r_size = arguments.r_size;

      let barrier = Arc::new(Barrier::new(thread_count));
      let addr = format!("http://{}:{}", ip, port);
      println!("remote addr:{}", addr);
      let mut j_vec = Vec::new();
      let data: Vec<u8> = vec![0; size];
      let mut req = PingRequest{ data, resp_length: r_size as i32 };
      

      // let mut handles = Vec::with_capacity(thread_count);
      let spawn_reqs = |req: PingRequest, 
                        addr: String, 
                        barrier_c: Arc<Barrier>| -> JoinHandle<()> {
            thread::spawn(move || {
                  let runtime = Builder::new_current_thread()
                                          .enable_all()
                                          .build()
                                          .unwrap();
                  
                  // let rt_run = Builder::new_current_thread()
                  //                         .enable_all()
                  //                         .build()
                  //                         .unwrap();

                  tokio::task::LocalSet::new().block_on(&runtime, async move {
                        let mut client = greeter_client::GreeterClient::connect(addr).await.unwrap();

                        let mut tasks_w = Vec::new();

                        for _i in 0..tasks {
                              let b_c = barrier_c.clone();
                              let mut cli_c = client.clone();

                              // let mut reply = cli_c.say_hello_async(&req).unwrap();
                              let req_c = req.clone();
                              // let f1 = rt_run.spawn(async move {
                              let f1 = task::spawn(async move { 
                                    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                                    for _j in 0..times {
                                          let r_c = req_c.clone();
                                          let mut _reply = cli_c.say_hello(r_c).await.unwrap();
                                          // println!("res {:?}", _reply.into_inner().data.len()); 
                                    }
                                    
                                    b_c.wait().await
                              });

                              tasks_w.push(f1);
                        }

                        for f in tasks_w {
                              f.await.unwrap();
                        }
                  });
            })
      };

      for _i in 0..=thread_count {
            j_vec.push(spawn_reqs(req.clone(), addr.to_owned(), barrier.clone()));
            println!("thread_{} start...", _i);
      }

      for _job in j_vec {
            // runtime.block_on(_job).unwrap();
            _job.join().unwrap();
            println!("thread end...");
      }
}
