use serde::{Deserialize, Serialize};
use tungstenite::Message;

use crate::{orderbook_aggregator::Summary, exchange::{Exchange, self, Socket}, order::{Order, summarise_order}, error::OrderbookError};
use crate::order::Quote;

static WSS_BASE_ENDPOINT: &str = "wss://stream.binance.com:9443";

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BinanceOrder {
    pub last_update_id: usize,
    pub asks: Vec<Quote>,
    pub bids: Vec<Quote>,
}

impl Order for BinanceOrder {
    fn exchange_name(&self) -> String {
        String::from("Binance")
    }

    fn asks(&self) -> &Vec<Quote> {
        self.asks.as_ref()
    }

    fn bids(&self) -> &Vec<Quote> {
        self.bids.as_ref()
    }
}

pub struct Binance {
    socket: Socket,
}

impl Exchange for Binance {
    fn new(symbol: String) -> Self {
        let binance_endpoint = format!(
            "{}/ws/{}@depth10@100ms",
            WSS_BASE_ENDPOINT, symbol);

        Binance {
            socket: exchange::connect(binance_endpoint.as_str()),
        }
    }

    fn stream(&mut self) -> Result<Option<Summary>, OrderbookError> {
        let socket = match &mut self.socket {
            Some(socket) => socket,
            None => {
                return Err(OrderbookError::NoConnectionError);
            }
        };

        let mut last_sent_order: Option<BinanceOrder> = None;

        let is_new_order = |curr_order: &BinanceOrder, last_sent_order: Option<BinanceOrder>| -> bool {
            if last_sent_order.is_some() {
                let last_order = last_sent_order.unwrap();
                return curr_order.last_update_id != last_order.last_update_id
            }

            true
        };

        loop {
            let msg = match socket.read_message() {
                Ok(Message::Text(msg)) => msg,
                Err(tungstenite::Error::ConnectionClosed) => {
                    return Err(OrderbookError::NoConnectionError);
                }
                _ => {
                    return Err(OrderbookError::SocketReadError);
                }
            };

            let order: BinanceOrder = match serde_json::from_str(&msg) {
                Ok(order) => order,
                Err(_) => {
                    return Err(OrderbookError::JsonParseError);
                }
            };

            if is_new_order(&order, last_sent_order) {
                last_sent_order = Some(order.clone());
                return Ok(Some(summarise_order(order)));
            }
            else {
                return Ok(None);
            }
        }
    }
}
