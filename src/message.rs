use crate::features::*;

#[derive(Default, Clone)]
pub enum Priority {
    High,
    #[default]
    Low,
}

#[derive(Clone)]
pub struct Message<E = NonEncrypted> {
    content: String,
    priority: Priority,
    state: std::marker::PhantomData<E>,
}

impl Message<Encrypted> {
    pub fn decrypt(self) -> Message<NonEncrypted> {
        Message {
            content: self.content,
            priority: self.priority,
            state: std::marker::PhantomData::<NonEncrypted>,
        }
    }
}

impl Message<NonEncrypted> {
    pub fn encrypt(self) -> Message<Encrypted> {
        Message {
            content: self.content,
            priority: self.priority,
            state: std::marker::PhantomData::<Encrypted>,
        }
    }
}

impl<E> Message<E> {
    pub fn new(content: &str) -> Self {
        Self {
            content: content.to_string(),
            priority: Priority::default(),
            state: std::marker::PhantomData::<E>,
        }
    }

    pub fn with_priority(mut self, priority: Priority) -> Self {
        self.priority = priority;
        self
    }

    pub fn content(&self) -> &String {
        &self.content
    }
}
