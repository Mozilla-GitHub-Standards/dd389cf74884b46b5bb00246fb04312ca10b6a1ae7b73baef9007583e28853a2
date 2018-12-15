mod capability;
mod server;
mod testing;
mod queue;

use std::sync::{Arc, Mutex};

use env_logger::{Builder, Target};
use log::info;
use iron::prelude::*;
use rusoto_sqs::SqsClient;

use crate::server::Proxy;
use crate::queue::SQS;


// Required configuration:
// 1. address and port to bind web server to
// 2. url of SQS queue to write to
// 3. source of AWS credentials (use standard env vars?)

fn main() {
  let mut logger = Builder::from_default_env();
  logger.target(Target::Stderr);
  logger.init();

  let sqs = testing::MockSqs::new();
  let sqs = Arc::new(Mutex::new(SQS::new(sqs, "testqueue")));
  let proxy = Proxy::new(sqs);

  info!("Running proxy server on 127.0.0.1:8080");
  Iron::new(proxy).http("127.0.0.1:8080").unwrap();
}
