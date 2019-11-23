use crate::config::AppConfig;
use async_std::task;
use chrono::{DateTime, Local, Utc};
use failure::{bail, format_err, Error};
use futures::future;
use log::{error, info, warn};
use std::time::Duration;

pub async fn run_todays_games(config: &AppConfig) -> Result<(), Error> {
    let client = stats_api::Client::new();

    let today = Local::today().naive_local();
    let todays_schedule = client.get_schedule_for(today).await?;

    let (verb, plural) = log_words(&todays_schedule.games);
    info!(
        "There {} {} game{} today, {}",
        verb,
        todays_schedule.games.len(),
        plural,
        todays_schedule.date.format("%A, %-d %B, %C%y").to_string(),
    );

    let subscriptions = config.subscriptions_as_hashmap();
    let subscription_team_ids: Vec<u32> = subscriptions.keys().cloned().collect();

    let (verb, plural) = log_words(&subscription_team_ids);
    info!(
        "There {} {} team{} with an active subscription",
        verb,
        subscription_team_ids.len(),
        plural,
    );

    let games_to_notify: Vec<(stats_api::model::ScheduleGame, Vec<String>)> = todays_schedule
        .games
        .into_iter()
        .filter_map(|game| {
            let mut numbers = vec![];
            if subscription_team_ids.contains(&game.teams.home.detail.id) {
                numbers.extend(
                    subscriptions
                        .get(&game.teams.home.detail.id)
                        .unwrap()
                        .clone(),
                );
            }
            if subscription_team_ids.contains(&game.teams.away.detail.id) {
                numbers.extend(
                    subscriptions
                        .get(&game.teams.away.detail.id)
                        .unwrap()
                        .clone(),
                );
            }
            if numbers.is_empty() {
                None
            } else {
                Some((game, numbers))
            }
        })
        .collect();

    let (verb, plural) = log_words(&games_to_notify);
    info!(
        "There {} {} subscribable game{}",
        verb,
        games_to_notify.len(),
        plural,
    );

    let run_games = games_to_notify
        .into_iter()
        .map(|_game| {
            async {
                let game_name = format!(
                    "{} vs. {}",
                    _game.0.teams.home.detail.name, _game.0.teams.away.detail.name
                );
                let game = Game::new(_game.0, _game.1, &config).await;
                match game {
                    Ok(mut game) => {
                        game.run().await;
                    }
                    Err(e) => error!("Error running game for {}: {:?}", game_name, e),
                }
            }
        })
        .collect::<Vec<_>>();

    future::join_all(run_games).await;

    Ok(())
}

#[allow(dead_code)]
struct Game {
    stats_client: stats_api::Client,
    twil_client: twilio::Client,
    twil_from: String,
    game_id: u64,
    date: DateTime<Utc>,
    home_team: stats_api::model::ScheduleGameTeam,
    away_team: stats_api::model::ScheduleGameTeam,
    scoring_event_ids: Vec<u32>,
    subscriptions: Vec<String>,
    preview: Option<stats_api::model::GameContentEditorialItemArticle>,
}

impl Game {
    async fn new(
        game: stats_api::model::ScheduleGame,
        subscriptions: Vec<String>,
        config: &AppConfig,
    ) -> Result<Self, Error> {
        let stats_client = stats_api::Client::new();

        let twil_client = twilio::Client::new(
            config.twilio.account_sid.clone(),
            config.twilio.auth_token.clone(),
        );
        let twil_from = config.twilio.from.clone();

        let game_id = game.game_pk;
        let date = game.date;
        let home_team = game.teams.home;
        let away_team = game.teams.away;

        let scoring_event_ids = vec![];

        Ok(Game {
            stats_client,
            twil_client,
            twil_from,
            game_id,
            date,
            home_team,
            away_team,
            scoring_event_ids,
            subscriptions,
            preview: None,
        })
    }

    fn local_datetime(&self) -> DateTime<Local> {
        self.date.with_timezone(&Local)
    }

    fn log_info<S: std::fmt::Display>(&self, msg: S) {
        info!("Game({}) - {}", self.game_id, msg);
    }

    fn log_error<S: std::fmt::Display>(&self, msg: S) {
        error!("Game({}) - {}", self.game_id, msg);
    }

    fn log_warn<S: std::fmt::Display>(&self, msg: S) {
        warn!("Game({}) - {}", self.game_id, msg);
    }

    async fn get_preview(&mut self) -> Result<(), Error> {
        let content = self.stats_client.get_game_content(self.game_id).await?;
        let preview_items = content.editorial.preview.items;
        if let Some(items) = preview_items {
            if let Some(preview) = items.first() {
                self.preview = Some(preview.clone());
                return Ok(());
            }
        }
        bail!("Preview not available");
    }

    fn subhead(&self) -> String {
        if let Some(preview) = self.preview.as_ref() {
            return preview.subhead.clone();
        }
        self.log_error("Subhead called before preview retrieved");
        String::from("")
    }

    async fn send_message(&self, number: &str, message: &str) -> Result<(), Error> {
        let _response = self
            .twil_client
            .send_message(self.twil_from.as_str(), number, message)
            .await;
        match _response {
            Ok(response) => {
                if response.status == "sent" || response.status == "queued" {
                    return Ok(());
                }
                bail!(format_err!(
                    "Failure sending message, response was: {:?}",
                    response
                ));
            }
            Err(e) => bail!(format_err!("Failure sending message, error: {:?}", e)),
        }
    }

    async fn send_preview_notification(&self) -> Result<(), Error> {
        let notification = format!(
            "{} @ {} - {}\n\
             \n\
             {}",
            self.home_team.detail.name,
            self.away_team.detail.name,
            self.local_datetime().format("%I:%M:%S %p"),
            self.subhead(),
        );

        for number in self.subscriptions.iter() {
            if let Err(e) = self.send_message(&number, &notification).await {
                return Err(e);
            }
            self.log_info(format!("Preview notification sent for: {}", number));
        }

        Ok(())
    }

    async fn run(&mut self) {
        info!(
            "Running Game({}) - {} vs. {} @ {}...",
            self.game_id,
            self.home_team.detail.name,
            self.away_team.detail.name,
            self.local_datetime().to_rfc2822()
        );

        while let Err(e) = self.get_preview().await {
            self.log_warn(e);
            task::sleep(Duration::from_secs(10 * 60)).await;
        }
        self.log_info(format!("Got preview: {}", self.subhead()));

        if let Err(e) = self.send_preview_notification().await {
            self.log_error(e);
        };
    }
}

fn log_words<'a, T>(vec: &[T]) -> (&'a str, &'a str) {
    let verb = {
        if vec.len() > 1 {
            "are"
        } else {
            "is"
        }
    };
    let plural = {
        if vec.len() > 1 {
            "s"
        } else {
            ""
        }
    };
    (verb, plural)
}
