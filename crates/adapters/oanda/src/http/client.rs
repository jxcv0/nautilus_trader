// -------------------------------------------------------------------------------------------------
//  Copyright (C) 2015-2025 Nautech Systems Pty Ltd. All rights reserved.
//  https://nautechsystems.io
//
//  Licensed under the GNU Lesser General Public License Version 3.0 (the "License");
//  You may not use this file except in compliance with the License.
//  You may obtain a copy of the License at https://www.gnu.org/licenses/lgpl-3.0.en.html
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
// -------------------------------------------------------------------------------------------------

//! Provides the HTTP client integration for the [OANDA](https://www.oanda.com/) V20 REST API
//!
//! OANDA V20 api reference <https://developer.oanda.com/rest-live-v20/development-guide/>

use std::{collections::HashMap, fmt::Debug, num::NonZeroU32, sync::LazyLock};

use super::error::OandaHttpError;
use crate::common::{credential::Credential, enums::OandaEnvironment, urls::oanda_http_base_url};
use nautilus_core::consts::NAUTILUS_USER_AGENT;
use nautilus_network::{http::HttpClient, ratelimiter::quota::Quota};
use reqwest::header::USER_AGENT;

/// OANDA implements a rate limit of 120 requests per second against requesting IP address.
/// Excess requests receive HTTP 429 error.
pub static OANDA_REST_QUOTA: LazyLock<Quota> =
    LazyLock::new(|| Quota::per_second(NonZeroU32::new(120).expect("120 is a valid non-zero u32")));

/// Inner HTTP client implementation containing HTTP logic
pub struct OandaHttpInnerClient {
    base_url: String,
    client: HttpClient,
    credential: Option<Credential>,
}

impl Default for OandaHttpInnerClient {
    fn default() -> Self {
        Self::new(None, None).expect("Failed to create default OandaHttpInnerClient")
    }
}

impl Debug for OandaHttpInnerClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OandaHttpInnerClient")
            .field("base_url", &self.base_url)
            .field("has_credentials", &self.credential.is_some())
            .finish()
    }
}

impl OandaHttpInnerClient {
    pub fn new(
        base_url: Option<String>,
        timeout_secs: Option<u64>,
    ) -> Result<Self, OandaHttpError> {
        let base_url =
            base_url.unwrap_or_else(|| oanda_http_base_url(OandaEnvironment::FxTrade).to_string());

        let client = HttpClient::new(
            Self::default_headers(),
            vec![],
            vec![],
            Some(*OANDA_REST_QUOTA),
            timeout_secs,
        );
        Ok(Self {
            base_url,
            client,
            credential: None,
        })
    }

    fn default_headers() -> HashMap<String, String> {
        HashMap::from([(USER_AGENT.to_string(), NAUTILUS_USER_AGENT.to_string())])
    }
}
