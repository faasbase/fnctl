pub mod types;

pub fn get_server_url() -> String {
    let app_env = std::env::var("APP_ENV");
    match app_env {
        Ok(app_env) => {
            if app_env == "dev" {
                return "https://api.faasly.dev".to_string();
            }
            "https://api.faasly.dev".to_string()
        }
        Err(_err) => "https://api.faasly.dev".to_string(),
    }
}