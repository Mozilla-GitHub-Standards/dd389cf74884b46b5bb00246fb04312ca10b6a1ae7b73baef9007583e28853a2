use std::sync::{Arc, Mutex};

use futures::{
  future::FutureResult,
  Future,
};
use rusoto_core::request::DispatchSignedRequest;
use rusoto_credential::{AwsCredentials, CredentialsError, ProvideAwsCredentials};
use rusoto_sqs::{SendMessageError, SendMessageRequest, Sqs, SqsClient};
use serde::Serialize;
use serde_derive::Serialize;

use crate::capability::Capability;


/// An operation for enqueuing data e.g. into SQS.
pub struct Enqueue<T>(pub T);

/// Errors that may be encountered trying to queue an event.
pub enum EnqueueError{}

/// SQS supports enqueueing event data to Amazon SQS.
pub struct SQS<S> {
  client: S,
  queue: String,
}

impl<S> SQS<S> {
  /// Construct a new SQS configuration.
  pub fn new<T: Into<String>>(client: S, queue_url: T) -> Self {
    SQS{
      client: client,
      queue: queue_url.into(),
    }
  }
}

impl<S, T> Capability<Enqueue<T>> for SQS<S>
  where S: Sqs,
        T: Serialize,
{
  type Output = Result<(), SendMessageError>;

  fn perform(&self, op: Enqueue<T>) -> Self::Output {
    let message_body = serde_json::to_string(&op.0).unwrap();
    let request = SendMessageRequest {
      message_body: message_body,
      queue_url: self.queue.clone(),
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

impl<S, T> Capability<Enqueue<T>> for Arc<Mutex<SQS<S>>>
  where S: Sqs,
        T: Serialize,
{
  type Output = Result<(), SendMessageError>;

  fn perform(&self, op: Enqueue<T>) -> Self::Output {
    let sqs = self.lock().unwrap();
    sqs.perform(op)
  }
}

#[cfg(test)]
pub mod tests {
  use super::*;

  use rusoto_sqs::SqsClient;
  use serde_derive::{Deserialize, Serialize};

  use crate::testing::{Message, MockSqs};


  #[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
  struct TestData {
    pub id: u32,
    pub name: String,
  }

  #[test]
  fn can_queue_data() {
    let data = TestData {
      id: 9001,
      name: "test data".to_string(),
    };
    let client = SqsClient::new_with(
      rusoto_mock::MockRequestDispatcher::default(),
      rusoto_mock::MockCredentialsProvider,
      Default::default());
    let sqs = SQS::new(client, "doesn't matter".to_string());

    assert!(sqs.perform(Enqueue(data)).is_ok());
  }

  #[test]
  fn messages_get_sent_to_sqs() {
    let data = TestData {
      id: 10000,
      name: "test".to_string(),
    };
    let client = MockSqs::new();
    let sqs = SQS::new(&client, "queue_name".to_string());

    assert!(sqs.perform(Enqueue(&data)).is_ok());

    let messages = client.messages.lock().unwrap();

    assert_eq!(messages.len(), 1);

    let Message{ body, queue } = messages[0].clone(); 
    let recvd: TestData = serde_json::from_str(&body).unwrap();

    assert_eq!(queue, "queue_name".to_string());
    assert_eq!(recvd, data);
  }
}
