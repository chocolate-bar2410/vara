use std::path::PathBuf;
use clap::{Parser,Subcommand};

mod luau;
mod stdlib;

#[derive(Parser)]
#[command(version,name="vara")]
struct CLI {
    #[command(subcommand)]
    command : Commands
}

#[derive(Subcommand)]
enum Commands{
    Run(RunArgs)
}

#[derive(Parser)]
struct RunArgs{
    path : PathBuf
}

fn main() -> Result<(),Box<dyn std::error::Error>> {

    let cli = CLI::parse();

    match cli.command {
        Commands::Run(args) => {
            let mut current = luau::Runtime::new();
            let path = args.path;
            current.run(&path)?;
        }
    }


    Ok(())
}
