use std::path::PathBuf;
use clap::{Parser,Subcommand};
use clap::builder::styling;

mod runtime;
mod stdlib;

use runtime::luau;
use runtime::ui_manager;

fn get_style() -> styling::Styles {
    let accent = styling::Style::new().fg_color(Some(styling::RgbColor(0xb3,0x7d,0xf4).into()));
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
    version = "0.1.0",
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
    command : Commands
}

#[derive(Subcommand)]
enum Commands{
    Run(RunArgs),
    Test{}
}

#[derive(Parser)]
struct RunArgs{
    path : PathBuf
}

// if only windows used utf8 🥀
#[cfg(windows)]
mod handle_windows{
    use std::ffi::c_uint;
    const CP_UTF8: c_uint = 65001;

    #[cfg(windows)]
    unsafe extern "system" {
        fn SetConsoleOutputCP(wCodePageID: c_uint) -> i32;
    }

    pub fn init(){
        unsafe{SetConsoleOutputCP(CP_UTF8)};
    }
}

fn main() -> Result<(),Box<dyn std::error::Error>> {
    #[cfg(windows)]
    handle_windows::init();

    let cli = CLI::parse();

    match cli.command {
        Commands::Run(args) => {
            let mut current = luau::Runtime::new()?;
            let path = args.path;
            match current.run(&path) {
                Ok(()) => println!("file {} ran successfully!",path.display()),
                Err(err) => println!("file {} failed to run\n{}",path.display(),err)
            }
        },


        Commands::Test{} => {  
            let mut ui = ui_manager::UIManager::new();

            match ui.consume_file(&PathBuf::from("./demo_ui/test.slint")) {
                Ok(()) => println!("successfully compiled"),
                Err(err) => println!("failed to compile {}",err),
            }
        }
    }


    Ok(())
}
