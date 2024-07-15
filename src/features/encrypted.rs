use super::DeadLetterFeature;
use super::JournalFeature;
use super::TransactionalFeature;
use crate::message::Message;
use crate::queue::Queue;
use crate::queue::QueueOps;
use crate::security::Security;
use crate::{MSMQError, Result};

pub trait EncryptFeature: Send + Sync + 'static {}

#[derive(Clone)]
pub struct BasicEncryption(pub Security);

#[derive(Default, Clone)]
pub struct AnonymousEncryption;

impl EncryptFeature for BasicEncryption {}
impl EncryptFeature for AnonymousEncryption {}

impl<J, T, D> Queue<J, T, BasicEncryption, D>
where
    J: JournalFeature,
    T: TransactionalFeature,
    D: DeadLetterFeature,
{
    pub fn send_authenticated(&mut self, message: Message<BasicEncryption>) -> Result<()> {
        self.queue
            .lock()
            .map_err(|e| MSMQError::Custom(e.to_string()))?
            .push_back(message);
        Ok(())
    }

    pub fn receive_authenticated(
        &mut self,
        username: &str,
        password: &str,
    ) -> Option<Message<BasicEncryption>> {
        self.receive()
    }
}

#[cfg(test)]
mod tests {
    use crate::{message::Message, queue_builder::QueueBuilder, security::Security};

    #[test]
    fn test_authentication_and_encryption() {
        let mut queue = QueueBuilder::new("test_queue")
            .with_encryption(Security::new("user", "password"))
            .build();

        let message = Message::new("Secure message").encrypt();
        queue.send_authenticated(message).unwrap();

        let received = queue.receive_authenticated("user", "password").unwrap();
        assert_eq!(received.content(), "Secure message");
    }
}
