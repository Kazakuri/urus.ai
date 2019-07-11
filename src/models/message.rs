#[derive(Serialize, Deserialize)]
pub enum MessageType {
    Error,
    Notice,
}

#[derive(Serialize, Deserialize)]
pub struct Message<'a> {
    pub message: &'a str,
    pub message_type: MessageType,
}
