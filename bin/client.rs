// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.
#[allow(deprecated)]
use tracing::info;

use std::sync::Arc;

use demo::helloworld::HelloRequest;
use demo::helloworld_grpc::GreeterClient;
use grpcio::{ChannelBuilder, EnvBuilder};

fn main() {
    tracing_subscriber::fmt::init();

    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect("localhost:50051");
    let client = GreeterClient::new(ch);

    let mut req = HelloRequest::default();
    req.name = "world".to_owned();
    let reply = client.say_hello(&req).expect("rpc");
    info!("Greeter received: {}", reply.message);
}
