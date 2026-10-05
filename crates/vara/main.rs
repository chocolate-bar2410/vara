use std::process::ExitCode;

mod cli;
mod luau;
mod require;
mod require_loader;
mod setup;

fn main() -> ExitCode {
    #[cfg(windows)]
    vara_util::win_compat::init();

    async_io::block_on(async {
        match cli::run().await {
            Ok(_) => ExitCode::SUCCESS,
            Err(err) => {
                eprintln!("{err}");
                ExitCode::FAILURE
            }
        }
    })
    
}
