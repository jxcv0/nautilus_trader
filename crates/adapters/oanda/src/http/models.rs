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

//! Data structures representing OANDA REST API payloads.

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Instrument name identifier. Used by clients to refer to an Instrument.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OandaInstrumentName {
    #[serde(rename = "AU200_AUD")]
    Au200Aud,
    AudCad,
    AudChf,
    AudHkd,
    AudJpy,
    AudNzd,
    AudSgd,
    AudUsd,
    BcoUsd,
    CadChf,
    CadHkd,
    CadJpy,
    CadSgd,
    Ch20Chf,
    ChfHkd,
    ChfJpy,
    ChfZar,
    ChinahHkd,
    #[serde(rename = "CN50_USD")]
    Cn50Usd,
    #[serde(rename = "CORN_USD")]
    CornUsd,
    #[serde(rename = "DE10YB_EUR")]
    De10YbEur,
    #[serde(rename = "DE30_EUR")]
    De30Eur,
    EspixEur,
    #[serde(rename = "EU50_EUR")]
    Eu50Eur,
    EurAud,
    EurCad,
    EurChf,
    EurCzk,
    EurDkk,
    EurGbp,
    EurHkd,
    EurHuf,
    EurJpy,
    EurNok,
    EurNzd,
    EurPln,
    EurSek,
    EurSgd,
    EurTry,
    EurUsd,
    EurZar,
    #[serde(rename = "FR40_EUR")]
    Fr40Eur,
    GbpAud,
    GbpCad,
    GbpChf,
    GbpHkd,
    GbpJpy,
    GbpNzd,
    GbpPln,
    GbpSgd,
    GbpUsd,
    GbpZar,
    #[serde(rename = "HK33_HKD")]
    Hk33Hkd,
    HkdJpy,
    #[serde(rename = "JP225_USD")]
    Jp225Usd,
    #[serde(rename = "JP225Y_JPY")]
    Jp225YJpy,
    #[serde(rename = "NAS100_USD")]
    Nas100Usd,
    NatgasUsd,
    #[serde(rename = "NL25_EUR")]
    Nl25Eur,
    NzdCad,
    NzdChf,
    NzdHkd,
    NzdJpy,
    NzdSgd,
    NzdUsd,
    #[serde(rename = "SG30_SGD")]
    Sg30Sgd,
    SgdChf,
    SgdJpy,
    SoybnUsd,
    #[serde(rename = "SPX500_USD")]
    Spx500Usd,
    SugarUsd,
    TryJpy,
    #[serde(rename = "UK100_GBP")]
    Uk100Gbp,
    #[serde(rename = "UK10YB_GBP")]
    Uk10YbGbp,
    #[serde(rename = "US2000_USD")]
    Us2000Usd,
    #[serde(rename = "US30_USD")]
    Us30Usd,
    #[serde(rename = "USB02Y_USD")]
    Usb02YUsd,
    #[serde(rename = "USB05Y_USD")]
    Usb05YUsd,
    #[serde(rename = "USB10Y_USD")]
    Usb10YUsd,
    #[serde(rename = "USB30Y_USD")]
    Usb30YUsd,
    UsdCad,
    UsdChf,
    UsdCnh,
    UsdCzk,
    UsdDkk,
    UsdHkd,
    UsdHuf,
    UsdJpy,
    UsdMxn,
    UsdNok,
    UsdPln,
    UsdSek,
    UsdSgd,
    UsdThb,
    UsdTry,
    UsdZar,
    WheatUsd,
    WticoUsd,
    XagAud,
    XagCad,
    XagChf,
    XagEur,
    XagGbp,
    XagHkd,
    XagJpy,
    XagNzd,
    XagSgd,
    XagUsd,
    XauAud,
    XauCad,
    XauChf,
    XauEur,
    XauGbp,
    XauHkd,
    XauJpy,
    XauNzd,
    XauSgd,
    XauUsd,
    XauXag,
    XcuUsd,
    XpdUsd,
    XptUsd,
    ZarJpy,
}

/// The type of the Instrument
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OandaInstrumentType {
    Currency,
    Cfd,
    Metal,
}

// Instrument specific commission
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OandaInstrumentComission {
    commission: Decimal,
    units_traded: Decimal,
    minimum_commission: Decimal,
}

// The behavior of the account regarding guaranteed stop loss orders for an instrument
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OandaGuaranteedStopLossOrderModeForInstrument {
    Disabled,
    Allowed,
    Required,
}

