use chrono::NaiveTime;
use failure::{bail, format_err, Error, ResultExt};
use serde::Deserialize;
use std::{collections::HashMap, env, fs, path::PathBuf};

static REFERENCE_CONF: &str = "# Populate config with your own values

# The earliest time a notification will be sent that your team plays today: HH:MM:SS
earliest_notification_time: 07:00:00

# Subscriptions are declared as a team id, then a list of phone numbers
# that should receive notifications for that team.
#
# Team id's can be referenced from https://statsapi.web.nhl.com/api/v1/teams
#
# Phone numbers must be stored as \"+15555555\"
subscriptions:
  - team: 1
    numbers:
      - \"+15555555\"
      - \"+15555678\"
  - team: 54
    numbers:
      - \"+15557890\"
";

#[derive(Deserialize, Debug)]
pub struct AppConfig {
    pub earliest_notification_time: NaiveTime,
    pub subscriptions: Vec<TeamSubscription>,
    #[serde(skip)]
    pub twilio: TwilioConfig,
}

impl AppConfig {
    pub fn new(
        path: PathBuf,
        twil_sid: String,
        twil_token: String,
        twil_from: String,
    ) -> Result<Self, Error> {
        log::info!("Using config file: {:?}", path);
        let mut config = config::Config::default();
        config
            .merge(config::File::from(path.clone()))
            .context(format_err!("Could not get config from: {:?}", path))?;

        let mut app_config: AppConfig = config.try_into().context(format_err!(
            "Config file format is wrong, make sure it matches the following:\n\n{}",
            &REFERENCE_CONF
        ))?;

        app_config.twilio.account_sid = twil_sid;
        app_config.twilio.auth_token = twil_token;
        app_config.twilio.from = twil_from;

        Ok(app_config)
    }

    pub fn subscriptions_as_hashmap(&self) -> HashMap<u32, Vec<String>> {
        let mut map = HashMap::new();
        for sub in self.subscriptions.iter() {
            if !sub.numbers.is_empty() {
                map.insert(sub.team, sub.numbers.clone());
            }
        }
        map
    }
}

#[derive(Deserialize, Debug)]
pub struct TwilioConfig {
    pub account_sid: String,
    pub auth_token: String,
    pub from: String,
}

impl Default for TwilioConfig {
    fn default() -> Self {
        let account_sid = String::from("");
        let auth_token = String::from("");
        let from = String::from("");
        TwilioConfig {
            account_sid,
            auth_token,
            from,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct TeamSubscription {
    pub team: u32,
    pub numbers: Vec<String>,
}

pub fn generate_empty_config() -> Result<(), Error> {
    let mut path = env::current_dir()?;
    path.push("config.yml");

    if path.exists() {
        bail!("config.yml already exists");
    }

    fs::write(path, &REFERENCE_CONF)?;

    Ok(())
}
