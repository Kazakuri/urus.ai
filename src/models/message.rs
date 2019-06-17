#[derive(Serialize, Deserialize)]
pub enum MessageType {
    Error,
}

#[derive(Serialize, Deserialize)]
pub struct Message<'a> {
    pub message: &'a str,
    pub message_type: MessageType,
}