// Represents the total positions size that can exist within a given window for trades with
// guaranteed stop loss orders attached
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OandaGuaranteedStopLossOrderLevelRestriction {
    volume: Decimal,
    price_range: Decimal,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OandaDayOfWeek {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OandaFinancingDayOfWeek {
    day_of_week: OandaDayOfWeek,
    days_charged: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OandaInstrumentFinancing {
    long_rate: Decimal,
    short_rate: Decimal,
    financing_days_of_week: Vec<OandaFinancingDayOfWeek>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OandaTag {
    r#type: String,
    name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OandaInstrument {
    pub name: OandaInstrumentName,
    pub r#type: OandaInstrumentType,
    pub display_name: String,
    pub pip_location: i32,
    pub display_precision: i32,
    pub trade_units_precision: i32,
    pub minimum_trade_size: Decimal,
    pub maximum_trailing_stop_distance: Decimal,
    pub minimum_guaranteed_stop_loss_distance: Decimal,
    pub minimum_trailing_stop_distance: Decimal,
    pub maximum_position_size: Decimal,
    pub maximum_order_units: Decimal,
    pub margin_rate: Decimal,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commission: Option<OandaInstrumentComission>,
    pub guaranteed_stop_loss_order_mode: OandaGuaranteedStopLossOrderModeForInstrument,
    pub guaranteed_stop_loss_order_execution_premium: Decimal,
    pub guaranteed_stop_loss_order_level_restriction: OandaGuaranteedStopLossOrderLevelRestriction,
    pub financing: OandaInstrumentFinancing,
    pub tags: Vec<OandaTag>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OandaInstrumentsResponse {
    pub instruments: Vec<OandaInstrument>,
    #[serde(rename = "lastTransactionID")]
    pub last_transaction_id: String,
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use serde_json::json;

    use super::*;

    #[rstest]
    fn parse_instrument_json() {
        let val = json!(
        {
            "instruments": [
            {
                "name": "AUD_NZD",
                "type": "CURRENCY",
                "displayName": "AUD/NZD",
                "pipLocation": -4,
                "displayPrecision": 5,
                "tradeUnitsPrecision": 0,
                "minimumTradeSize": "1",
                "maximumTrailingStopDistance": "1.00000",
                "minimumTrailingStopDistance": "0.00050",
                "maximumPositionSize": "0",
                "maximumOrderUnits": "100000000",
                "marginRate": "0.05",
                "guaranteedStopLossOrderMode": "ALLOWED",
                "minimumGuaranteedStopLossDistance": "0.0010",
                "guaranteedStopLossOrderExecutionPremium": "0.001",
                "guaranteedStopLossOrderLevelRestriction": {
                    "volume": "1000000",
                    "priceRange": "0.0025"
                },
                "tags": [
                    {
                        "type": "ASSET_CLASS",
                        "name": "CURRENCY"
                    },
                    {
                        "type": "BRAIN_ASSET_CLASS",
                        "name": "FX"
                    }
                ],
                "financing": {
                    "longRate": "-0.0047",
                    "shortRate": "-0.0173",
                    "financingDaysOfWeek": [
                        {
                            "dayOfWeek": "MONDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "TUESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "WEDNESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "THURSDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "FRIDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "SATURDAY",
                            "daysCharged": 0
                        },
                        {
                            "dayOfWeek": "SUNDAY",
                            "daysCharged": 0
                        }
                    ]
                }
            }
        ],
        "lastTransactionID": "6356"});

        let instrument = serde_json::from_value::<OandaInstrumentsResponse>(val).unwrap();
        // now check fields
        assert_eq!(instrument.instruments.len(), 1);
        let aud_nzd = &instrument.instruments[0];
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
        assert_eq!(aud_nzd.minimum_trailing_stop_distance, Decimal::new(50, 5));
        assert_eq!(aud_nzd.maximum_position_size, Decimal::new(0, 0));
        assert_eq!(aud_nzd.maximum_order_units, Decimal::new(100000000, 0));
        assert_eq!(aud_nzd.margin_rate, Decimal::new(5, 2));
        assert!(aud_nzd.commission.is_none());
        assert_eq!(
            aud_nzd.guaranteed_stop_loss_order_mode,
            OandaGuaranteedStopLossOrderModeForInstrument::Allowed
        );
        assert_eq!(
            aud_nzd.minimum_guaranteed_stop_loss_distance,
            Decimal::new(10, 4)
        );
        assert_eq!(
            aud_nzd.guaranteed_stop_loss_order_execution_premium,
            Decimal::new(1, 3)
        );
        assert_eq!(
            aud_nzd.guaranteed_stop_loss_order_level_restriction.volume,
            Decimal::new(1000000, 0)
        );
        assert_eq!(
            aud_nzd
                .guaranteed_stop_loss_order_level_restriction
                .price_range,
            Decimal::new(25, 4)
        );
        assert_eq!(aud_nzd.tags.len(), 2);
        assert_eq!(aud_nzd.tags[0].r#type, "ASSET_CLASS");
        assert_eq!(aud_nzd.tags[0].name, "CURRENCY");
        assert_eq!(aud_nzd.tags[1].r#type, "BRAIN_ASSET_CLASS");
        assert_eq!(aud_nzd.tags[1].name, "FX");
        assert_eq!(aud_nzd.financing.long_rate, Decimal::new(-47, 4));
        assert_eq!(aud_nzd.financing.short_rate, Decimal::new(-173, 4));
        assert_eq!(aud_nzd.financing.financing_days_of_week.len(), 7);
        assert_eq!(
            aud_nzd.financing.financing_days_of_week[0].day_of_week,
            OandaDayOfWeek::Monday
        );
        assert_eq!(aud_nzd.financing.financing_days_of_week[0].days_charged, 1);
        assert_eq!(instrument.last_transaction_id, "6356");
    }
}
