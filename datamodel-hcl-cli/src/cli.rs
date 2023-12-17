use std::path::PathBuf;
use clap::{CommandFactory, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]

pub struct Cli {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count, default_value_t = 1)]
    debug: u8,

    ///takes an argument
    #[command(subcommand)]
    command: Option<Commands>,


}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Parses a HCL-Datamodel
    VALIDATE {
            /// the required path to the datamodel in hcl
            #[arg(short, long, value_name = "PROJECT", value_name = "TYPE")]
            project: PathBuf,
            type_: String,
    },
    CSV {
        #[arg(short, long, value_name = "FILE DM", value_name = "FILE TF", value_name = "FILE DT")]
        project: PathBuf,
        transform: PathBuf,
        data: PathBuf,
    },
    XLSX {
        #[arg(short, long, value_name = "FILE DM", value_name = "FILE TF", value_name = "FILE DT")]
        project: PathBuf,
        transform: PathBuf,
        data: PathBuf,
    }
}
pub fn read_in() -> () {
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
        Some(Commands::VALIDATE { project, type_}) => (
            // in CLion: '-- validate --project datamodel.hcl datamodel'
            println!("validate: project {:?}, file_type: {:?}", project, type_)
            ),
        Some(Commands::CSV {project, transform, data}) => (
            println!("Csv: project {:?}, transform: {:?}, data: {:?}", project, transform , data)
            ),
        Some(Commands::XLSX { project: project, transform, data}) => (
            println!("Xlsx: project {:?}, transform: {:?}, data: {:?}", project, transform, data)
        ),
        // Todo: return message "only the following commands have an effect: 'Import'" ?
        None => (println!("none: {:?}", cli.command)),
    }

}

