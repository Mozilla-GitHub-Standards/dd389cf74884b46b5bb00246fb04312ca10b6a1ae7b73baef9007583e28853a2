mod capability;
mod server;
mod queue;

use env_logger::{Builder, Target};
use log::info;
use iron::prelude::*;

use crate::server::Proxy;


// Required configuration:
// 1. address and port to bind web server to
// 2. url of SQS queue to write to
// 3. source of AWS credentials (use standard env vars?)

fn main() {
  let mut logger = Builder::from_default_env();
  logger.target(Target::Stderr);
  logger.init();

  info!("Running proxy server on 127.0.0.1:8080");
  Iron::new(Proxy::new()).http("127.0.0.1:8080").unwrap();
}
