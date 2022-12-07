use std::error::Error;

pub struct FunctionController {}

impl FunctionController {
    pub fn new() -> Self {
        Self {}
    }

    pub fn create_function(&self, name: String) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    pub fn get_functions(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    pub fn push_function(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    pub fn delete_function(&self, id: String) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
