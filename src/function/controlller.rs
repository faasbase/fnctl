use super::{entities::CargoTomlSchema, generator::FunctionGenerator};
use crate::utils::error::CLIError;
use crate::{
    config::{get_server_url, types::FaasbaseResponse},
    function::entities::FunctionResponse,
};
use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
use prettytable::{row, Table};
use reqwest::{multipart, Body};
use semver::Version;
use serde_json::json;
use std::io::prelude::*;
use std::{
    error::Error,
    ffi::OsStr,
    fs::{self, File},
    path::{Component, Path},
};
use tokio::fs::File as AsyncFile;
use tokio_util::codec::{BytesCodec, FramedRead};
use walkdir::WalkDir;
use zip::write::FileOptions;

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
                home_dir.join(".faasbase").join("creds"),
                PickleDbDumpPolicy::DumpUponRequest,
                SerializationMethod::Json,
            ).unwrap();
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
                        .await.unwrap();
                    if get_functions_response.status().is_success() {
                        let get_functions: FaasbaseResponse<Vec<FunctionResponse>> =
                            get_functions_response.json().await.unwrap();

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

    pub async fn push_function(&self) -> Result<(), Box<dyn Error>> {
        if let Some(home_dir) = dirs::home_dir() {
            if !(Path::new(&home_dir.join(".faasbase")).is_dir()) {
                fs::create_dir(home_dir.join(".faasbase")).unwrap();
            }

            if !(Path::new(&home_dir.join(".faasbase/temp")).is_dir()) {
                fs::create_dir(home_dir.join(".faasbase/temp")).unwrap();
            }

            let contents = fs::read_to_string("Cargo.toml").unwrap();
            let data: CargoTomlSchema = toml::from_str(&contents).unwrap();

            if !(Path::new(&home_dir.join(format!(".faasbase/temp/{}", data.package.name))).is_dir())
            {
                fs::create_dir(home_dir.join(format!(".faasbase/temp/{}", data.package.name))).unwrap();
            }

            let zip_file = home_dir.join(format!(".faasbase/temp/{}.zip", data.package.name));
            let path = Path::new(&zip_file);
            let file = File::create(&path).unwrap();

            let walkdir = WalkDir::new("./");
            let it = walkdir.into_iter();

            let mut zip = zip::ZipWriter::new(file);
            let options = FileOptions::default()
                .compression_method(zip::CompressionMethod::Stored)
                .unix_permissions(0o755);

            let mut buffer = Vec::new();
            for entry in it.filter_map(|e| e.ok()) {
                let path = entry.path();
                let name = path.strip_prefix(Path::new("./")).unwrap();

                if path.is_file() {
                    if !(name.components().nth(0) == Some(Component::Normal(OsStr::new("target"))))
                    {
                        #[allow(deprecated)]
                        zip.start_file_from_path(name, options).unwrap();
                        let mut f = File::open(path).unwrap();

                        f.read_to_end(&mut buffer).unwrap();
                        zip.write_all(&*buffer).unwrap();
                        buffer.clear();
                    }
                } else if !name.as_os_str().is_empty() {
                    if !(name.components().nth(0) == Some(Component::Normal(OsStr::new("target"))))
                    {
                        #[allow(deprecated)]
                        zip.add_directory_from_path(name, options).unwrap();
                    }
                }
            }

            zip.finish().unwrap();

            let client = reqwest::Client::new();
            let db = PickleDb::load(
                home_dir.join(".faasbase").join("creds"),
                PickleDbDumpPolicy::DumpUponRequest,
                SerializationMethod::Json,
            ).unwrap();
            let access_token = db.get::<String>("access_token");
            let workspace_id = db.get::<String>("workspace_id");

            if let Some(access_token) = access_token {
                if let Some(workspace_id) = workspace_id {
                    let file = AsyncFile::open(zip_file.clone()).await.unwrap();

                    // read file body stream
                    let stream = FramedRead::new(file, BytesCodec::new());
                    let file_body = Body::wrap_stream(stream);

                    let function_file = multipart::Part::stream(file_body)
                        .file_name("function.zip")
                        .mime_str("application/octet-stream").unwrap();
                    let form = multipart::Form::new().part("file", function_file);

                    let get_function_response = client
                        .get(format!(
                            "{}/functions?name={}&workspace_id={}",
                            get_server_url(),
                            data.package.name.clone(),
                            workspace_id.clone()
                        ))
                        .header("Authorization", format!("Bearer {}", access_token))
                        .send()
                        .await.unwrap();

                    if get_function_response.status().is_success() {
                        let get_function: FaasbaseResponse<FunctionResponse> =
                            get_function_response.json().await.unwrap();
                        if let Some(latest_version) = get_function.data.latest_version {
                            let current_version = Version::parse(&data.package.version.clone()).unwrap();
                            let latest_version = Version::parse(&latest_version).unwrap();
                            if current_version.le(&latest_version) {
                                return Err(CLIError::new(
                                    "VERSION_ERROR".to_string(),
                                    format!("Function's version is less than or equal to last build version, please bump up the version before pushing it"),
                                ));
                            }
                        }

                        let _upload_response = client
                            .post(format!(
                                "{}/functions/{}/upload",
                                get_server_url(),
                                get_function.data.id
                            ))
                            .query(&[
                                ("version", data.package.version.clone()),
                                ("workspace_id", workspace_id.to_string()),
                            ])
                            .header("Authorization", format!("Bearer {}", access_token))
                            .multipart(form)
                            .send()
                            .await.unwrap();

                        if !_upload_response.status().is_success() {
                            return Err(CLIError::new(
                                "UPLOAD_ERROR".to_string(),
                                format!(
                                    "Unable to upload a function, Error: {:?}",
                                    _upload_response.text().await.unwrap()
                                ),
                            ));
                        }
                    } else {
                        let visibility  = match data.package.visibility {
                            Some(visibility) => visibility,
                            None => "PUBLIC".to_string(),
                        };

                        let create_function_response = client
                            .post(format!("{}/functions", get_server_url()))
                            .header("Authorization", format!("Bearer {}", access_token))
                            .json(&json! {{
                                "name": data.package.name.clone(),
                                "visibility": visibility,
                                "description": data.package.description.clone(),
                                "latest_version": data.package.version.clone(),
                                "workspace_id": workspace_id,
                            }})
                            .send()
                            .await.unwrap();

                        if create_function_response.status().is_success() {
                            let create_function: FaasbaseResponse<FunctionResponse> =
                                create_function_response.json().await.unwrap();
                            let _upload_response = client
                                .post(format!(
                                    "{}/functions/{}/upload",
                                    get_server_url(),
                                    create_function.data.id
                                ))
                                .query(&[
                                    ("version", data.package.version.clone()),
                                    ("workspace_id", workspace_id),
                                ])
                                .header("Authorization", format!("Bearer {}", access_token))
                                .multipart(form)
                                .send()
                                .await.unwrap();

                            if !_upload_response.status().is_success() {
                                return Err(CLIError::new(
                                    "UPLOAD_ERROR".to_string(),
                                    format!(
                                        "Unable to upload a function, Error: {:?}",
                                        _upload_response.text().await.unwrap()
                                    ),
                                ));
                            }
                        } else {
                            return Err(CLIError::new(
                                "CREATE_FUNCTION_ERROR".to_string(),
                                format!(
                                    "Unable to create function `{}`, Status: {:?}, Error: {:?}",
                                    data.package.name,
                                    create_function_response.status(),
                                    create_function_response.text().await.unwrap()
                                ),
                            ));
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
