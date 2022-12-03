use clap::{Parser, Subcommand};

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
        application_id: Option<String>,
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
        name: Option<String>,
    },
    Application {
        #[clap(value_parser)]
        name: Option<String>,
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
    match &cli.command {
        Some(Commands::Create { command }) => {
            if let Some(command) = command {
                match command {
                    CreateCommands::Function { name } => {
                        if let Some(name) = name {

                        }
                    },
                    CreateCommands::Application { name } => {
                        if let Some(name) = name {
                            
                        }
                    }
                }
            }
        }
        Some(Commands::Push) => {}
        Some(Commands::Get { command }) => {
            if let Some(command) = command {
                match command {
                    GetCommands::Functions => {}
                    GetCommands::Applications => {}
                }
            }
        }
        Some(Commands::Deploy { application_id }) => {
            if let Some(application_id) = application_id {

            }
        },
        Some(Commands::Login) => {}
        Some(Commands::Logout) => {}
        None => {
            println!("Lost? type `egnitely --help` to see list of commands")
        }
    }
}