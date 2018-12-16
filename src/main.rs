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

/// Implementors have the ability to enqueue some data into a queue of some kind.
///
/// Largely used as an interface around which we can build mocks for testing.
trait Enqueue {
  type Data;
  type Error;

  fn queue(&self, data: &Self::Data) -> Result<(), Self::Error>;
}

/// The main HTTP handler type.
struct Proxy<S> {
  // We describe the Proxy as completely generic over its input type.
  // This does hypothetically leave room for instances of meaningless
  // things like `Proxy<u8>`, but it's a good habit to be in, as it
  // allows for better control over `S`'s trait bounds bounds in our own
  // implementations.
  message_queue: SQS<S>,
}

/// A configuration of an Amazon SQS to write to.
struct SQS<S> {
  // Its fields are private because it performs external state mutation
  // and thus cannot be thought of as a pure data type.
  client: S,
  queue_url: String,
}

/// Represents each of the possible severity levels for an event.
/// Specified by RFC5424.
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
enum Severity {
  // Enums automatically serialize to a string version of their names.
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
struct ClientEvent {
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
struct MozDefEvent {
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
  // TODO:
  // 1. Figure out how to handle AWS credentials in an acceptable way.
  // 2. Create a real SqsClient, authenticate, assume role, etc. as necessary.
  // 3. Write some tests.  Just make a best effort.
  // 4. Test it out in AWS!
  let sqs_client = SqsClient::new_with(
    rusoto_mock::MockRequestDispatcher::default(),
    rusoto_mock::MockCredentialsProvider,
    Default::default());
  let sqs = SQS::new(sqs_client, "test_queue");
  let proxy = Proxy::new(sqs);
  iron::Iron::new(proxy).http("127.0.0.1:8080").unwrap();
}

impl<S> Proxy<S> {
  fn new(queue: SQS<S>) -> Self {
    Proxy {
      message_queue: queue,
    }
  }
}

impl<S> SQS<S> {
  fn new<T: Into<String>>(client: S, queue: T) -> Self {
    SQS {
      client: client,
      queue_url: queue.into(),
    }
  }
}

impl<S> Handler for Proxy<S> 
  // The `Iron` instance that `Handler` is passed to will run our `handle` method
  // asynchronously, so we have to guarantee `S` is thread-safe.
  where S: Sqs + Send + Sync + 'static,
{
  fn handle(&self, req: &mut Request) -> IronResult<Response> {
    let data = req.get::<bodyparser::Struct<ClientEvent>>()
      .map_err(|err| IronError::new(err, (Status::BadRequest, "Invalid request data")))
      ?.unwrap();
    let event: MozDefEvent = From::from(data);
    println!("Got event {:?}", event);

    match self.message_queue.queue(&event) {
      Err(err) => Err(IronError::new(err, (Status::InternalServerError, "An error occurred in the server"))),
      Ok(_)    => Ok(Response::with((Status::Ok, "Success"))),
    }
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