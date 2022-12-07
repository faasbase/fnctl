use std::error::Error;

pub struct AuthNController {}

impl AuthNController {
    pub fn new() -> Self {
        Self {}
    }

    pub fn login(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    pub fn logout(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
