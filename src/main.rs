mod application;
mod authn;
mod function;
mod workspace;
mod config;
mod utils;

use authn::controller::AuthNController;
use clap::{Parser, Subcommand};
use workspace::controller::WorkspaceController;

use crate::{
    application::controller::ApplicationController, function::controlller::FunctionController,
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Login to registry (Default: https://faasbase.com)
    Login,
    /// Logout of registry
    Logout,
    /// List functions, workspaces and applications from repository
    Get {
        #[clap(subcommand)]
        command: Option<GetCommands>,
    },
    /// Configure your default workspace
    ConfigureWorkspace {
        #[clap(value_parser)]
        workspace_id: String,
    },
    /// Push function or application to registry
    Push,
    /// Create new function or application
    Create {
        #[clap(subcommand)]
        command: Option<CreateCommands>,
    },
    /// Deploy application to cluster
    Deploy {
        #[clap(value_parser)]
        application_id: String,
    },
}

#[derive(Subcommand)]
enum CreateCommands {
    /// Generate function
    Function {
        #[clap(value_parser)]
        name: String,
    },
    Application {
        #[clap(value_parser)]
        name: String,
    },
}

#[derive(Subcommand)]
enum GetCommands {
    /// Get list of functions
    Functions,
    /// Get list of applications
    Applications,
    /// Get list of workspaces
    Workspaces,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let function_controller = FunctionController::new();
    let application_controller = ApplicationController::new();
    let authn_controller = AuthNController::new();
    let workspace_controller = WorkspaceController::new();

    match &cli.command {
        Some(Commands::Create { command }) => {
            if let Some(command) = command {
                match command {
                    CreateCommands::Function { name } => {
                        function_controller
                            .create_function(name.to_string())
                            .unwrap();
                    }
                    CreateCommands::Application { name } => application_controller
                        .create_application(name.to_string())
                        .unwrap(),
                }
            }
        }
        Some(Commands::Push) => {
            function_controller.push_function().await.unwrap();
        }
        Some(Commands::Get { command }) => {
            if let Some(command) = command {
                match command {
                    GetCommands::Functions => {
                        function_controller.get_functions().await.unwrap();
                    }
                    GetCommands::Applications => application_controller.get_applications().await.unwrap(),
                    GetCommands::Workspaces => workspace_controller.get_workspaces().await.unwrap(),
                }
            }
        }
        Some(Commands::Deploy { application_id }) => {
            application_controller
                .deploy_application(application_id.to_string())
                .unwrap();
        }
        Some(Commands::ConfigureWorkspace { workspace_id }) => {
            workspace_controller
                .configure_workspace(workspace_id.to_string())
                .unwrap();
        }
        Some(Commands::Login) => {
            authn_controller.login().await.unwrap();
        }
        Some(Commands::Logout) => {
            authn_controller.logout().await.unwrap();
        }
        None => {
            println!("Lost? type `faasbase --help` to see list of commands")
        }
    }
}
