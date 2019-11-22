use crate::model::{Response, ResponseType, Schedule, Team};
use failure::{bail, format_err, Error, ResultExt};
use futures::AsyncReadExt;
use http::{Request, Uri};
use http_client::{native::NativeClient, Body, HttpClient};
use std::collections::HashMap;

const BASE_URL: &str = "http://statsapi.web.nhl.com/api/v1";

pub struct Client {
    client: NativeClient,
    base: String,
}

impl Client {
    pub fn new() -> Self {
        let client = NativeClient::new();
        let base = BASE_URL.into();
        Client { client, base }
    }

    #[allow(unused_variables)]
    fn get_url(&self, path: &str, modifiers: Option<HashMap<String, String>>) -> http::Uri {
        let uri = format!("{}/{}", self.base, path);
        uri.parse::<Uri>().unwrap()
    }

    async fn get(&self, url: Uri, response_type: ResponseType) -> Result<Response, Error> {
        let request = Request::builder()
            .method("GET")
            .uri(url)
            .body(Body::empty())
            .unwrap();

        let res = self
            .client
            .send(request)
            .await
            .context("Failed to get teams")?;

        let mut body = res.into_body();
        let mut bytes = Vec::new();
        body.read_to_end(&mut bytes).await?;

        let response = response_type.deserialize(&bytes);

        Ok(response)
    }

    pub async fn get_teams(&self) -> Result<Vec<Team>, Error> {
        let url = self.get_url("teams", None);
        let response_type = ResponseType::TeamsResponse;

        let _response = self.get(url, response_type).await?;

        if let Response::TeamsResponse(Some(response)) = _response {
            return Ok(response.teams);
        }
        bail!("Failed to get teams response.")
    }

    pub async fn get_team(&self, team_id: u32) -> Result<Team, Error> {
        let url = self.get_url(&format!("teams/{}", team_id), None);
        let response_type = ResponseType::TeamsResponse;

        let _response = self.get(url, response_type).await?;

        if let Response::TeamsResponse(Some(mut response)) = _response {
            let team = response
                .teams
                .pop()
                .ok_or_else(|| format_err!("Failed to get team response."))?;
            return Ok(team);
        }
        bail!("Failed to get team response.")
    }

    pub async fn get_todays_schedule(&self) -> Result<Schedule, Error> {
        let url = self.get_url("schedule", None);
        let response_type = ResponseType::ScheduleResponse;

        let _response = self.get(url, response_type).await?;

        if let Response::ScheudleResponse(Some(mut response)) = _response {
            let schedule = response
                .dates
                .pop()
                .ok_or_else(|| format_err!("Failed to get schedule response."))?;
            return Ok(schedule);
        }
        bail!("Failed to get schedule response.")
    }
}
