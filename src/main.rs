mod proxy;

use iron::{
  middleware::Handler,
  prelude::*,
  status::Status,
};


pub struct Proxy {
}

fn main() {
  let proxy = proxy::Proxy::new();
  iron::Iron::new(proxy).http("127.0.0.1:8080").unwrap();
}

impl Proxy {
  pub fn new() -> Self {
    Proxy {}
  }
}

impl Handler for Proxy {
  fn handle(&self, req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((Status::Ok, "Hello world")))
  }
}