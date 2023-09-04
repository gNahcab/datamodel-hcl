use datamodel_hcl::errors::DatamodelHCLError;
use std::path::PathBuf;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    ///takes an argument
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Parses an HCL-Datamodel
    Import {
            /// the required path to the datamodel in hcl
            #[arg(short, long, value_name = "FILE")]
            project: PathBuf,
    },
}
pub fn read_in() -> Result<(),DatamodelHCLError> {
    let cli = Cli::parse();
    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match cli.debug {
        0 => println!("Debug mode if off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode in on"),
        _ => println!("Don't be crazy"),
    }
// You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Import { project}) => datamodel_hcl::operations::import(project),
        None => Ok(()),
    }

}

