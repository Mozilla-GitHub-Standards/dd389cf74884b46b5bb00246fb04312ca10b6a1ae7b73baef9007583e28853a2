use iron::{
  middleware::Handler,
  prelude::*,
  status::Status,
};


pub struct Proxy {
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