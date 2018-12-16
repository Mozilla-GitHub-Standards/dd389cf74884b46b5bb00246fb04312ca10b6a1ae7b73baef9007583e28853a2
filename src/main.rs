use std::collections::HashMap;

use chrono::prelude::*;
use iron::{
  middleware::Handler,
  prelude::*,
  status::Status,
};
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;


pub struct Proxy {
}

/// The value of the `source` field in a `MozDefEvent`.
const MOZDEF_SOURCE: &'static str  = "mozdef-proxy";

/// Represents each of the possible severity levels for an event.
/// Specified by RFC5424.
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum Severity {
  DEBUG,
  INFO,
  NOTICE,
  WARNING,
  ERROR,
  CRITICAL,
  ALERT,
  EMERGENCY,
}

/// Contains all of the information that a client must send to queue an event.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ClientEvent {
  pub category: String,
  pub hostname: String,
  pub severity: Severity,
  pub process: String,
  pub summary: String,
  pub tags: Vec<String>,
  pub details: HashMap<String, Value>,
}

/// Contains all of the information MozDef expects to be in an event.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MozDefEvent {
  pub category: String,
  pub hostname: String,
  pub severity: Severity,
  pub process: String,
  pub summary: String,
  pub tags: Vec<String>,
  pub details: HashMap<String, Value>,
  pub source: &'static str,
  pub timestamp: DateTime<Local>,
  pub utctimestamp: DateTime<Utc>,
}

fn main() {
  let proxy = Proxy::new();
  iron::Iron::new(proxy).http("127.0.0.1:8080").unwrap();
}

impl Proxy {
  pub fn new() -> Self {
    Proxy {}
  }
}

impl Handler for Proxy {
  fn handle(&self, req: &mut Request) -> IronResult<Response> {
    let data = req.get::<bodyparser::Struct<ClientEvent>>()
      .map_err(|err| IronError::new(err, (Status::BadRequest, "Invalid request data")))
      ?.unwrap();
    let event: MozDefEvent = From::from(data);
    println!("Got event {:?}", event);

    Ok(Response::with((Status::Ok, "Success")))
  }
}


impl From<ClientEvent> for MozDefEvent {
  fn from(event: ClientEvent) -> MozDefEvent {
    MozDefEvent {
      category: event.category,
      hostname: event.hostname,
      severity: event.severity,
      process: event.process,
      summary: event.summary,
      tags: event.tags,
      details: event.details,
      source: MOZDEF_SOURCE,
      timestamp: Local::now(),
      utctimestamp: Utc::now(),
    }
  }
}