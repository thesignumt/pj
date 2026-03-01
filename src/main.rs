use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "pj_cli")]
#[command(about = "harpoon from neovim but for projects")]
#[command(version = "0.1.0")]
struct Cli {
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a project directory
    #[command(alias = "a", alias = "append")]
    Add {
        /// Path to the project directory (defaults to current dir)
        #[arg(default_value = ".")]
        path: String,
    },

    /// Remove a project directory
    #[command(alias = "d", alias = "rm")]
    Remove {
        /// Path to the project directory (defaults to current dir)
        #[arg(default_value = ".")]
        path: String,
    },

    /// Open GUI
    // #[command(alias = "g", alias = "s", alias = "switch")]
    // Open,

    /// List projects
    #[command(alias = "l", alias = "ls")]
    List,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { path } => {
            println!("Added project: {}", path);
        }
        Commands::Remove { path } => {
            println!("Removed project: {}", path);
        }
        Commands::List => {
            println!("Listing projects...");
        }
    }
}
