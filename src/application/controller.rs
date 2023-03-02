use std::error::Error;

use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
use prettytable::{Table, row};

use crate::{config::{get_server_url, types::FaasbaseResponse}, application::entities::ApplicationResponse};

pub struct ApplicationController {}

impl ApplicationController {
    pub fn new() -> Self {
        Self {}
    }

    pub fn create_application(&self, name: String) -> Result<(), Box<dyn Error>> {
        println!("Creating application: {}", name);
        Ok(())
    }

    pub async fn get_applications(&self) -> Result<(), Box<dyn Error>> {
        println!("");
        let client = reqwest::Client::new();
        if let Some(home_dir) = dirs::home_dir() {
            let db = PickleDb::load(
                home_dir.join(".faasbase").join("creds"),
                PickleDbDumpPolicy::DumpUponRequest,
                SerializationMethod::Json,
            )?;
            let access_token = db.get::<String>("access_token");
            let workspace_id = db.get::<String>("workspace_id");
            if let Some(access_token) = access_token {
                if let Some(workspace_id) = workspace_id {
                    let get_applications_response = client
                        .get(format!(
                            "{}/applications?workspace_id={}",
                            get_server_url(),
                            workspace_id
                        ))
                        .header("Authorization", format!("Bearer {}", access_token))
                        .send()
                        .await?;
                    if get_applications_response.status().is_success() {
                        let get_applications: FaasbaseResponse<Vec<ApplicationResponse>> =
                            get_applications_response.json().await?;

                        let mut table = Table::new();

                        // Add a row per time
                        table.add_row(row!["ID", "NAME", "CREATED AT"]);
                        for application in get_applications.data {
                            table.add_row(row![
                                application.id,
                                application.name,
                                application.created_at
                            ]);
                        }
                        table.printstd();
                    } else {
                        println!("No applications found, please check if you are logged in");
                    }
                } else {
                    println!("No workspace configured, please configure a workspace");
                }
            } else {
                println!("No access token found, please login");
            }
        }
        println!("");
        Ok(())
    }

    pub fn deploy_application(&self, application_id: String) -> Result<(), Box<dyn Error>> {
        println!("Deploying application: {}", application_id);
        Ok(())
    }
}
