use crate::model::{CreateMessageRequest, CreateMessageResponse, Response, ResponseType};
use failure::{bail, Error, ResultExt};
use futures::AsyncReadExt;
use http::{Request, Uri};
use http_client::{native::NativeClient, Body, HttpClient};
use std::collections::HashMap;

pub struct Client {
    client: NativeClient,
    base: String,
    account_sid: String,
}

impl Client {
    pub fn new(account_sid: String, auth_token: String) -> Self {
        let client = NativeClient::new();
        let base = format!(
            "https://{}:{}@api.twilio.com/2010-04-01",
            account_sid, auth_token
        );
        Client {
            client,
            base,
            account_sid,
        }
    }

    fn get_url(&self, path: &str, params: Option<HashMap<&str, String>>) -> http::Uri {
        if let Some(params) = params {
            let params = serde_urlencoded::to_string(params).unwrap_or_else(|_| String::from(""));
            let uri = format!("{}/{}?{}", self.base, path, params);
            uri.parse::<Uri>().unwrap()
        } else {
            let uri = format!("{}/{}", self.base, path);
            uri.parse::<Uri>().unwrap()
        }
    }

    async fn post(
        &self,
        url: Uri,
        body: Body,
        response_type: ResponseType,
    ) -> Result<Response, Error> {
        let request = Request::builder()
            .method("POST")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .uri(url)
            .body(body)
            .unwrap();

        let res = self
            .client
            .send(request)
            .await
            .context("Failed to post request")?;

        let mut body = res.into_body();
        let mut bytes = Vec::new();
        body.read_to_end(&mut bytes).await?;

        let response = response_type.deserialize(&bytes);

        Ok(response)
    }

    pub async fn send_message(
        &self,
        from: &str,
        to: &str,
        message: &str,
    ) -> Result<CreateMessageResponse, Error> {
        let url = self.get_url(
            &format!("Accounts/{}/Messages.json", self.account_sid),
            None,
        );
        let response_type = ResponseType::CreateMessage;

        let message = CreateMessageRequest {
            from: String::from(from),
            to: String::from(to),
            body: String::from(message),
        };

        if let Ok(string) = serde_urlencoded::to_string(&message) {
            let body = Body::from(string.into_bytes());
            let _response = self.post(url, body, response_type).await?;

            if let Response::CreateMessage(Some(response)) = _response {
                return Ok(response);
            }
        }
        bail!("Could not serialize / deserialize message to send");
    }
}
