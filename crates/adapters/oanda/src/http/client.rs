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

use std::{
    collections::HashMap,
    fmt::Debug,
    num::NonZeroU32,
    sync::{Arc, LazyLock},
};

use super::error::OandaHttpError;
use crate::{
    common::{credential::Credential, enums::OandaEnvironment, urls::oanda_http_base_url},
    http::models::{OandaInstrument, OandaInstrumentsResponse},
};
use nautilus_core::consts::NAUTILUS_USER_AGENT;
use nautilus_network::{
    http::HttpClient,
    ratelimiter::quota::Quota,
    retry::{RetryConfig, RetryManager},
};
use reqwest::{Method, header::USER_AGENT};
use serde::de::DeserializeOwned;
use tokio_util::sync::CancellationToken;

/// OANDA implements a rate limit of 120 requests per second against requesting IP address.
/// Excess requests receive HTTP 429 error.
pub static OANDA_REST_QUOTA: LazyLock<Quota> =
    LazyLock::new(|| Quota::per_second(NonZeroU32::new(120).expect("120 is a valid non-zero u32")));

/// Inner HTTP client implementation containing HTTP logic.
pub struct OandaHttpInnerClient {
    base_url: String,
    client: HttpClient,
    credential: Option<Credential>,
    retry_manager: RetryManager<OandaHttpError>,
    cancellation_token: CancellationToken,
}

impl Default for OandaHttpInnerClient {
    fn default() -> Self {
        Self::new(None, Some(60), None, None, None)
            .expect("Failed to create default OandaHttpInnerClient")
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
    /// Creates a new [`OandaHttpInnerClient`] using the default OANDA HTTP URL.
    ///
    /// # Errors
    ///
    /// Returns an error if the retry manager cannot be created.
    pub fn new(
        base_url: Option<String>,
        timeout_secs: Option<u64>,
        max_retries: Option<u32>,
        retry_delay_ms: Option<u64>,
        retry_delay_max_ms: Option<u64>,
    ) -> Result<Self, OandaHttpError> {
        let retry_config = RetryConfig {
            max_retries: max_retries.unwrap_or(3),
            initial_delay_ms: retry_delay_ms.unwrap_or(1000),
            max_delay_ms: retry_delay_max_ms.unwrap_or(10_000),
            backoff_factor: 2.0,
            jitter_ms: 1000,
            operation_timeout_ms: Some(60_000),
            immediate_first: false,
            max_elapsed_ms: Some(180_000),
        };

        let retry_manager = RetryManager::new(retry_config).map_err(|e| {
            OandaHttpError::NetworkError(format!("Failed to create retry manager: {e}"))
        })?;

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
            retry_manager,
            cancellation_token: CancellationToken::new(),
        })
    }

    /// Creates a new [`OandaHttpInnerClient`] configured with credentials.
    ///
    /// # Errors
    ///
    /// Returns an error if the retry manager cannot be created.
    pub fn with_credentials(
        bearer_token: String,
        base_url: Option<String>,
        timeout_secs: Option<u64>,
        max_retries: Option<u32>,
        retry_delay_ms: Option<u64>,
        retry_delay_max_ms: Option<u64>,
    ) -> Result<Self, OandaHttpError> {
        let retry_config = RetryConfig {
            max_retries: max_retries.unwrap_or(3),
            initial_delay_ms: retry_delay_ms.unwrap_or(1000),
            max_delay_ms: retry_delay_max_ms.unwrap_or(10_000),
            backoff_factor: 2.0,
            jitter_ms: 1000,
            operation_timeout_ms: Some(60_000),
            immediate_first: false,
            max_elapsed_ms: Some(180_000),
        };

        let retry_manager = RetryManager::new(retry_config).map_err(|e| {
            OandaHttpError::NetworkError(format!("Failed to create retry manager: {e}"))
        })?;

        Ok(Self {
            base_url: base_url
                .unwrap_or_else(|| oanda_http_base_url(OandaEnvironment::FxTrade).to_string()),
            client: HttpClient::new(
                Self::default_headers(),
                vec![],
                vec![],
                Some(*OANDA_REST_QUOTA),
                timeout_secs,
            ),
            credential: Some(Credential::new(bearer_token)),
            retry_manager,
            cancellation_token: CancellationToken::new(),
        })
    }

    /// Returns the base URL used for requests.
    #[must_use]
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Returns the API credential if configured.
    #[must_use]
    pub fn credential(&self) -> Option<&Credential> {
        self.credential.as_ref()
    }

    fn default_headers() -> HashMap<String, String> {
        HashMap::from([(USER_AGENT.to_string(), NAUTILUS_USER_AGENT.to_string())])
    }

    /// Get the list of tradeable instruments for a given account
    /// TODO: Where do we store the account?
    pub async fn http_get_instruments(
        &self,
        account_id: &str,
    ) -> Result<Vec<OandaInstrument>, OandaHttpError> {
        let path = format!("/v3/accounts/{account_id}/instruments");
        let res: OandaInstrumentsResponse =
            self.send_request(Method::GET, &path, None, false).await?;
        Ok(res.instruments)
    }

    fn bearer_auth_headers(&self) -> Option<HashMap<String, String>> {
        let credential = self.credential.as_ref();
        credential.map(|credential| {
            HashMap::from([("bearer auth".to_string(), credential.bearer_token())])
        })
    }

    async fn send_request<T: DeserializeOwned>(
        &self,
        method: Method,
        endpoint: &str,
        body: Option<Vec<u8>>,
        authenticate: bool,
    ) -> Result<T, OandaHttpError> {
        if authenticate && self.credential.is_none() {
            return Err(OandaHttpError::MissingCredentials);
        }

        let endpoint = endpoint.to_string();
        let url = format!("{}{endpoint}", self.base_url);
        let method_clone = method.clone();
        let body_clone = body.clone();

        let operation = || {
            let url = url.clone();
            let method = method_clone.clone();
            let body = body_clone.clone();

            async move {
                let headers = if authenticate {
                    self.bearer_auth_headers()
                } else {
                    None
                };

                let resp = self
                    .client
                    .request(method.clone(), url, headers, body, None, None)
                    .await?;

                if resp.status.is_success() {
                    serde_json::from_slice::<T>(&resp.body).map_err(Into::into)
                } else {
                    Err(OandaHttpError::NetworkError("TODO".to_string()))
                }
            }
        };

        // Retry strategy:
        //
        // 1. Network errors: always retry (transient connection issues)
        // 2. HTTP 5xx/429: server errors and rate limiting should be retried
        //
        // TODO: Errors that should not be retried (invalid requests/account balance)
        let should_retry = |error: &OandaHttpError| -> bool {
            match error {
                OandaHttpError::NetworkError(_) => true,
                OandaHttpError::UnexpectedStatus { status, .. } => *status >= 500 || *status == 429,
                _ => false,
            }
        };

        let create_error = |msg: String| -> OandaHttpError { OandaHttpError::NetworkError(msg) };

        self.retry_manager
            .execute_with_retry_with_cancel(
                endpoint.as_str(),
                operation,
                should_retry,
                create_error,
                &self.cancellation_token,
            )
            .await
    }
}

