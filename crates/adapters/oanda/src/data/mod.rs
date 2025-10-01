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

//! Data client implementation for the OANDA adapter.

use std::{
    convert::TryFrom,
    sync::{
        Arc, RwLock,
        atomic::{AtomicBool, Ordering},
    },
};

use ahash::AHashMap;
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use nautilus_common::{
    messages::{
        DataEvent,
        data::{
            DataResponse, InstrumentResponse, InstrumentsResponse, RequestInstrument,
            RequestInstruments,
        },
    },
    runner::get_data_event_sender,
};
use nautilus_core::{
    UnixNanos,
    time::{AtomicTime, get_atomic_clock_realtime},
};
use nautilus_data::client::DataClient;
use nautilus_model::{
    identifiers::{ClientId, InstrumentId, Venue},
    instruments::{InstrumentAny, Instrument},
};
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

use crate::{
    common::consts::OANDA_VENUE, config::OandaDataClientConfig, http::client::OandaHttpClient,
};

#[derive(Debug)]
pub struct OandaDataClient {
    client_id: ClientId,
    config: OandaDataClientConfig,
    http_client: OandaHttpClient,
    is_connected: AtomicBool,
    cancellation_token: CancellationToken,
    tasks: Vec<JoinHandle<()>>,
    data_sender: tokio::sync::mpsc::UnboundedSender<DataEvent>,
    instruments: Arc<RwLock<AHashMap<InstrumentId, InstrumentAny>>>,
    clock: &'static AtomicTime,
    instrument_refresh_active: bool,
}

impl OandaDataClient {
    /// Creates a new [`OandaDataClient`] instance.
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP client fails to initialize.
    pub fn new(client_id: ClientId, config: OandaDataClientConfig) -> Result<Self> {
        let clock = get_atomic_clock_realtime();
        let data_sender = get_data_event_sender();

        let http_client = if let Some(token) = &config.bearer_token {
            OandaHttpClient::with_credentials(
                token.clone(),
                Some(config.http_base_url()),
                config.http_timeout_secs,
                config.max_retries,
                config.retry_delay_initial_ms,
                config.retry_delay_max_ms,
            )
            .context("failed to construct OANDA HTTP client with credentials")?
        } else {
            OandaHttpClient::new(
                Some(config.http_base_url()),
                config.http_timeout_secs,
                config.max_retries,
                config.retry_delay_initial_ms,
                config.retry_delay_max_ms,
            )
            .context("failed to construct OANDA HTTP client")?
        };

        Ok(Self {
            client_id,
            config,
            http_client,
            is_connected: AtomicBool::new(false),
            cancellation_token: CancellationToken::new(),
            tasks: Vec::new(),
            data_sender,
            instruments: Arc::new(RwLock::new(AHashMap::new())),
            clock,
            instrument_refresh_active: false,
        })
    }

    fn venue(&self) -> Venue {
        *OANDA_VENUE
    }

    fn get_instrument(&self, instrument_id: &InstrumentId) -> Option<InstrumentAny> {
        let instruments = self.instruments.read().ok()?;
        instruments.get(instrument_id).cloned()
    }

    async fn bootstrap_instruments(&mut self) -> Result<()> {
        tracing::debug!("OANDA instrument bootstrap not implemented yet");
        Ok(())
    }
}

#[async_trait::async_trait]
impl DataClient for OandaDataClient {
    fn client_id(&self) -> ClientId {
        self.client_id
    }

    fn venue(&self) -> Option<Venue> {
        Some(self.venue())
    }

    fn start(&mut self) -> Result<()> {
        tracing::info!("Starting OANDA data client {}", self.client_id);
        Ok(())
    }

    fn stop(&mut self) -> Result<()> {
        tracing::info!("Stopping OANDA data client {}", self.client_id);
        self.cancellation_token.cancel();
        for task in self.tasks.drain(..) {
            task.abort();
        }
        self.is_connected.store(false, Ordering::Release);
        Ok(())
    }

    fn reset(&mut self) -> Result<()> {
        tracing::debug!("Resetting OANDA data client {}", self.client_id);
        self.stop()?;
        self.cancellation_token = CancellationToken::new();
        if let Ok(mut instruments) = self.instruments.write() {
            instruments.clear();
        }
        self.instrument_refresh_active = false;
        Ok(())
    }

    fn dispose(&mut self) -> Result<()> {
        tracing::debug!("Disposing OANDA data client {}", self.client_id);
        self.stop()
    }

    fn is_connected(&self) -> bool {
        self.is_connected.load(Ordering::Acquire)
    }

    fn is_disconnected(&self) -> bool {
        !self.is_connected()
    }

    async fn connect(&mut self) -> Result<()> {
        if self.is_connected() {
            return Ok(());
        }

        tracing::info!("Connecting OANDA data client {}", self.client_id);

        self.cancellation_token = CancellationToken::new();

        self.bootstrap_instruments().await?;

        self.is_connected.store(true, Ordering::Release);
        tracing::info!("OANDA data client {} connected", self.client_id);
        Ok(())
    }

    async fn disconnect(&mut self) -> Result<()> {
        if !self.is_connected() {
            return Ok(());
        }

        tracing::info!("Disconnecting OANDA data client {}", self.client_id);

        self.cancellation_token.cancel();
        for task in self.tasks.drain(..) {
            if let Err(err) = task.await {
                tracing::error!("Task error during shutdown: {err}");
            }
        }

        if let Ok(mut instruments) = self.instruments.write() {
            instruments.clear();
        }

        self.cancellation_token = CancellationToken::new();
        self.is_connected.store(false, Ordering::Release);
        tracing::info!("OANDA data client {} disconnected", self.client_id);
        Ok(())
    }

    fn request_instruments(&self, request: &RequestInstruments) -> Result<()> {
        let instruments = {
            let guard = self.instruments.read();
            match guard {
                Ok(map) => map.values().cloned().collect(),
                Err(_) => Vec::new(),
            }
        };

        let response = DataResponse::Instruments(InstrumentsResponse::new(
            request.request_id,
            request.client_id.unwrap_or(self.client_id),
            self.venue(),
            instruments,
            datetime_to_unix_nanos(request.start),
            datetime_to_unix_nanos(request.end),
            self.clock.get_time_ns(),
            request.params.clone(),
        ));

        if let Err(err) = self.data_sender.send(DataEvent::Response(response)) {
            tracing::error!("Failed to send instruments response: {err}");
        }

        Ok(())
    }

    fn request_instrument(&self, request: &RequestInstrument) -> Result<()> {
        let instrument = self
            .get_instrument(&request.instrument_id)
            .ok_or_else(|| anyhow::anyhow!("Instrument {} not found", request.instrument_id))?;

        let response = DataResponse::Instrument(Box::new(InstrumentResponse::new(
            request.request_id,
            request.client_id.unwrap_or(self.client_id),
            instrument.id(),
            instrument,
            datetime_to_unix_nanos(request.start),
            datetime_to_unix_nanos(request.end),
            self.clock.get_time_ns(),
            request.params.clone(),
        )));

        if let Err(err) = self.data_sender.send(DataEvent::Response(response)) {
            tracing::error!("Failed to send instrument response: {err}");
        }

        Ok(())
    }
}

fn datetime_to_unix_nanos(value: Option<DateTime<Utc>>) -> Option<UnixNanos> {
    // TODO: This function is reimplemented in multiple places. We should consider centralizing it.
    value
        .and_then(|dt| dt.timestamp_nanos_opt())
        .and_then(|nanos| u64::try_from(nanos).ok())
        .map(UnixNanos::from)
}
