use crate::Result;

pub struct DistributedTransaction {}

impl DistributedTransaction {
    pub fn new() -> Self {
        Self {}
    }

    pub fn commit(&self) -> Result<()> {
        Ok(())
    }

    pub fn prepare(&self) -> Result<()> {
        Ok(())
    }
}
