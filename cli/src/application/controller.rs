use std::error::Error;

pub struct ApplicationController {}

impl ApplicationController {
    pub fn new() -> Self {
        Self {}
    }

    pub fn create_application(&self, name: String) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    pub fn get_applications(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    pub fn push_application(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    pub fn deploy_application(&self, application_id: String) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    pub fn delete_application(&self, id: String) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
