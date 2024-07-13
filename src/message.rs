use crate::features::*;

#[derive(Default, Clone)]
pub struct Message<E = AnonymousEncryption> {
    content: String,
    state: std::marker::PhantomData<E>,
}

impl Message<BasicEncryption> {
    pub fn decrypt(self) -> Message<AnonymousEncryption> {
        Message {
            content: self.content,
            state: std::marker::PhantomData,
        }
    }
}

impl Message<AnonymousEncryption> {
    pub fn encrypt(self) -> Message<BasicEncryption> {
        Message {
            content: self.content,
            state: std::marker::PhantomData,
        }
    }
}

impl<E> Message<E> {
    pub fn new(content: &str) -> Self {
        Self {
            content: content.to_string(),
            state: std::marker::PhantomData,
        }
    }

    pub fn content(&self) -> &String {
        &self.content
    }
}
