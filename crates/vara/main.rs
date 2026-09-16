use std::process::ExitCode;

mod cli;
mod luau;
mod require;
mod require_loader;

#[cfg(windows)]
mod win_compat;

fn main() -> ExitCode {
    #[cfg(windows)]
    win_compat::init();

    match cli::run() {
        Ok(_) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::FAILURE
        }
    }
}
