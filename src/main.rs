use std::collections::HashMap;

use chrono::prelude::*;
use iron::{
  middleware::Handler,
  prelude::*,
  status::Status,
};
use rusoto_sqs::{Sqs, SqsClient, SendMessageError, SendMessageRequest};
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;


/// The value of the `source` field in a `MozDefEvent`.
const MOZDEF_SOURCE: &'static str  = "mozdef-proxy";

trait Enqueue {
  type Data;
  type Error;

  fn queue(&self, data: &Self::Data) -> Result<(), Self::Error>;
}

pub struct Proxy<S> {
  message_queue: SQS<S>,
}

struct SQS<S> {
  client: S,
  queue_url: String,
}

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
  let sqs_client = SqsClient::new_with(
    rusoto_mock::MockRequestDispatcher::default(),
    rusoto_mock::MockCredentialsProvider,
    Default::default());
  let sqs = SQS::new(sqs_client, "test_queue");
  let proxy = Proxy::new(sqs);
  iron::Iron::new(proxy).http("127.0.0.1:8080").unwrap();
}

impl<S> Proxy<S> {
  pub fn new(queue: SQS<S>) -> Self {
    Proxy {
      message_queue: queue,
    }
  }
}

impl<S> SQS<S> {
  pub fn new<T: Into<String>>(client: S, queue: T) -> Self {
    SQS {
      client: client,
      queue_url: queue.into(),
    }
  }
}

impl<S> Handler for Proxy<S> {
  fn handle(&self, req: &mut Request) -> IronResult<Response> {
    let data = req.get::<bodyparser::Struct<ClientEvent>>()
      .map_err(|err| IronError::new(err, (Status::BadRequest, "Invalid request data")))
      ?.unwrap();
    let event: MozDefEvent = From::from(data);
    println!("Got event {:?}", event);

    Ok(Response::with((Status::Ok, "Success")))
  }
}

impl<S> Enqueue for SQS<S>
  where S: Sqs,
{
  type Data = MozDefEvent;
  type Error = SendMessageError;

  fn queue(&self, event: &Self::Data) -> Result<(), Self::Error> {
    let body = serde_json::to_string(event).unwrap();
    let request = SendMessageRequest {
      message_body: body,
      queue_url: self.queue_url.clone(),
      delay_seconds: None,
      message_attributes: None,
      message_deduplication_id: None,
      message_group_id: None,
    };

    self.client
      .send_message(request)
      .sync()
      .map(|_| ())
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