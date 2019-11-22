use chrono::NaiveDate;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TeamsResponse {
    pub teams: Vec<Team>,
}

#[derive(Debug, Deserialize)]
pub struct Team {
    pub id: u32,
    pub name: String,
    pub link: String,
    pub abbreviation: String,
    #[serde(rename(deserialize = "teamName"))]
    pub team_name: String,
    #[serde(rename(deserialize = "locationName"))]
    pub location_name: String,
    #[serde(rename(deserialize = "firstYearOfPlay"))]
    pub first_year_of_play: String,
    #[serde(rename(deserialize = "shortName"))]
    pub short_name: String,
    #[serde(rename(deserialize = "officialSiteUrl"))]
    pub official_site_url: String,
    #[serde(rename(deserialize = "franchiseId"))]
    pub franchise_id: u32,
    pub active: bool,
}

pub enum ResponseType {
    TeamsResponse,
    ScheduleResponse,
}

pub enum Response {
    TeamsResponse(Option<TeamsResponse>),
    ScheudleResponse(Option<ScheduleResponse>),
}

impl ResponseType {
    pub fn deserialize(&self, body: &[u8]) -> Response {
        match self {
            ResponseType::TeamsResponse => {
                if let Ok(deser) = serde_json::from_slice(body) {
                    Response::TeamsResponse(Some(deser))
                } else {
                    Response::TeamsResponse(None)
                }
            }
            ResponseType::ScheduleResponse => {
                if let Ok(deser) = serde_json::from_slice(body) {
                    Response::ScheudleResponse(Some(deser))
                } else {
                    Response::ScheudleResponse(None)
                }
            }
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ScheduleResponse {
    pub dates: Vec<Schedule>,
}

#[derive(Debug, Deserialize)]
pub struct Schedule {
    pub date: NaiveDate,
    pub games: Vec<ScheduleGame>,
}

#[derive(Debug, Deserialize)]
pub struct ScheduleGame {
    #[serde(rename(deserialize = "gamePk"))]
    pub game_pk: u64,
    pub link: String,
    #[serde(rename(deserialize = "gameDate"))]
    pub date: chrono::DateTime<chrono::Utc>,
    #[serde(rename(deserialize = "gameType"))]
    pub game_type: String,
    pub season: String,
    pub teams: ScheduleGameTeams,
}

#[derive(Debug, Deserialize)]
pub struct ScheduleGameTeams {
    pub away: ScheduleGameTeam,
    pub home: ScheduleGameTeam,
}

#[derive(Debug, Deserialize)]
pub struct ScheduleGameTeam {
    pub score: u8,
    #[serde(rename(deserialize = "team"))]
    pub detail: ScheduleGameTeamDetail,
}

#[derive(Debug, Deserialize)]
pub struct ScheduleGameTeamDetail {
    pub id: u32,
    pub name: String,
    pub link: String,
}
