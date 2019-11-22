use crate::client::Client;
use crate::model::ScheduleGame;
use async_std::task;
use env_logger::Env;
use failure::Error;
use futures::future;
use log::{error, info};

mod client;
mod model;

fn main() -> Result<(), Error> {
    env_logger::init_from_env(Env::new().default_filter_or("async_std_test=info"));

    task::block_on(async {
        if let Err(e) = run().await {
            error!("{:?}", e);
        };
    });

    Ok(())
}

async fn run() -> Result<(), Error> {
    let client = Client::new();

    let teams = client.get_teams().await?;

    info!("Got {} teams", teams.len());

    let get_teams = teams
        .into_iter()
        .map(|_team| {
            async {
                let team = &client.get_team(_team.id).await;
                drop(_team);
                match team {
                    Ok(team) => info!("Got response for the {}", team.name),
                    Err(e) => error!("{:?}", e),
                }
            }
        })
        .collect::<Vec<_>>();

    future::join_all(get_teams).await;

    let todays_schedule = client.get_todays_schedule().await?;

    info!(
        "There are {} games today, {}",
        todays_schedule.games.len(),
        todays_schedule.date
    );

    let run_games = todays_schedule
        .games
        .into_iter()
        .map(run_game)
        .collect::<Vec<_>>();

    future::join_all(run_games).await;

    info!("All games have finished.");

    Ok(())
}

async fn run_game(game: ScheduleGame) {
    info!(
        "{} vs {} starts at {}",
        game.teams.home.detail.name, game.teams.away.detail.name, game.date
    );
    while chrono::Utc::now() < game.date {
        info!(
            "{} vs {} hasn't started yet...",
            game.teams.home.detail.name, game.teams.away.detail.name
        );
        task::sleep(std::time::Duration::from_secs(60 * 5)).await;
    }
}
