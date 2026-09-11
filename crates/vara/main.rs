use std::process::ExitCode;

mod cli;

#[cfg(windows)]
mod win_compat;

fn main() -> ExitCode {
    #[cfg(windows)]
    win_compat::init();

    match cli::run() {
        Ok(_) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("error:\n {err:?}");
            ExitCode::FAILURE
        }
    }
}
