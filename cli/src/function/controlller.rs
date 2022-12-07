use std::error::Error;

use super::generator::FunctionGenerator;

pub struct FunctionController {
    generator: FunctionGenerator
}

impl FunctionController {
    pub fn new() -> Self {
        let generator = FunctionGenerator::new();
        Self {
            generator
        }
    }

    pub fn create_function(&self, name: String) -> Result<(), Box<dyn Error>> {
        self.generator.generate_function(name, "rust".to_string()).unwrap();
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
