use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateMessageResponse {
    pub error_code: Option<String>,
    pub error_message: Option<String>,
    pub status: String,
    pub body: Option<String>,
}

pub enum ResponseType {
    CreateMessage,
}

#[derive(Debug)]
pub enum Response {
    CreateMessage(Option<CreateMessageResponse>),
}

impl ResponseType {
    pub fn deserialize(&self, body: &[u8]) -> Response {
        match self {
            ResponseType::CreateMessage => {
                if let Ok(deser) = serde_json::from_slice(body) {
                    Response::CreateMessage(Some(deser))
                } else {
                    Response::CreateMessage(None)
                }
            }
        }
    }
}

#[derive(Serialize)]
pub struct CreateMessageRequest {
    #[serde(rename = "Body")]
    pub body: String,
    #[serde(rename = "From")]
    pub from: String,
    #[serde(rename = "To")]
    pub to: String,
}
