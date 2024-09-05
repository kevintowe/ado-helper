use clap::{Args, Parser, Subcommand};
use log::info;
use std::path::PathBuf;

// I think these need to be declared for global use across all files
mod commands {
    pub mod ado_service;
    pub mod login;
    pub mod logout;
    pub mod org;
    pub mod util;
}

// where this is specific imports for use by the rest of the code in the main.rs file (I think)
use commands::{login, logout, org, util};

//
//
//
// App
//
//
//

#[derive(Debug, Parser)]
#[command(name = "ado-helper")]
#[command(
    author,
    version,
    about = "A utility tool for cloning lots of repos.",
    arg_required_else_help = true
)]
struct Cli {
    // #[clap(subcommand)]
    #[command(subcommand)]
    command: Commands,
    #[arg(
        long,
        global = true,
        help = "Run the CLI as if it had been started in another directory."
    )]
    cwd: Option<PathBuf>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Persist an Azure DevOps PAT for later use
    Login,
    /// Remove the persisted Azure DevOps PAT from storage
    Logout,
    Org(OrgArgs),
}

//
//
//
// org
//
//
//

#[derive(Debug, Args)]
#[command()]
struct OrgArgs {
    #[command(subcommand)]
    command: Option<OrgCommands>,
}

#[derive(Debug, Subcommand)]
enum OrgCommands {
    Add {
        /// The org name, as denoted in your ADO org url: `https://dev.azure.com/yourOrgNameHere`
        org_name: String,
    },
    /// Clear the persisted org name from storage
    Remove,
    /// Retrieve the persisted org name
    Get,
}

//
//
//
// repo
//
//
//

// #[derive(Subcommand)]
// enum RepoCommands {
//     /// A subcommand for repo's 'install' action
//     Install,
// }

//
//
//
// main
//
//
//
fn main() {
    env_logger::init();
    info!("Cli starting up");
    let args = Cli::parse();

    util::resolve_and_validate_cwd(args.cwd);

    match &args.command {
        Commands::Login {} => {
            login::run();
        }
        Commands::Logout {} => {
            logout::run();
        }
        Commands::Org(org) => {
            let org_cmd = org.command.as_ref().unwrap_or(&OrgCommands::Get);
            match org_cmd {
                OrgCommands::Add { org_name } => {
                    println!("Adding org name: {}", org_name);
                }
                OrgCommands::Remove {} => {
                    org::remove();
                }
                OrgCommands::Get {} => {
                    org::get();
                }
            }
        } //} // Commands::Repo { action } => match action {
          //     RepoActions::Clone { url } => {
          //         println!("Cloning repository from {}", url);
          //     }
          //     RepoActions::List => {
          //         println!("Listing all repositories");
          //     }
          // },
    }
}
