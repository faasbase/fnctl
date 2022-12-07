use std::error::Error;

pub struct WorkspaceController {}

impl WorkspaceController {
    pub fn new() -> Self {
        Self {}
    }

    pub fn configure_workspace(&self, name: String) -> Result<(), Box<dyn Error>> {
         println!("Configuring workspace: {}", name);
        Ok(())
    }

    pub fn get_workspaces(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
