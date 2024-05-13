use crate::signer::{Signer, Verifier};
use crate::{Error, Result};

use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// One hour in milliseconds
const ONE_HOUR_MILLIS: u128 = 3600000;

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
    pub sig: Vec<u8>,
}

impl PageToken {
    /// Encode a page id as a token.
    pub fn encode(page_id: i32) -> Option<String> {
        if page_id <= 0 {
            return None;
        }
        let token = PageToken::new(page_id);
        let bytes = borsh::to_vec(&token).unwrap_or_default();
        Some(URL_SAFE.encode(bytes))
    }

    /// Extract page id from encoded token param
    pub fn decode(token: Option<String>) -> Result<i32> {
        match token {
            None => Ok(1),
            Some(token) => {
                let bytes = URL_SAFE.decode(token)?;
                let page_token: PageToken = borsh::from_slice(&bytes).unwrap();
                page_token.verify()?;
                Ok(page_token.id)
            }
        }
    }

    fn new(id: i32) -> Self {
        let ts = now();
        let msg = Msg::bytes(id, ts);
        let signer: Signer = Default::default();
        let sig = signer.sign(&msg).unwrap_or_default();
        Self { id, ts, sig }
    }

    fn verify(&self) -> Result<()> {
        // Check signature first
        let msg = Msg::bytes(self.id, self.ts);
        let verifier: Verifier = Default::default();
        verifier.verify(&msg, &self.sig)?;

        // Check for expiriration
        if now() - self.ts > ONE_HOUR_MILLIS {
            return Err(Error::invalid_args("page token expired"));
        }

        Ok(())
    }
}

/// Message for signing / verification
#[derive(BorshSerialize)]
struct Msg {
    id: i32,
    ts: u128,
}

impl Msg {
    /// Create a binary signing message
    fn bytes(id: i32, ts: u128) -> Vec<u8> {
        borsh::to_vec(&Msg { id, ts }).unwrap_or_default()
    }
}

/// Calculate the number of milliseconds since the unix epoch.
fn now() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::MAX)
        .as_millis()
}
