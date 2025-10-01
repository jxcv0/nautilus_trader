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

//! Configuration types for the OANDA adapter clients.

use crate::common::{enums::OandaEnvironment, urls::oanda_http_base_url};

/// Configuration for the OANDA live data client.
#[derive(Clone, Debug)]
pub struct OandaDataClientConfig {
    /// Bearer token used for authenticated REST requests.
    pub bearer_token: Option<String>,
    /// Optional override for the REST base URL.
    pub base_url_http: Option<String>,
    /// When `true`, connect to the practice (demo) environment.
    pub demo: bool,
    /// Optional REST timeout in seconds.
    pub http_timeout_secs: Option<u64>,
    /// Optional maximum retry attempts for REST requests.
    pub max_retries: Option<u32>,
    /// Optional initial retry backoff in milliseconds.
    pub retry_delay_initial_ms: Option<u64>,
    /// Optional maximum retry backoff in milliseconds.
    pub retry_delay_max_ms: Option<u64>,
    /// Optional interval (minutes) for instrument refresh from REST.
    pub update_instruments_interval_mins: Option<u64>,
}

impl Default for OandaDataClientConfig {
    fn default() -> Self {
        Self {
            bearer_token: None,
            base_url_http: None,
            demo: false,
            http_timeout_secs: Some(60),
            max_retries: None,
            retry_delay_initial_ms: Some(1_000),
            retry_delay_max_ms: Some(5_000),
            update_instruments_interval_mins: Some(60),
        }
    }
}

impl OandaDataClientConfig {
    /// Creates a configuration with default values.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the OANDA environment (practice vs live).
    #[must_use]
    pub fn environment(&self) -> OandaEnvironment {
        if self.demo {
            OandaEnvironment::FxTradePractice
        } else {
            OandaEnvironment::FxTrade
        }
    }

    /// Returns the REST base URL, considering overrides and the environment flag.
    #[must_use]
    pub fn http_base_url(&self) -> String {
        self.base_url_http
            .clone()
            .unwrap_or_else(|| oanda_http_base_url(self.environment()).to_string())
    }
}
