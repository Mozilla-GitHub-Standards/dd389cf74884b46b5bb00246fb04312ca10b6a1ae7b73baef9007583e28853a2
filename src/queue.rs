use rusoto_sqs::SqsClient;
use serde::Serialize;
use serde_derive::Serialize;

use crate::capability::Capability;


/// An operation for enqueuing data e.g. into SQS.
pub struct Enqueue<T>(T);

/// Errors that may be encountered trying to queue an event.
pub enum EnqueueError{}

/// SQS supports enqueueing event data to Amazon SQS.
pub struct SQS<P, D>
  where P: rusoto_credential::ProvideAwsCredentials,
        D: rusoto_core::request::DispatchSignedRequest,
{
  client: SqsClient<P, D>,
}

impl<P, D> SQS<P, D>
  where P: rusoto_credential::ProvideAwsCredentials,
        D: rusoto_core::request::DispatchSignedRequest,
{
  /// Construct a new SQS configuration.
  pub fn new(client: SqsClient<P, D>) -> Self {
    SQS{
      client: client,
    }
  }
}

impl<P, D, T> Capability<Enqueue<T>> for SQS<P, D>
  where P: rusoto_credential::ProvideAwsCredentials,
        D: rusoto_core::request::DispatchSignedRequest,
        T: Serialize,
{
  type Output = Result<(), EnqueueError>;

  fn perform(&self, op: Enqueue<T>) -> Self::Output {
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use serde_derive::{Deserialize, Serialize};


  #[derive(PartialEq, Eq, Serialize)]
  struct TestData {
    pub id: u32,
    pub name: String,
  }

  #[test]
  fn can_queue_data_into_sqs() {
    let data = TestData {
      id: 9001,
      name: "test data".to_string(),
    };
    let client = rusoto_sqs::SqsClient::new_with(
      rusoto_mock::MockRequestDispatcher::default(),
      rusoto_mock::MockCredentialsProvider,
      Default::default());
    let sqs = SQS::new(client);

    assert!(sqs.perform(Enqueue(data)).is_ok());
  }
}
