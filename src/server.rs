use std::collections::HashMap;

use chrono::prelude::*;
use iron::{
  prelude::*,
  middleware::Handler,
  status::Status,
};
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

use crate::capability::Capability;
use crate::queue::Enqueue;


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
#[derive(Debug, Deserialize)]
pub struct ClientEvent {
  pub category: String,
  pub hostname: String,
  pub severity: Severity,
  pub process: String,
  pub summary: String,
  pub details: HashMap<String, Value>,
}

/// Contains all of the information MozDef expects to be in an event.
#[derive(Debug, Serialize)]
struct MozDefEvent {
  pub category: String,
  pub hostname: String,
  pub severity: Severity,
  pub process: String,
  pub summary: String,
  pub details: HashMap<String, Value>,
  pub processid: u32,
  pub source: &'static str,
  pub tags: Vec<String>,
  pub timestamp: DateTime<Local>,
  pub utctimestamp: DateTime<Utc>,
}

/// A request handler that enqueues events sent by clients for consumption by MozDef.
pub(crate) struct Proxy {
}

impl Proxy {
  /// Construct a new proxy server.
  pub fn new() -> Self {
    Proxy{}
  }
}

impl Handler for Proxy {
  fn handle(&self, req: &mut Request) -> IronResult<Response> {
    Ok(Response::with(Status::Ok))
  }
}