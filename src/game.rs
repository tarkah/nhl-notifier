use crate::config::AppConfig;
use async_std::task;
use chrono::{DateTime, Local, NaiveTime, Utc};
use failure::{bail, format_err, Error};
use futures::future;
use log::{error, info, warn};
use stats_api::model::{
    GameContentEditorialItemArticle, GameContentMilestoneItem, GameContentMilestoneItemHighlight,
    GameContentMilestones, ScheduleGame, Team,
};
use std::{collections::HashMap, time::Duration};

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

    let games_to_notify: Vec<(ScheduleGame, Vec<String>)> = todays_schedule
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

struct Game {
    stats_client: stats_api::Client,
    twil_client: twilio::Client,
    twil_from: String,
    earliest_notification: chrono::NaiveTime,
    game_id: u64,
    game_type: String,
    date: DateTime<Utc>,
    home_team: Team,
    away_team: Team,
    score: GameScore,
    goals: HashMap<u32, Goal>,
    highlights_notified: Vec<u32>,
    subscriptions: Vec<String>,
    preview: Option<GameContentEditorialItemArticle>,
    status: GameStatus,
}

impl Game {
    async fn new(
        game: ScheduleGame,
        subscriptions: Vec<String>,
        config: &AppConfig,
    ) -> Result<Self, Error> {
        let stats_client = stats_api::Client::new();

        let twil_client = twilio::Client::new(
            config.twilio.account_sid.clone(),
            config.twilio.auth_token.clone(),
        );
        let twil_from = config.twilio.from.clone();

        let earliest_notification = config.earliest_notification_time;

        let game_id = game.game_pk;
        let date = game.date;

        let home_team = stats_client.get_team(game.teams.home.detail.id).await?;
        let away_team = stats_client.get_team(game.teams.away.detail.id).await?;

        Ok(Game {
            stats_client,
            twil_client,
            twil_from,
            earliest_notification,
            game_id,
            game_type: game.game_type,
            date,
            home_team,
            away_team,
            score: GameScore::new(),
            goals: HashMap::new(),
            highlights_notified: vec![],
            subscriptions,
            preview: None,
            status: GameStatus::Scheduled,
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

    async fn get_milestones(&mut self) -> Result<GameContentMilestones, Error> {
        let content = self.stats_client.get_game_content(self.game_id).await?;
        let milestones = content.media.milestones;
        Ok(milestones)
    }

    async fn get_milestone_items(&mut self) -> Result<Vec<GameContentMilestoneItem>, Error> {
        let milestones = self.get_milestones().await?;

        if let Some(items) = milestones.items {
            Ok(items)
        } else {
            bail!("No milestone items yet");
        }
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

    async fn send_message(&self, message: &str) {
        for number in self.subscriptions.iter() {
            let _response = self
                .twil_client
                .send_message(self.twil_from.as_str(), number, message)
                .await;

            match _response {
                Ok(response) => {
                    if response.status == "sent" || response.status == "queued" {
                        self.log_info(format!("Notification sent for: {}", number));
                    } else {
                        self.log_error(format!("Notification couldn't send for: {}", number));
                    }
                }
                Err(e) => self.log_error(format_err!("Failure sending message, error: {:?}", e)),
            }
        }
    }

    async fn send_preview_notification(&self) {
        let notification = format!(
            "{} @ {} - {}\n\
             \n\
             {}",
            self.home_team.name,
            self.away_team.name,
            self.local_datetime().format("%I:%M:%S %p"),
            self.subhead(),
        );

        self.send_message(&notification).await;
    }

    #[allow(clippy::ptr_arg)]
    fn check_end(&self, milestone_items: &Vec<GameContentMilestoneItem>) -> bool {
        for item in milestone_items {
            if item.r#type == "BROADCAST_END" {
                return true;
            }
        }
        false
    }

    #[allow(clippy::ptr_arg)]
    fn parse_goals(
        &mut self,
        milestone_items: &Vec<GameContentMilestoneItem>,
    ) -> HashMap<u32, Goal> {
        let mut goals = HashMap::new();
        for item in milestone_items {
            if item.r#type == "GOAL" {
                let event_id = item.stats_event_id.parse::<u32>().ok();
                let team_id = item.team_id.parse::<u32>().ok();
                let period_time =
                    NaiveTime::parse_from_str(&format!("00:{}", item.period_time), "%H:%M:%S").ok();

                if event_id.is_none() || team_id.is_none() || period_time.is_none() {
                    self.log_error("Could not parse goal milestone items.");
                    continue;
                }

                let time = {
                    match self.game_type.as_str() {
                        "P" => 20,
                        _ => match item.ordinal_num.as_str() {
                            "OT" => 5,
                            "SO" => 0,
                            _ => 20,
                        },
                    }
                };

                let period_time = NaiveTime::from_hms(0, 0, 0)
                    + (NaiveTime::from_hms(0, time, 0) - period_time.unwrap());

                let goal = Goal {
                    event_id: event_id.unwrap(),
                    team_id: team_id.unwrap(),
                    description: item.description.clone(),
                    ordinal_num: item.ordinal_num.clone(),
                    period_time,
                    highlight: item.highlight.clone(),
                };

                goals.insert(goal.event_id, goal);
            }
        }

        goals
    }

    async fn process_goals(&mut self, goals: &HashMap<u32, Goal>) {
        if goals.is_empty() {
            return;
        }

        // Add any goals that don't yet exist in stored goals. We will need to add these
        // to the score.
        let goals_to_add: HashMap<u32, Goal> = goals
            .clone()
            .into_iter()
            .filter(|(id, _)| {
                if self.goals.contains_key(&id) {
                    return false;
                }
                true
            })
            .collect();

        // Remove any goals that no longer exist (goal is overturned). We will need
        // to deduct these back from the score.
        let goals_to_remove: HashMap<u32, Goal> = self
            .goals
            .iter()
            .filter_map(|(id, stored_goal)| {
                if goals.contains_key(id) {
                    None
                } else {
                    Some((*id, stored_goal.clone()))
                }
            })
            .collect();

        for (id, goal) in goals_to_add {
            self.add_goal_score(&goal);
            self.notify_goal(&goal).await;

            self.goals.insert(id, goal);
        }

        for (id, goal) in goals_to_remove {
            self.deduct_goal_score(&goal);

            // Remove the goal from stored goals
            self.goals.remove(&id);
        }
    }

    fn add_goal_score(&mut self, goal: &Goal) {
        let scoring_team = goal.team_id;
        if scoring_team == self.home_team.id {
            self.score.home += 1;
        } else {
            self.score.away += 1;
        }
    }

    fn deduct_goal_score(&mut self, goal: &Goal) {
        let scoring_team = goal.team_id;
        if scoring_team == self.home_team.id {
            self.score.home -= 1;
        } else {
            self.score.away -= 1;
        }
    }

    async fn notify_goal(&self, goal: &Goal) {
        let scoring_team_name = if goal.team_id == self.home_team.id {
            &self.home_team.team_name
        } else {
            &self.away_team.team_name
        };

        let message = format!(
            "{} score\n\
             \n\
             {} {}, {} {} - {} {}\n\
             \n\
             {}",
            scoring_team_name,
            goal.period_time.format("%M:%S"),
            goal.ordinal_num,
            self.home_team.abbreviation,
            self.score.home,
            self.away_team.abbreviation,
            self.score.away,
            goal.description
        );

        self.send_message(&message).await;

        self.log_info(format!(
            "{} score, {} {}, {} {} - {} {}, {}",
            scoring_team_name,
            goal.period_time.format("%M:%S"),
            goal.ordinal_num,
            self.home_team.abbreviation,
            self.score.home,
            self.away_team.abbreviation,
            self.score.away,
            goal.description
        ));
    }

    async fn process_highlights(&mut self, goals: &HashMap<u32, Goal>) {
        let highlights: HashMap<u32, GameContentMilestoneItemHighlight> = goals
            .clone()
            .into_iter()
            .filter_map(|(id, goal)| {
                if let Some(highlight) = goal.highlight {
                    Some((id, highlight))
                } else {
                    None
                }
            })
            .collect();

        for (id, highlight) in highlights {
            if !self.highlights_notified.contains(&id) {
                if let Err(e) = self.notify_highlight(highlight).await {
                    self.log_error(e);
                    return;
                };
                self.highlights_notified.push(id);
            }
        }
    }

    async fn notify_highlight(
        &self,
        highlight: GameContentMilestoneItemHighlight,
    ) -> Result<(), Error> {
        if let Some(playback) = highlight.playbacks {
            let mut clip_1800k = None;
            for clip in playback {
                if clip.name == "FLASH_1800K_896x504" {
                    clip_1800k = Some(clip)
                }
            }

            if let Some(clip) = clip_1800k {
                let message = format!(
                    "Highlight\n\
                     \n\
                     {}\n\
                     \n\
                     {}",
                    highlight.description, clip.url
                );

                self.send_message(&message).await;

                self.log_info(format!(
                    "Highlight, {}, {}",
                    highlight.description, clip.url
                ));

                return Ok(());
            }
        };
        bail!("No playback clip available");
    }

    async fn run(&mut self) {
        info!(
            "Running Game({}) - {} vs. {} @ {}...",
            self.game_id,
            self.home_team.name,
            self.away_team.name,
            self.local_datetime().to_rfc2822()
        );

        loop {
            match self.status {
                GameStatus::Scheduled => {
                    self.run_scheduled_game().await;
                    task::sleep(Duration::from_secs(60 * 10)).await;
                }
                GameStatus::Live => {
                    self.run_live_game().await;
                    task::sleep(Duration::from_secs(10)).await;
                }
                GameStatus::Ended => break,
            }
        }

        let winning_team_name = if self.score.home > self.score.away {
            self.home_team.team_name.clone()
        } else {
            self.away_team.team_name.clone()
        };

        let message = format!(
            "{} win\n\nFinal score: {} {} - {} {}",
            winning_team_name,
            self.home_team.abbreviation,
            self.score.home,
            self.away_team.abbreviation,
            self.score.away
        );

        self.send_message(&message).await;

        self.log_info(format!(
            "{} win. Final score: {} {} - {} {}",
            winning_team_name,
            self.home_team.abbreviation,
            self.score.home,
            self.away_team.abbreviation,
            self.score.away
        ));
    }

    async fn run_scheduled_game(&mut self) {
        // Don't try to get preview until earliest notification time.
        if Local::now().time() < self.earliest_notification {
            self.log_info("Before notification time, sleeping...");
            return;
        }

        // Once preview is sent, then move on to checking if game has started
        if self.preview.is_none() {
            // After time is passed, try to get preview. Don't proceed until
            // preview article is fetched.
            if let Err(e) = self.get_preview().await {
                self.log_warn(e);
                return;
            }

            // Now that preview is fetched, send out notification
            self.log_info(format!("Got preview: {}", self.subhead()));
            self.send_preview_notification().await;
        } else {
            // Check stream start time in milestone struct, if populated, game
            // has started. Change game to Live which will progress game forward.
            let milestones = self.get_milestones().await;
            match milestones {
                Err(e) => self.log_error(e),
                Ok(milestones) => {
                    if milestones.stream_start.is_some() {
                        self.status = GameStatus::Live
                    } else {
                        self.log_info("Game hasn't started yet, sleeping...");
                    }
                }
            }
        }
    }

    async fn run_live_game(&mut self) {
        if let Ok(items) = self.get_milestone_items().await {
            let goals = self.parse_goals(&items);

            self.process_goals(&goals).await;
            self.process_highlights(&goals).await;

            if self.check_end(&items) {
                self.status = GameStatus::Ended;
            }
        }
    }
}

fn log_words<'a, T>(vec: &[T]) -> (&'a str, &'a str) {
    let verb = {
        if vec.len() != 1 {
            "are"
        } else {
            "is"
        }
    };
    let plural = {
        if vec.len() != 1 {
            "s"
        } else {
            ""
        }
    };
    (verb, plural)
}

#[derive(PartialEq)]
enum GameStatus {
    Scheduled,
    Live,
    Ended,
}

struct GameScore {
    home: u8,
    away: u8,
}

impl GameScore {
    fn new() -> Self {
        GameScore { home: 0, away: 0 }
    }
}

#[derive(Clone)]
struct Goal {
    event_id: u32,
    team_id: u32,
    description: String,
    ordinal_num: String,
    period_time: NaiveTime,
    highlight: Option<GameContentMilestoneItemHighlight>,
}
