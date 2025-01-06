mod ten_polls;
use std::io::Read;
use std::{io, thread};

use futures_cpupool::Builder;
use tracing::error;
use tracing::info;
use tracing_subscriber;

use futures_channel::oneshot;
use futures_executor::block_on;
use futures_util::future::{FutureExt as _, TryFutureExt as _};
use grpcio::{
    ChannelBuilder, Environment, ResourceQuota, RpcContext, ServerBuilder, ServerCredentials,
    UnarySink,
};

use demo::helloworld::{HelloReply, HelloRequest};
use demo::helloworld_grpc::{create_greeter, Greeter};
use future01::Future;
use future03::compat::Future01CompatExt;
use std::sync::Arc;

#[derive(Clone)]
struct GreeterService {
    pool: futures_cpupool::CpuPool,
}

impl Greeter for GreeterService {
    fn say_hello(&mut self, ctx: RpcContext<'_>, req: HelloRequest, sink: UnarySink<HelloReply>) {
        let f01 = self.pool.spawn_fn(move || {
            let f = ten_polls::TenPolls::new(req.name.clone());
            Box::new(f)
        });
        // let f = Box::new(ten_polls::TenPolls::new(req.name.clone()));
        // let f01 = self.pool.spawn(f);
        // let f01 = {
        //     info!("Create future directly");
        //     let f = ten_polls::TenPolls::new(req.name.clone());
        //     Box::new(f)
        // };

        ctx.spawn(async move {
            let r = f01.compat().await.unwrap();

            let mut resp = HelloReply::default();
            resp.message = r;
            sink.success(resp)
                .map_err(move |e| error!("failed to reply {:?}", e))
                .map(|_| ())
                .await;
        })
    }
}

fn main() {
    tracing_subscriber::fmt::init();

    let env = Arc::new(Environment::new(1));
    let pool = Builder::new().pool_size(4).name_prefix("my-pool-").create();
    let service = create_greeter(GreeterService { pool });
    let addr = "127.0.0.1:50051";

    let quota = ResourceQuota::new(Some("HelloServerQuota")).resize_memory(1024 * 1024);
    let ch_builder = ChannelBuilder::new(env.clone()).set_resource_quota(quota);

    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .channel_args(ch_builder.build_args())
        .build()
        .unwrap();
    server
        .add_listening_port(addr, ServerCredentials::insecure())
        .unwrap();
    server.start();
    info!("listening on {addr}");
    let (tx, rx) = oneshot::channel();
    thread::spawn(move || {
        info!("Press ENTER to exit...");
        let _ = io::stdin().read(&mut [0]).unwrap();
        tx.send(())
    });
    let _ = block_on(rx);
    let _ = block_on(server.shutdown());
}
