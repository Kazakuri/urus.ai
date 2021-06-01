#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum MessageType {
  Error,
  Notice,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct Message<'a> {
  pub message: &'a str,
  pub message_type: MessageType,
}
