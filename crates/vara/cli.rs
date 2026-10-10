use clap::builder::styling;
use clap::{CommandFactory, Parser, Subcommand};
use std::path::PathBuf;

use crate::{compile, luau};
use crate::setup;
fn get_style() -> styling::Styles {
    let accent = styling::Style::new().fg_color(Some(styling::RgbColor(0xb3, 0x7d, 0xf4).into()));
    let dimmed = styling::Style::new().fg_color(Some(styling::AnsiColor::BrightBlack.into()));

    styling::Styles::styled()
        .header(styling::Style::new())
        .usage(styling::Style::new())
        .literal(accent)
        .placeholder(dimmed)
        .error(accent)
        .valid(accent)
        .invalid(dimmed)
}

#[derive(Parser)]
#[command(
    name="vara",
    version = env!("CARGO_PKG_VERSION"),
    about = "A luau runtime for creating standalone applications.",
    styles = get_style(),
    help_template = "\
{about}

{usage-heading} {usage}

{all-args}
"
)]
struct CLI {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(trailing_var_arg = true)]
    Run(RunArgs),
    Setup,
    Compile(CompileArgs)
}

#[derive(clap::Args)]
struct RunArgs {
    path: PathBuf,

    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    args: Vec<String>,
}
#[derive(clap::Args)]

struct CompileArgs {
    path: PathBuf,
}

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = CLI::parse();

    match cli.command {
        Some(Commands::Run(args)) => {
            let path = args.path;
            let trailing_args = args.args;
            let mut current = luau::Runtime::new(trailing_args)?;
            
            match current.run(&path) {
                Ok(()) => Ok(()),
                Err(err) => Err(format!("file {} failed to run\n{}", path.display(), err).into()),
            }
        },
        Some(Commands::Setup) => {
            match setup::setup().await {
                Ok(()) => Ok(()),
                Err(err) => Err(format!("failed to setup luaurc:\n{}",err).into())
            }
        }

        Some(Commands::Compile(args)) => {
            let path = args.path;
            compile::compile(path).await
        }

        None => Ok(CLI::command().print_help()?),
    }
}
