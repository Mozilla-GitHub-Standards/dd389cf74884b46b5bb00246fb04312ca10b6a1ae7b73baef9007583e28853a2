use std::collections::HashMap;
use std::convert::From;
use std::error::Error;
use std::fmt;

use chrono::prelude::*;
use iron::{
  prelude::*,
  middleware::Handler,
  status::Status,
};
use log::{error, debug};
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

/// A request handler that enqueues events sent by clients for consumption by MozDef.
pub struct Proxy<Q> {
  message_queue: Q,
}

#[derive(Clone, Copy, Debug)]
enum APIError {
  InvalidRequestData,
  InternalError,
}

impl fmt::Display for APIError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      APIError::InvalidRequestData => write!(f, "Invalid request data"),
      APIError::InternalError      => write!(f, "Internal error"),
    }
  }
}

impl Error for APIError {
  fn description(&self) -> &str {
    "API Error"
  }
}

impl<Q> Proxy<Q> {
  /// Construct a new proxy server.
  pub fn new(message_queue: Q) -> Self {
    Proxy{
      message_queue: message_queue,
    }
  }
}

impl<Q, E> Handler for Proxy<Q>
  where Q: Capability<Enqueue<MozDefEvent>, Output = Result<(), E>> + Send + Sync + 'static,
        E: Error,
{
  fn handle(&self, req: &mut Request) -> IronResult<Response> {
    let body = req.get::<bodyparser::Struct<ClientEvent>>()
      .map_err(|_| IronError::new(
        APIError::InvalidRequestData,
        (Status::BadRequest, "Invalid request data")
      ))?;

    if body.is_none() {
      return Err(IronError::new(
        APIError::InvalidRequestData,
        (Status::BadRequest, "Invalid request data")
      ));
    }

    let evt: MozDefEvent = From::from(body.unwrap());
    self.message_queue
      .lock()
      .map_err(|e| {
        error!("Could not get lock on mutex. {}", e);
        ()
      })
      .map(|queue| -> Result<(), ()> {
        let res = queue.perform(Enqueue(evt)) as Result<(), E>;
        res
          .map(|_| {
            debug!("Successfully enqueued message");
            ()
          })
          .map_err(|e| {
            error!("Failed to enqueue message: {}", e);
            ()
          })
      })
      .map_err(|_| IronError::new(
        APIError::InternalError,
        (Status::InternalServerError, "Failed to queue event")
      ))?;

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

#[cfg(test)]
mod tests {
  use super::*;

  use std::collections::HashMap;

  use iron::prelude::*;
  use iron::status::Status;
  use rusoto_sqs::SqsClient;

  use crate::testing::{Message, MockSqs};
  use crate::queue::SQS;


}
