use std::path::PathBuf;
use clap::{Parser, Subcommand};
use crate::operations;

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
            #[arg(short, long, value_name = "TYPE", value_name = "PATH")]
            type_: String,
            path: PathBuf,
    },
    CSV {
        #[arg(short, long, value_name = "FILE DM", value_name = "FILE TF", value_name = "FILE DT")]
        datamodel: PathBuf,
        transform: PathBuf,
        data: PathBuf,
    },
    XLSX {
        #[arg(short, long, value_name = "RETURN", value_name = "FILE DM", value_name = "FILE TF", value_name = "FILE DT")]
        return_format: String,
        datamodel: PathBuf,
        transform: PathBuf,
        data: PathBuf,
    }
}
pub fn read_in() -> () {
    let cli = Cli::parse();
    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    /*
    match cli.debug {
        0 => println!("Debug mode if off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode in on"),
        _ => println!("Don't be crazy"),
    }
     */

// You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::VALIDATE { path, type_}) => {
            // in CLion: '-- validate --path datamodel.hcl datamodel'
            println!("[validate] path: {:?}, file_type: {:?}", path, type_);
            match type_.as_str() {
                "datamodel"=> {
                    operations::validate_datamodel(path);
                },
                "transform"=> {
                    operations::validate_transform(path);
                },
                &_ => {
                    println!("unknown sub-command '{}'. Valid subcommands are: 'datamodel' and 'transform'", type_);
                } }
            },
        Some(Commands::CSV {datamodel, transform, data}) => {
            println!("Csv: project {:?}, transform: {:?}, data: {:?}", datamodel, transform, data);
            println!("not implemented");
        },
        Some(Commands::XLSX { return_format, datamodel, transform, data }) => {
            println!("[Xlsx] datamodel: {:?}, transform: {:?}, data: {:?}, return: {:?}", datamodel, transform, data, return_format);
            match return_format.as_str() {
                "parquet" => operations::export_parquet(data, datamodel, transform),
                "csv" => operations::export_csv(data, datamodel, transform),
                _ => println!("unknown return-format '{}'. Only 'parquet' and 'csv' are valid formats.", return_format) }
        },
        None => println!("Command '{:?}' does not exist: Commands are 'validate', 'csv', 'xlsx'", cli.command),
    }
}