////////////////////////////////////////////////////////////////////////////////
// Outer Client
////////////////////////////////////////////////////////////////////////////////

/// Provides the HTTP client for connecting to the [OANDA](https://www.oanda.com/) V20 REST API
#[derive(Clone, Debug)]
#[cfg_attr(
    feature = "python",
    pyo3::pyclass(module = "nautilus_trader.core.nautilus_pyo3.adapters")
)]
pub struct OandaHttpClient {
    pub(crate) inner: Arc<OandaHttpInnerClient>,
}

impl Default for OandaHttpClient {
    fn default() -> Self {
        Self::new(None, Some(60), None, None, None)
            .expect("Failed to create default OandaHttpClient")
    }
}

impl OandaHttpClient {
    /// Creates a new [`OandaHttpClient`] using the default OANDA HTTP URL.
    ///
    /// # Errors
    ///
    /// Returns an error if the retry manager cannot be created.
    pub fn new(
        base_url: Option<String>,
        timeout_secs: Option<u64>,
        max_retries: Option<u32>,
        retry_delay_ms: Option<u64>,
        retry_delay_max_ms: Option<u64>,
    ) -> Result<Self, OandaHttpError> {
        Ok(Self {
            inner: Arc::new(OandaHttpInnerClient::new(
                base_url,
                timeout_secs,
                max_retries,
                retry_delay_ms,
                retry_delay_max_ms,
            )?),
        })
    }

    pub fn with_credentials(
        bearer_token: String,
        base_url: Option<String>,
        timeout_secs: Option<u64>,
        max_retries: Option<u32>,
        retry_delay_ms: Option<u64>,
        retry_delay_max_ms: Option<u64>,
    ) -> Result<Self, OandaHttpError> {
        Ok(Self {
            inner: Arc::new(OandaHttpInnerClient::with_credentials(
                bearer_token,
                base_url,
                timeout_secs,
                max_retries,
                retry_delay_ms,
                retry_delay_max_ms,
            )?),
        })
    }

    /// Returns the base URL used for requests.
    pub fn base_url(&self) -> &str {
        self.inner.base_url()
    }

    /// Returns the API credential if configured.
    #[must_use]
    pub fn credential(&self) -> Option<&Credential> {
        self.inner.credential()
    }

    pub async fn http_get_instruments(
        &self,
        account_id: &str,
    ) -> Result<Vec<OandaInstrument>, OandaHttpError> {
        self.inner.http_get_instruments(account_id).await
    }
}

////////////////////////////////////////////////////////////////////////////////
// Tests
////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::consts::OANDA_HTTP_URL;

    use rstest::rstest;

    #[rstest]
    fn test_client_creation() {
        let client = OandaHttpClient::new(None, Some(60), None, None, None);
        assert!(client.is_ok());

        let client = client.unwrap();
        assert_eq!(client.base_url(), OANDA_HTTP_URL);
        assert!(client.credential().is_none());
    }

    #[rstest]
    fn test_client_with_credentials() {
        let client = OandaHttpClient::with_credentials(
            "test_token".to_string(),
            None,
            Some(60),
            None,
            None,
            None,
        );
        assert!(client.is_ok());

        let client = client.unwrap();
        assert!(client.credential().is_some());
    }
}
