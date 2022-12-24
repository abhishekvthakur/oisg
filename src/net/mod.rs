use message_io::node::{
  self, NodeHandler, NodeListener
};

pub enum Message {
}

///
pub struct Network {
  handler: NodeHandler<Message>,
  listener: NodeListener<Message>
}

impl Network {
  fn new() {
  }
}
