use std::cell::RefCell;

use rusoto_core::RusotoFuture;
use rusoto_sqs::*;
  
#[derive(Clone)]
pub struct Message {
  pub body: String,
  pub queue: String,
}

pub struct MockSqs {
  pub messages: RefCell<Vec<Message>>,
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
