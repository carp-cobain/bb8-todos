use crate::Result;
use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tracing::{event, span, Level};

/// The query parameters for getting a page of domain objects from a list endpoint.
#[derive(Debug, Deserialize, Default)]
pub struct PageParams {
    pub page_token: Option<String>,
}

/// A page of domain objects
#[derive(Debug, Serialize)]
pub struct Page<T: Serialize> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev_page: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page: Option<String>,
    pub data: Vec<T>,
}

impl<T: Serialize> Page<T> {
    // Create a new page of domain objects
    pub fn new(prev_page: Option<String>, next_page: Option<String>, data: Vec<T>) -> Self {
        Self {
            prev_page,
            next_page,
            data,
        }
    }
}

/// A paging token for accessing previous, next pages of domain objects in a list call.
#[derive(BorshSerialize, BorshDeserialize)]
pub struct PageToken {
    pub id: i32,
    pub ts: u128,
}

impl PageToken {
    /// Encode a page id as a token.
    pub fn encode(page_id: i32) -> Option<String> {
        let _span = span!(Level::DEBUG, "PageToken::encode").entered();
        if page_id <= 0 {
            return None;
        }
        event!(Level::DEBUG, "start");
        let token = PageToken::new(page_id);
        event!(Level::DEBUG, "created");
        let bytes = borsh::to_vec(&token).unwrap_or_default();
        event!(Level::DEBUG, "borsh serialized");
        let bytes = URL_SAFE.encode(bytes);
        event!(Level::DEBUG, "b64 encoded");
        Some(bytes)
    }

    /// Extract page id from encoded token param
    pub fn decode(token: Option<String>) -> Result<i32> {
        let _span = span!(Level::DEBUG, "PageToken::decode").entered();
        match token {
            None => Ok(1),
            Some(token) => {
                event!(Level::DEBUG, "start");
                let bytes = URL_SAFE.decode(token)?;
                event!(Level::DEBUG, "b64 decoded");
                let page_token: PageToken = borsh::from_slice(&bytes).unwrap();
                event!(Level::DEBUG, "borsh deserialized");
                Ok(page_token.id)
            }
        }
    }

    fn new(id: i32) -> Self {
        Self { id, ts: now() }
    }
}

/// Calculate the number of milliseconds since the unix epoch.
fn now() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::MAX)
        .as_millis()
}
