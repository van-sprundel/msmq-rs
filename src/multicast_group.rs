use crate::message::Message;
use crate::Result;

pub struct MulticastGroup {
    name: String,
}

impl MulticastGroup {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }

    pub fn send(&self, message: Message) -> Result<()> {
        Ok(())
    }
}
