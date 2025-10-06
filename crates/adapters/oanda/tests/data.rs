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

//! Integration-style tests for the OANDA data client scaffolding.

use std::sync::OnceLock;
use std::time::Duration;

use nautilus_common::{
    messages::{
        DataEvent,
        data::{DataResponse, RequestInstrument, RequestInstruments},
    },
    runner::set_data_event_sender,
};
use nautilus_core::{UUID4, UnixNanos};
use nautilus_data::client::DataClient;
use nautilus_model::identifiers::{ClientId, InstrumentId, Symbol};
use nautilus_oanda::{
    common::consts::OANDA_VENUE, config::OandaDataClientConfig, data::OandaDataClient,
};
use tokio::{sync::Mutex, time::timeout};

fn data_event_receiver() -> &'static Mutex<tokio::sync::mpsc::UnboundedReceiver<DataEvent>> {
    static RECEIVER: OnceLock<Mutex<tokio::sync::mpsc::UnboundedReceiver<DataEvent>>> =
        OnceLock::new();
    RECEIVER.get_or_init(|| {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        set_data_event_sender(tx);
        Mutex::new(rx)
    })
}

#[tokio::test]
async fn data_client_handles_basic_lifecycle_and_requests() {
    // Ensure we have a receiver hooked up before constructing the client.
    let receiver = data_event_receiver();

    // Drain any leftover events from previous test runs.
    {
        let mut rx = receiver.lock().await;
        while rx.try_recv().is_ok() {}
    }

    let client_id = ClientId::new("OANDA-DATA-TEST");
    let config = OandaDataClientConfig::default();
    let mut client = OandaDataClient::new(client_id, config).expect("client construction succeeds");

    assert!(client.is_disconnected());
    client.start().expect("start succeeds");
    client.connect().await.expect("connect succeeds");
    assert!(client.is_connected());

    // Request instruments and ensure we receive an empty response (bootstrap is not implemented yet).
    let instruments_request = RequestInstruments::new(
        None,
        None,
        Some(client_id),
        Some(*OANDA_VENUE),
        UUID4::new(),
        UnixNanos::from(0_u64),
        None,
    );
    client
        .request_instruments(&instruments_request)
        .expect("request_instruments should succeed");

    {
        let mut rx = receiver.lock().await;
        let event = timeout(Duration::from_millis(500), rx.recv())
            .await
            .expect("timed out waiting for instruments response")
            .expect("data event channel closed unexpectedly");
        match event {
            DataEvent::Response(DataResponse::Instruments(response)) => {
                assert!(
                    response.data.is_empty(),
                    "expected no cached instruments yet"
                );
                assert_eq!(response.client_id, client_id);
                assert_eq!(response.venue, *OANDA_VENUE);
                assert!(rx.try_recv().is_err(), "unexpected extra events");
            }
            other => panic!("unexpected data event: {other:?}"),
        }
    }

    // Requesting a specific instrument without bootstrap data should return an error.
    let instrument_id = InstrumentId::new(Symbol::new("EUR_USD"), *OANDA_VENUE);
    let instrument_request = RequestInstrument::new(
        instrument_id,
        None,
        None,
        Some(client_id),
        UUID4::new(),
        UnixNanos::from(0_u64),
        None,
    );
    assert!(
        client.request_instrument(&instrument_request).is_err(),
        "expected an error for unknown instrument"
    );

    client.disconnect().await.expect("disconnect succeeds");
    assert!(client.is_disconnected());
    client.stop().expect("stop succeeds");
}
