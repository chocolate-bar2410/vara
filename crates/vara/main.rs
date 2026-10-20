#![deny(clippy::unwrap_in_result)]
#![deny(clippy::unwrap_used)]

use std::process::ExitCode;

mod cli;

mod setup;
mod compile;

fn main() -> ExitCode {
    #[cfg(windows)]
    vara_util::win_compat::init();

    async_io::block_on(async {

        let (is_binary, contents) = match compile::check().await {
            Ok((bin,source)) => (bin,source),
            Err(err) => {
                eprintln!("failed to read executable {err}");
                return ExitCode::FAILURE
            }
        };

        if is_binary {

            return match compile::run_standalone(contents).await{
                Ok(_) => ExitCode::SUCCESS,
                Err(err) => {
                    eprintln!("{err}");
                    ExitCode::FAILURE
                },
            }
        }


        match cli::run().await {
            Ok(_) => ExitCode::SUCCESS,
            Err(err) => {
                eprintln!("{err}");
                ExitCode::FAILURE
            }
        }
    })
    
}
