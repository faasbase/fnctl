use std::error::Error;

use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
use prettytable::{row, Table};

use crate::config::{
    get_server_url,
    types::{FaaslyResponse, WorkspaceResponse},
};

pub struct WorkspaceController {}

impl WorkspaceController {
    pub fn new() -> Self {
        Self {}
    }

    pub fn configure_workspace(&self, workspace_id: String) -> Result<(), Box<dyn Error>> {
        println!("Configuring workspace: {}", workspace_id);
        if let Some(home_dir) = dirs::home_dir() {
            let mut db = PickleDb::load(
                home_dir.join(".faasly").join("creds"),
                PickleDbDumpPolicy::AutoDump,
                SerializationMethod::Json,
            )?;
            db.set("workspace_id", &workspace_id)?;
        }
        Ok(())
    }

    pub async fn get_workspaces(&self) -> Result<(), Box<dyn Error>> {
        println!("");
        let client = reqwest::Client::new();
        if let Some(home_dir) = dirs::home_dir() {
            let db = PickleDb::load(
                home_dir.join(".faasly").join("creds"),
                PickleDbDumpPolicy::DumpUponRequest,
                SerializationMethod::Json,
            )?;
            let access_token = db.get::<String>("access_token");
            if let Some(access_token) = access_token {
                let get_workspaces_response = client
                    .get(format!("{}/workspaces", get_server_url()))
                    .header("Authorization", format!("Bearer {}", access_token))
                    .send()
                    .await?;

                if get_workspaces_response.status().is_success() {
                    let get_workspaces: FaaslyResponse<Vec<WorkspaceResponse>> =
                        get_workspaces_response.json().await?;

                    let mut table = Table::new();

                    // Add a row per time
                    table.add_row(row!["ID", "NAME", "CREATED AT"]);
                    for function in get_workspaces.data {
                        table.add_row(row![function.id, function.name, function.created_at]);
                    }
                    table.printstd();
                } else {
                    println!("No workspaces found, please check if you are logged in");
                }
            }
        }
        println!("");
        Ok(())
    }
}
