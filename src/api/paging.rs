use crate::signer::{Signer, Verifier};
use crate::Result;

use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

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
#[derive(Debug, Deserialize, Serialize)]
pub struct PageToken {
    pub id: i32,
    pub ts: u128,
    pub sig: String,
}

impl PageToken {
    /// Encode a page id as a token.
    pub fn encode(page_id: i32) -> Option<String> {
        if page_id <= 0 {
            return None;
        }
        let token = PageToken::new(page_id);
        let json = serde_json::to_string(&token).unwrap_or_default();
        Some(URL_SAFE.encode(json))
    }

    /// Extract page id from encoded token param
    pub fn decode(token: Option<String>) -> Result<i32> {
        match token {
            None => Ok(1),
            Some(token) => {
                let bytes = URL_SAFE.decode(token)?;
                let page_token: PageToken = serde_json::from_slice(&bytes)?;
                page_token.verify()?;
                Ok(page_token.id)
            }
        }
    }

    fn new(id: i32) -> Self {
        let ts = now();
        let message = format!("\n{}\n{}", id, ts);
        let signer: Signer = Default::default();
        let sig = signer.sign(message.as_bytes()).unwrap_or_default();
        Self { id, ts, sig }
    }

    fn verify(&self) -> Result<()> {
        let signature = URL_SAFE.decode(&self.sig)?;
        let message = format!("\n{}\n{}", self.id, self.ts);
        let verifier: Verifier = Default::default();
        verifier.verify(message.as_bytes(), &signature)
    }
}

/// Calculate the number of milliseconds since the unix epoch.
fn now() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::MAX)
        .as_millis()
}
