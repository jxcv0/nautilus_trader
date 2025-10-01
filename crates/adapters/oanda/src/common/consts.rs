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

//! OANDA adapter constants including base URLs and the venue identifier.

use std::sync::LazyLock;

use nautilus_model::identifiers::Venue;
use ustr::Ustr;

pub const OANDA: &str = "OANDA";

pub const OANDA_HTTP_URL: &str = "https://api-fxtrade.oanda.com";
pub const OANDA_HTTP_PRACTICE_URL: &str = "https://api-fxpractice.oanda.com";

pub static OANDA_VENUE: LazyLock<Venue> = LazyLock::new(|| Venue::new(Ustr::from(OANDA)));
