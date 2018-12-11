mod capability;
mod server;
mod queue;

use env_logger::{Builder, Target};
use log::info;
use iron::prelude::*;

use crate::server::Proxy;


fn main() {
  let mut logger = Builder::from_default_env();
  logger.target(Target::Stderr);
  logger.init();

  info!("Running proxy server on 127.0.0.1:8080");
  Iron::new(Proxy::new()).http("127.0.0.1:8080").unwrap();
}
