/// The `Capability` trait is used to name, restrict, and compose functionality
/// that results in the mutation of or depends on external state, such as a database.
pub trait Capability<Operation> {
  type Output;

  fn perform(&self, op: Operation) -> Self::Output;
}

