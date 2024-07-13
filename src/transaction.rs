use crate::Result;

pub struct Transaction {}

impl Transaction {
    pub fn new() -> Self {
        Self {}
    }

    pub fn commit(&self) -> Result<()> {
        Ok(())
    }
}
