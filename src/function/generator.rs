use std::{error::Error, fs::{self, File}};
use std::io::prelude::*;

pub struct FunctionGenerator {}

impl FunctionGenerator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn generate_function(&self, name: String, language: String) -> Result<(), Box<dyn Error>> {

        match language.as_str() {
            "rust" => {
                self.generate_rust_function(name)?;
            }
            _ => {
                println!("Unable to create function because language was unknown");
            }
        }

        Ok(())
    }

    pub fn generate_rust_function(&self, name: String) -> Result<(), Box<dyn Error>> {
        println!("Generating Rust function: {}", name);
        let lib_rs = "use std::error::Error;\n\nuse schemars::JsonSchema;\nuse serde::{Deserialize, Serialize};\nuse serde_json::{Value};\n\n// TODO: replace the fields in this struct with your own input data structure\n#[derive(Serialize, Deserialize, JsonSchema, Debug)]\npub struct Input {\n\tpub firstname: String,\n\tpub lastname: String,\n}\n\n// TODO: replace the fields in this struct with your own output data structure\n#[derive(Serialize, Deserialize, JsonSchema, Debug)]\npub struct Output {\n\tpub name: String,\n\tpub data: Value,\n}\n\npub fn handler(Json(data): Json<Input>) -> impl IntoResponse {\n\t// TODO: implement your function here\n\n\tlet output = Output {\n\t\tname: \"hello\".to_string(),\n\t\tdata: serde_json::to_value(data)?,\n\t};\n\t(\n\t\tStatusCode::OK,\n\t\tJson(json!({\n\t\t\t\"status\": \"ok\",\n\t\t\t\"data\": output,\n\t\t})),\n)\n}\n".to_string();

        let mut cargo_toml = "[package] \nname = \"".to_string();
        cargo_toml.push_str(&name);
        cargo_toml.push_str("\"\ndescription = \"Replace this with your function description\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html\n\n\n[dependencies]\nserde = { version = \"1.0\", features = [\"derive\"] }\nserde_json = \"1.0\"\nschemars = \"0.8.11\"\n");

        let gitignore = "/target\n/Cargo.lock";

        let mut folder_name = name.clone();
        folder_name.push_str("/src");

        fs::create_dir(name.clone())?;
        fs::create_dir(folder_name.clone())?;

        let mut lib_rs_filepath = folder_name.clone();
        lib_rs_filepath.push_str("/lib.rs");
        let mut lib_rs_file = File::create(lib_rs_filepath)?;
        lib_rs_file.write_all(lib_rs.as_bytes())?;

        let mut cargo_toml_filepath = name.clone();
        cargo_toml_filepath.push_str("/Cargo.toml");
        let mut cargo_toml_file = File::create(cargo_toml_filepath)?;
        cargo_toml_file.write_all(cargo_toml.as_bytes())?;

        let mut gitignore_filepath = name.clone();
        gitignore_filepath.push_str("/.gitignore");
        let mut gitignore_file = File::create(gitignore_filepath)?;
        gitignore_file.write_all(gitignore.as_bytes())?;
        
        println!("Function generated successfully");
        Ok(())
    }

    pub fn _generate_rust_function_wrapper(&self, name: String, description: String) -> Result<(), Box<dyn Error>> {
        let lib_rs = "".to_string();

        let mut cargo_toml = "[package] \nname = \"".to_string();
        cargo_toml.push_str(&name);
        cargo_toml.push_str("\"\ndescription = \"");
        cargo_toml.push_str(&description);
        cargo_toml.push_str("\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html\n\n\n[dependencies]\nserde = { version = \"1.0\", features = [\"derive\"] }\nserde_json = \"1.0\"\nschemars = \"0.8.11\"\n");

        let gitignore = "/target\n/Cargo.lock";

        let mut folder_name = name.clone();
        folder_name.push_str("/src");

        fs::create_dir(name.clone())?;
        fs::create_dir(folder_name.clone())?;

        let mut lib_rs_filepath = folder_name.clone();
        lib_rs_filepath.push_str("/lib.rs");
        let mut lib_rs_file = File::create(lib_rs_filepath)?;
        lib_rs_file.write_all(lib_rs.as_bytes())?;

        let mut cargo_toml_filepath = name.clone();
        cargo_toml_filepath.push_str("/Cargo.toml");
        let mut cargo_toml_file = File::create(cargo_toml_filepath)?;
        cargo_toml_file.write_all(cargo_toml.as_bytes())?;

        let mut gitignore_filepath = name.clone();
        gitignore_filepath.push_str("/.gitignore");
        let mut gitignore_file = File::create(gitignore_filepath)?;
        gitignore_file.write_all(gitignore.as_bytes())?;
        
        println!("Function generated successfully");
        Ok(())
    }
}
