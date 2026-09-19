use std::process::ExitCode;

mod cli;
mod luau;
mod require;
mod require_loader;

fn main() -> ExitCode {
    #[cfg(windows)]
    vara_util::win_compat::init();

    match cli::run() {
        Ok(_) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::FAILURE
        }
    }
}
