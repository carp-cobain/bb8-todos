use crate::signer::{Signer, Verifier};
use crate::{Error, Result};

use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tracing::{event, span, Level};

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
                page_token.verify()?;
                event!(Level::DEBUG, "verified");
                Ok(page_token.id)
            }
        }
    }

    fn new(id: i32) -> Self {
        let ts = now();
        let msg = Msg::bytes(id, ts);
        let signer: Signer = Default::default();
        let sig = signer.sign(&msg);
        event!(Level::DEBUG, "signed");
        Self { id, ts, sig }
    }

    fn verify(&self) -> Result<()> {
        // Check signature first
        let msg = Msg::bytes(self.id, self.ts);
        let verifier: Verifier = Default::default();
        verifier.verify(&msg, &self.sig)?;
        event!(Level::DEBUG, "signature verified");

        // Check for expiriration
        if now() - self.ts > ONE_HOUR_MILLIS {
            return Err(Error::invalid_args("page token expired"));
        }
        event!(Level::DEBUG, "timestamp verified");

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
