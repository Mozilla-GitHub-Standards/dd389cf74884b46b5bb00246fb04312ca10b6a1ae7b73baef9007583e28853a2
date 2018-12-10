extern crate clap;
extern crate env_logger;
extern crate iron;
#[macro_use] extern crate log;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;


use env_logger::{Builder, Target};

fn main() {
  let mut logger = Builder::from_default_env();
  logger.target(Target::Stderr);
  logger.init();

  error!("Testing");
}
