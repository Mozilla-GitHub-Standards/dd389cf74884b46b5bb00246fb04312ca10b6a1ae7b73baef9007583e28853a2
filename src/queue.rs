use serde::Serialize;
use serde_derive::Serialize;

use crate::capability::Capability;


/// An operation for enqueuing data e.g. into SQS.
pub struct Enqueue<T>(T);

/// Errors that may be encountered trying to queue an event.
pub enum EnqueueError{}

/// SQS supports enqueueing event data to Amazon SQS.
pub struct SQS {
}

impl SQS {
  /// Construct a new SQS configuration.
  pub fn new() -> Self {
    SQS{}
  }
}

impl<T> Capability<Enqueue<T>> for SQS
  where T: Serialize
{
  type Output = Result<(), EnqueueError>;

  fn perform(&self, op: Enqueue<T>) -> Self::Output {
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use rusoto_mock;
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
    let sqs = SQS::new();

    assert!(sqs.perform(Enqueue(data)).is_ok());
  }
}
