mod application;
mod authn;
mod function;
mod workspace;

use authn::controller::AuthNController;
use clap::{Parser, Subcommand};

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
    /// List functions, and applications
    Get {
        #[clap(subcommand)]
        command: Option<GetCommands>,
    },
    /// Create a new wasmrpc function
    Create {
        #[clap(subcommand)]
        command: Option<CreateCommands>,
    },
    /// Generate function
    Deploy {
        #[clap(value_parser)]
        application_id: String,
    },
    /// Push the function/application to wasmrpc registory
    Push,
    /// Login to your wasmrpc registory
    Login,
    /// Logout of your wasmrpc registory
    Logout,
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
    /// Get list of providers
    Applications,
}

fn main() {
    let cli = Cli::parse();
    let function_controller = FunctionController::new();
    let application_controller = ApplicationController::new();
    let authn_controller = AuthNController::new();

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
            // TODO: Find out the current scope is application or function
        }
        Some(Commands::Get { command }) => {
            if let Some(command) = command {
                match command {
                    GetCommands::Functions => {
                        function_controller.get_functions().unwrap();
                    }
                    GetCommands::Applications => application_controller.get_applications().unwrap(),
                }
            }
        }
        Some(Commands::Deploy { application_id }) => {
            application_controller
                .delete_application(application_id.to_string())
                .unwrap();
        }
        Some(Commands::Login) => {
            authn_controller.login().unwrap();
        }
        Some(Commands::Logout) => {
            authn_controller.logout().unwrap();
        }
        None => {
            println!("Lost? type `faasly --help` to see list of commands")
        }
    }
}
