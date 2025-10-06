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

//! Error structures and enumerations for the OANDA integration.

use nautilus_network::http::HttpClientError;
use thiserror::Error;

/// A typed error enumeration for the OANDA HTTP client.
#[derive(Debug, Clone, Error)]
pub enum OandaHttpError {
    /// Error variant when credentials are missing but the request is authenticated.
    #[error("Missing credentials for authenticated request")]
    MissingCredentials,
    /// Generic network error (for retries, cancellations, etc).
    #[error("Network error: {0}")]
    NetworkError(String),
    /// Failure during JSON serialization/deserialization.
    #[error("JSON error: {0}")]
    JsonError(String),
    /// Any unknown HTTP status or unexpected response from OANDA.
    #[error("Unexpected HTTP status code {status}: {body}")]
    UnexpectedStatus { status: u16, body: String },
}

impl From<HttpClientError> for OandaHttpError {
    fn from(error: HttpClientError) -> Self {
        Self::NetworkError(error.to_string())
    }
}

// Allow use of the `?` operator on `serde_json` results inside the HTTP
// client implementation by converting them into our typed error.
impl From<serde_json::Error> for OandaHttpError {
    fn from(error: serde_json::Error) -> Self {
        Self::JsonError(error.to_string())
    }
}
