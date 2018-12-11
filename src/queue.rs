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
