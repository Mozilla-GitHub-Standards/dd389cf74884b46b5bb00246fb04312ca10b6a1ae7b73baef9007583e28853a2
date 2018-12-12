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
pub struct Enqueue<T>(T);

/// Errors that may be encountered trying to queue an event.
pub enum EnqueueError{}

/// SQS supports enqueueing event data to Amazon SQS.
pub struct SQS<S> {
  client: S,
  queue: String,
}

impl<S> SQS<S> {
  /// Construct a new SQS configuration.
  pub fn new(client: S, queue_url: String) -> Self {
    SQS{
      client: client,
      queue: queue_url,
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

#[cfg(test)]
mod tests {
  use super::*;
  
  use std::cell::RefCell;

  use rusoto_core::RusotoFuture;
  use rusoto_sqs::*;
  use serde_derive::{Deserialize, Serialize};


  #[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
  struct TestData {
    pub id: u32,
    pub name: String,
  }

  #[derive(Clone)]
  struct Message {
    pub body: String,
    pub queue: String,
  }

  struct MockSqs {
    pub messages: RefCell<Vec<Message>>,
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
    assert_eq!(client.messages.borrow().len(), 1);

    let Message{ body, queue } = client.messages.borrow()[0].clone(); 
    let recvd: TestData = serde_json::from_str(&body).unwrap();

    assert_eq!(queue, "queue_name".to_string());
    assert_eq!(recvd, data);
  }

  impl MockSqs {
    pub fn new() -> Self {
      MockSqs {
        messages: RefCell::new(Vec::new()),
      }
    }
  }

  impl Sqs for &MockSqs {
    fn add_permission(
      &self,
      _: AddPermissionRequest
    ) -> RusotoFuture<(), AddPermissionError> 
    {
      From::from(Err(AddPermissionError::Validation("not implemented".to_string())))
    }
    
    fn change_message_visibility(
      &self,
      _: ChangeMessageVisibilityRequest
    ) -> RusotoFuture<(), ChangeMessageVisibilityError> 
    {
      From::from(Err(ChangeMessageVisibilityError::Validation("not implemented".to_string())))
    }

    fn change_message_visibility_batch(
      &self,
      _: ChangeMessageVisibilityBatchRequest
    ) -> RusotoFuture<
      ChangeMessageVisibilityBatchResult,
      ChangeMessageVisibilityBatchError> 
    {
      From::from(Err(ChangeMessageVisibilityBatchError::Validation("not implemented".to_string())))
    }

    fn create_queue(
      &self,
      _: CreateQueueRequest
    ) -> RusotoFuture<CreateQueueResult, CreateQueueError>
    {
      From::from(Err(CreateQueueError::Validation("not implemented".to_string())))
    }

    fn delete_message(
      &self,
      _: DeleteMessageRequest
    ) -> RusotoFuture<(), DeleteMessageError>
    {
      From::from(Err(DeleteMessageError::Validation("not implemented".to_string())))
    }

    fn delete_message_batch(
      &self,
      _: DeleteMessageBatchRequest
    ) -> RusotoFuture<DeleteMessageBatchResult, DeleteMessageBatchError>
    {
      From::from(Err(DeleteMessageBatchError::Validation("not implemented".to_string())))
    }

    fn delete_queue(
      &self,
      _: DeleteQueueRequest
    ) -> RusotoFuture<(), DeleteQueueError>
    {
      From::from(Err(DeleteQueueError::Validation("not implemented".to_string())))
    }

    fn get_queue_attributes(
      &self,
      _: GetQueueAttributesRequest
    ) -> RusotoFuture<GetQueueAttributesResult, GetQueueAttributesError>
    {
      From::from(Err(GetQueueAttributesError::Validation("not implemented".to_string())))
    }

    fn get_queue_url(
      &self,
      _: GetQueueUrlRequest
    ) -> RusotoFuture<GetQueueUrlResult, GetQueueUrlError>
    {
      From::from(Err(GetQueueUrlError::Validation("not implemented".to_string())))
    }

    fn list_dead_letter_source_queues(
      &self,
      _: ListDeadLetterSourceQueuesRequest
    ) -> RusotoFuture<ListDeadLetterSourceQueuesResult, ListDeadLetterSourceQueuesError>
    {
      From::from(Err(ListDeadLetterSourceQueuesError::Validation("not implemented".to_string())))
    }

    fn list_queue_tags(
      &self,
      _: ListQueueTagsRequest
    ) -> RusotoFuture<ListQueueTagsResult, ListQueueTagsError>
    {
      From::from(Err(ListQueueTagsError::Validation("not implemented".to_string())))
    }

    fn list_queues(
      &self,
      _: ListQueuesRequest
    ) -> RusotoFuture<ListQueuesResult, ListQueuesError>
    {
      From::from(Err(ListQueuesError::Validation("not implemented".to_string())))
    }

    fn purge_queue(
      &self,
      _: PurgeQueueRequest
    ) -> RusotoFuture<(), PurgeQueueError>
    {
      From::from(Err(PurgeQueueError::Validation("not implemented".to_string())))
    }

    fn receive_message(
      &self, 
      _: ReceiveMessageRequest
    ) -> RusotoFuture<ReceiveMessageResult, ReceiveMessageError>
    {
      From::from(Err(ReceiveMessageError::Validation("not implemented".to_string())))
    }

    fn remove_permission(
      &self,
      _: RemovePermissionRequest
    ) -> RusotoFuture<(), RemovePermissionError>
    {
      From::from(Err(RemovePermissionError::Validation("not implemented".to_string())))
    }

    fn send_message(
      &self,
      req: SendMessageRequest
    ) -> RusotoFuture<SendMessageResult, SendMessageError>
    {
      let mut received = self.messages.borrow_mut();
      received.push(Message {
        body: req.message_body,
        queue: req.queue_url,
      });
      From::from(Ok(Default::default()))
    }

    fn send_message_batch(
      &self,
      _: SendMessageBatchRequest
    ) -> RusotoFuture<SendMessageBatchResult, SendMessageBatchError>
    {
      From::from(Err(SendMessageBatchError::Validation("not implemented".to_string())))
    }

    fn set_queue_attributes(
      &self,
      _: SetQueueAttributesRequest
    ) -> RusotoFuture<(), SetQueueAttributesError>
    {
      From::from(Err(SetQueueAttributesError::Validation("not implemented".to_string())))
    }

    fn tag_queue(&self, input: TagQueueRequest) -> RusotoFuture<(), TagQueueError> {
      From::from(Err(TagQueueError::Validation("not implemented".to_string())))
    }

    fn untag_queue(
      &self, 
      _: UntagQueueRequest
    ) -> RusotoFuture<(), UntagQueueError>
    {
      From::from(Err(UntagQueueError::Validation("not implemented".to_string())))
    }
  }
}
