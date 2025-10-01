#!/usr/bin/env python3
# -------------------------------------------------------------------------------------------------
#  Copyright (C) 2015-2025 Nautech Systems Pty Ltd. All rights reserved.
#  https://nautechsystems.io
#
#  Licensed under the GNU Lesser General Public License Version 3.0 (the "License");
#  You may not use this file except in compliance with the License.
#  You may obtain a copy of the License at https://www.gnu.org/licenses/lgpl-3.0.en.html
#
#  Unless required by applicable law or agreed to in writing, software
#  distributed under the License is distributed on an "AS IS" BASIS,
#  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
#  See the License for the specific language governing permissions and
#  limitations under the License.
# -------------------------------------------------------------------------------------------------

from nautilus_trader.adapters.oanda import OANDA
from nautilus_trader.adapters.oanda import OandaDataClientConfig
from nautilus_trader.adapters.oanda import OandaLiveDataClientFactory
from nautilus_trader.config import InstrumentProviderConfig
from nautilus_trader.config import LiveExecEngineConfig
from nautilus_trader.config import LoggingConfig
from nautilus_trader.config import TradingNodeConfig
from nautilus_trader.live.node import TradingNode
from nautilus_trader.model.data import BarType
from nautilus_trader.model.enums import BookType
from nautilus_trader.model.identifiers import InstrumentId
from nautilus_trader.model.identifiers import TraderId
from nautilus_trader.test_kit.strategies.tester_data import DataTester
from nautilus_trader.test_kit.strategies.tester_data import DataTesterConfig


# *** THIS IS A TEST STRATEGY WITH NO ALPHA ADVANTAGE WHATSOEVER. ***
# *** IT IS NOT INTENDED TO BE USED TO TRADE LIVE WITH REAL MONEY. ***

# Example symbols for different Oanda products
# Perpetual swap: XBTUSD (Bitcoin perpetual)
# Futures: XBTH25 (Bitcoin futures expiring March 2025)
# Alt perpetuals: ETHUSD, SOLUSD, etc.

symbol = "AUD_CAD"  # FX
# symbol = "XAU_USD"    # Commodity
# symbol = "UK100_GBP"  # Index

# Configure the trading node
config_node = TradingNodeConfig(
    trader_id=TraderId("TESTER-001"),
    logging=LoggingConfig(log_level="INFO", use_pyo3=True),
    exec_engine=LiveExecEngineConfig(
        reconciliation=False,  # Not applicable
    ),
    data_clients={
        OANDA: OandaDataClientConfig(
            bearer_token=None,  # 'OANDA_API_TOKEN' env var
            base_url_http=None,  # Override with custom endpoint
            base_url_ws=None,  # Override with custom endpoint
            instrument_provider=InstrumentProviderConfig(load_all=True),
            demo=False,  # If client uses the practice API
        ),
    },
    timeout_connection=30.0,
    timeout_reconciliation=10.0,
    timeout_disconnection=10.0,
    timeout_post_stop=1.0,
)

# Configure the data tester actor
config_tester = DataTesterConfig(
    instrument_ids=[InstrumentId.from_str(f"{symbol}.{OANDA}")],
    bar_types=[BarType.from_str(f"{symbol}.{OANDA}-1-MINUTE-LAST-EXTERNAL")],
    subscribe_instrument=True,
    # subscribe_quotes=True,
    # subscribe_trades=True,
    # subscribe_mark_prices=True,
    # subscribe_index_prices=True,
    # subscribe_funding_rates=True,
    # subscribe_bars=True,
    # subscribe_book_deltas=True,
    # subscribe_book_depth=True,
    # subscribe_book_at_interval=True,
    # book_type=BookType.L2_MBP,
    # book_depth=25,
    # book_interval_ms=10,
    # request_trades=True,
    # request_bars=True,
)

# Setup and run the trading node
node = TradingNode(config=config_node)

# Add the strategy to the node
node.trader.add_actor(DataTester(config=config_tester))

# Register the data client factory
node.add_data_client_factory(OANDA, OandaLiveDataClientFactory)
node.build()

# Run the node
try:
    node.run()
except KeyboardInterrupt:
    node.stop()
    node.dispose()
