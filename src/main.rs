mod proxy;


fn main() {
  let proxy = proxy::Proxy::new();
  iron::Iron::new(proxy).http("127.0.0.1:8080").unwrap();
}