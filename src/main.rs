use async_std::task;
use env_logger::Env;
use failure::Error;
use log::error;
use std::process;

mod cli;
mod config;
mod game;

fn main() -> Result<(), Error> {
    task::block_on(async {
        if let Err(e) = run().await {
            log_error(&e);
            process::exit(1);
        };
    });

    Ok(())
}

async fn run() -> Result<(), Error> {
    env_logger::init_from_env(Env::new().default_filter_or("nhl_notifier=info"));

    let result = cli::parse_opts()?;
    match result {
        cli::CliStatus::Exit => process::exit(0),
        cli::CliStatus::Continue(config) => {
            log::debug!("Config is: {:?}", config);
            game::run_todays_games(&config).await?;
        }
    }

    Ok(())
}

/// Log any errors and causes
pub fn log_error(e: &Error) {
    error!("{}", e);
    for cause in e.iter_causes() {
        error!("Caused by: {}", cause);
    }
}
