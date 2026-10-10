use std::process::ExitCode;

mod cli;
mod luau;
mod require;
mod require_loader;
mod setup;
mod compile;

fn main() -> ExitCode {
    #[cfg(windows)]
    vara_util::win_compat::init();

    async_io::block_on(async {

        let (is_binary, contents) = compile::check().await;

        if is_binary {

            return match compile::run_standalone(contents){
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
