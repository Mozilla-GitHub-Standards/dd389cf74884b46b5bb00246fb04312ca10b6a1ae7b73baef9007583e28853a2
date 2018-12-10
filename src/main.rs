use log::error;
use env_logger::{Builder, Target};

fn main() {
  let mut logger = Builder::from_default_env();
  logger.target(Target::Stderr);
  logger.init();

  error!("Testing");
}
