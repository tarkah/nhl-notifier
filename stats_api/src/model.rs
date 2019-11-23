use chrono::{DateTime, NaiveDate, Utc};
use serde::Deserialize;

#[serde(rename_all = "camelCase")]
#[derive(Debug, Deserialize)]
pub struct TeamsResponse {
    pub teams: Vec<Team>,
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Deserialize)]
pub struct Team {
    pub id: u32,
    pub name: String,
    pub link: String,
    pub abbreviation: String,
    pub team_name: String,
    pub location_name: String,
    pub first_year_of_play: String,
    pub short_name: String,
    pub official_site_url: String,
    pub franchise_id: u32,
    pub active: bool,
}

pub enum ResponseType {
    TeamsResponse,
    ScheduleResponse,
    GameContentResponse,
    GameLinescoreResponse,
}

pub enum Response {
    TeamsResponse(Option<TeamsResponse>),
    ScheudleResponse(Option<ScheduleResponse>),
    GameContentResponse(Option<GameContentResponse>),
    GameLinescoreResponse(Option<GameLinescoreResponse>),
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
            ResponseType::GameContentResponse => {
                if let Ok(deser) = serde_json::from_slice(body) {
                    Response::GameContentResponse(Some(deser))
                } else {
                    Response::GameContentResponse(None)
                }
            }
            ResponseType::GameLinescoreResponse => {
                if let Ok(deser) = serde_json::from_slice(body) {
                    Response::GameLinescoreResponse(Some(deser))
                } else {
                    Response::GameLinescoreResponse(None)
                }
            }
        }
    }
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Deserialize)]
pub struct ScheduleResponse {
    pub dates: Vec<Schedule>,
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Deserialize)]
pub struct Schedule {
    pub date: NaiveDate,
    pub games: Vec<ScheduleGame>,
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Deserialize)]
pub struct ScheduleGame {
    pub game_pk: u64,
    pub link: String,
    #[serde(rename(deserialize = "gameDate"))]
    pub date: DateTime<Utc>,
    pub game_type: String,
    pub season: String,
    pub teams: ScheduleGameTeams,
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Deserialize)]
pub struct ScheduleGameTeams {
    pub away: ScheduleGameTeam,
    pub home: ScheduleGameTeam,
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Deserialize)]
pub struct ScheduleGameTeam {
    pub score: u8,
    #[serde(rename(deserialize = "team"))]
    pub detail: ScheduleGameTeamDetail,
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Deserialize)]
pub struct ScheduleGameTeamDetail {
    pub id: u32,
    pub name: String,
    pub link: String,
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Deserialize)]
pub struct GameContentResponse {
    pub editorial: GameContentEditorial,
    pub media: GameContentMedia,
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Deserialize)]
pub struct GameContentMedia {
    pub milestones: GameContentMilestones,
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Deserialize)]
pub struct GameContentEditorial {
    pub preview: GameContentEditorialItem,
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Deserialize)]
pub struct GameContentEditorialItem {
    pub title: String,
    pub items: Option<Vec<GameContentEditorialItemArticle>>,
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Deserialize, Clone)]
pub struct GameContentEditorialItemArticle {
    pub r#type: String,
    pub headline: String,
    pub subhead: String,
    pub seo_title: String,
    pub seo_description: String,
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Deserialize)]
pub struct GameContentMilestones {
    pub stream_start: Option<DateTime<Utc>>,
    pub items: Option<Vec<GameContentMilestoneItem>>,
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Deserialize)]
pub struct GameContentMilestoneItem {
    pub description: String,
    pub r#type: String,
    pub period: String,
    pub ordinal_num: String,
    pub team_id: String,
    pub stats_event_id: String,
    #[serde(deserialize_with = "fail_as_none")]
    pub highlight: Option<GameContentMilestoneItemHighlight>,
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Deserialize)]
pub struct GameContentMilestoneItemHighlight {
    pub title: String,
    pub blurb: String,
    pub description: String,
    pub playbacks: Option<Vec<GameContentMilestoneItemHighlightPlayback>>,
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Deserialize)]
pub struct GameContentMilestoneItemHighlightPlayback {
    pub name: String,
    pub url: String,
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Deserialize)]
pub struct GameLinescoreResponse {
    pub current_period: u8,
    pub teams: GameLinescoreTeams,
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Deserialize)]
pub struct GameLinescoreTeams {
    pub home: GameLinescoreTeam,
    pub away: GameLinescoreTeam,
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Deserialize)]
pub struct GameLinescoreTeam {
    #[serde(rename(deserialize = "team"))]
    pub detail: GameLinescoreTeamDetail,
    pub goals: u8,
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Deserialize)]
pub struct GameLinescoreTeamDetail {
    pub id: u32,
    pub name: String,
    pub link: String,
}

fn fail_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: serde::Deserialize<'de>,
{
    let result = T::deserialize(de);
    match result {
        Ok(t) => Ok(Some(t)),
        Err(_) => Ok(None),
    }
}
