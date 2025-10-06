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

//! Basic configuration tests for the OANDA HTTP integration.

use nautilus_oanda::{
    common::{
        consts::{OANDA_HTTP_PRACTICE_URL, OANDA_HTTP_URL},
        enums::OandaEnvironment,
    },
    config::OandaDataClientConfig,
    http::{
        client::OandaHttpClient,
        models::{OandaDayOfWeek, OandaInstrumentName, OandaInstrumentType},
    },
};
use rust_decimal::Decimal;

use rstest::rstest;
use serde_json::Value;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn load_test_data(filename: &str) -> Value {
    let path = format!("test_data/{}", filename);
    let content = std::fs::read_to_string(path).expect("Failed to read test data");
    serde_json::from_str(&content).expect("Failed to parse test data")
}

async fn start_mock_server(account: &str) -> MockServer {
    let mock_server = MockServer::start().await;

    let instruments_template =
        ResponseTemplate::new(200).set_body_json(load_test_data("http_get_instruments.json"));

    Mock::given(method("GET"))
        .and(path(format!("/v3/accounts/{account}/instruments")))
        .respond_with(instruments_template)
        .mount(&mock_server)
        .await;

    mock_server
}

#[rstest]
fn config_resolves_environment_and_urls() {
    let default = OandaDataClientConfig::default();
    assert_eq!(default.environment(), OandaEnvironment::FxTrade);
    assert_eq!(default.http_base_url(), OANDA_HTTP_URL);

    let practice = OandaDataClientConfig {
        demo: true,
        ..Default::default()
    };
    assert_eq!(practice.environment(), OandaEnvironment::FxTradePractice);
    assert_eq!(practice.http_base_url(), OANDA_HTTP_PRACTICE_URL);

    let override_config = OandaDataClientConfig {
        base_url_http: Some("https://example.oanda.test".to_string()),
        ..Default::default()
    };
    assert_eq!(
        override_config.http_base_url(),
        "https://example.oanda.test"
    );
}

#[rstest]
#[tokio::test]
async fn http_get_instruments() {
    let mock_server = start_mock_server("123").await;

    let client = OandaHttpClient::new(Some(mock_server.uri()), None, None, None, None).unwrap();
    let instruments = client
        .http_get_instruments("123")
        .await
        .expect("http_get_instruments failed");

    // now check fields
    let aud_nzd = &instruments[0];
    assert_eq!(aud_nzd.name, OandaInstrumentName::AudNzd);
    assert_eq!(aud_nzd.r#type, OandaInstrumentType::Currency);
    assert_eq!(aud_nzd.display_name, "AUD/NZD");
    assert_eq!(aud_nzd.pip_location, -4);
    assert_eq!(aud_nzd.display_precision, 5);
    assert_eq!(aud_nzd.trade_units_precision, 0);
    assert_eq!(aud_nzd.minimum_trade_size, Decimal::new(1, 0));
    assert_eq!(
        aud_nzd.maximum_trailing_stop_distance,
        Decimal::new(100000, 5)
    );
}
