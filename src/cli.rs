use crate::config::{generate_empty_config, AppConfig};
use failure::{bail, Error, ResultExt};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "nhl-notifier",
    about = "Get live game updates via SMS for your favorite NHL team.",
    version = "0.1.0",
    author = "tarkah <admin@tarkah.dev>"
)]
pub struct Opt {
    #[structopt(subcommand)] // Note that we mark a field as a subcommand
    pub cmd: Command,
}

#[derive(Debug, StructOpt)]
pub enum Command {
    /// Run the program
    Run {
        #[structopt(short, long, parse(from_os_str))]
        /// Specify path to the config.yml file
        config: PathBuf,
        #[structopt(long = "twil-sid", env = "TWIL_ACCOUNT_SID")]
        twil_sid: Option<String>,
        #[structopt(long = "twil-token", env = "TWIL_AUTH_TOKEN", hide_env_values = true)]
        twil_token: Option<String>,
        #[structopt(long = "twil-from", env = "TWIL_FROM")]
        /// Specify the From number for twilio, must be formatted as '+15555555'
        twil_from: Option<String>,
    },
    /// Generate an empty config.yml file to the current directory
    Generate,
}

pub fn parse_opts() -> Result<CliStatus, Error> {
    let opt = Opt::from_args();
    log::debug!("Cli opts are: {:?}", opt);

    match opt.cmd {
        Command::Generate => {
            generate_empty_config().context("Failed to generate config")?;
            log::info!("config.yml generated");
            Ok(CliStatus::Exit)
        }
        Command::Run {
            config,
            twil_sid,
            twil_token,
            twil_from,
        } => {
            if twil_sid.is_none() || twil_token.is_none() || twil_from.is_none() {
                bail!("TWIL_ACCOUNT_SID, TWIL_AUTH_TOKEN & TWIL_FROM env variables must be set, or passed via --twil-sid, --twil-token & --twil-from");
            }
            let twil_sid = twil_sid.unwrap();
            let twil_token = twil_token.unwrap();
            let twil_from = twil_from.unwrap();

            let app_config = AppConfig::new(config, twil_sid, twil_token, twil_from)
                .context("Failed to get config")?;

            Ok(CliStatus::Continue(app_config))
        }
    }
}

pub enum CliStatus {
    Continue(AppConfig),
    Exit,
}
