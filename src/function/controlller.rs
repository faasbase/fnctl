use std::{error::Error, path::Path, fs};

use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
use prettytable::{row, Table};

use crate::{
    config::{get_server_url, types::FaaslyResponse},
    function::entities::FunctionResponse,
};

use super::generator::FunctionGenerator;

pub struct FunctionController {
    generator: FunctionGenerator,
}

impl FunctionController {
    pub fn new() -> Self {
        let generator = FunctionGenerator::new();
        Self { generator }
    }

    pub fn create_function(&self, name: String) -> Result<(), Box<dyn Error>> {
        self.generator
            .generate_function(name, "rust".to_string())
            .unwrap();
        Ok(())
    }

    pub async fn get_functions(&self) -> Result<(), Box<dyn Error>> {
        println!("");
        let client = reqwest::Client::new();
        if let Some(home_dir) = dirs::home_dir() {
            let db = PickleDb::load(
                home_dir.join(".faasly").join("creds"),
                PickleDbDumpPolicy::DumpUponRequest,
                SerializationMethod::Json,
            )?;
            let access_token = db.get::<String>("access_token");
            let workspace_id = db.get::<String>("workspace_id");

            if let Some(access_token) = access_token {
                if let Some(workspace_id) = workspace_id {
                    let get_functions_response = client
                        .get(format!(
                            "{}/functions?workspace_id={}",
                            get_server_url(),
                            workspace_id
                        ))
                        .header("Authorization", format!("Bearer {}", access_token))
                        .send()
                        .await?;
                    if get_functions_response.status().is_success() {
                        let get_functions: FaaslyResponse<Vec<FunctionResponse>> =
                            get_functions_response.json().await?;

                        let mut table = Table::new();

                        // Add a row per time
                        table.add_row(row!["ID", "NAME", "CREATED AT"]);
                        for function in get_functions.data {
                            table.add_row(row![function.id, function.name, function.created_at]);
                        }
                        table.printstd();
                    } else {
                        println!("No functions found, please check if you are logged in");
                    }
                } else {
                    println!("Bad workspace id, please check if its configured correctly")
                }
            } else {
                println!("Bad access_token, please check if you are logged in")
            }
        }
        println!("");
        Ok(())
    }

    pub fn push_function(&self) -> Result<(), Box<dyn Error>> {
        if let Some(home_dir) = dirs::home_dir() {
            if !(Path::new(&home_dir.join(".faasly")).is_dir()) {
                fs::create_dir(home_dir.join(".faasly"))?;
            }

            if !(Path::new(&home_dir.join(".faasly/.temp")).is_dir()) {
                fs::create_dir(home_dir.join(".faasly/.temp"))?;
            }

            



        }
        Ok(())
    }
}
