use serde::{Deserialize, Serialize};
use serde_json::json;
use tungstenite::Message;

use crate::error::OrderbookError;
use crate::exchange::{Exchange, self, Socket};
use crate::order::{Quote, Order, summarise_order};
use crate::orderbook_aggregator::Summary;

static WSS_BASE_ENDPOINT: &str = "wss://ws.bitstamp.net";

#[derive(Clone, Deserialize, Serialize)]
pub struct BitstampOrder {
    pub timestamp: String,
    pub microtimestamp: String,
    pub bids: Vec<Quote>,
    pub asks: Vec<Quote>,
}

impl Order for BitstampOrder {
    fn exchange_name(&self) -> String {
        String::from("Bitstamp")
    }

    fn asks(&self) -> &Vec<Quote> {
        self.asks.as_ref()
    }

    fn bids(&self) -> &Vec<Quote> {
        self.bids.as_ref()
    }
}


#[derive(Deserialize)]
#[serde(untagged)]
pub enum Data {
    Order(BitstampOrder),
    None {},
}

#[derive(Deserialize)]
pub struct BitstampMsg {
    pub event: String,
    pub channel: String,
    pub data: Data,
}

pub struct Bitstamp {
    socket: Socket,
    symbol: String,
}

impl Exchange for Bitstamp {
    fn new(symbol: String) -> Self {
        let bitstamp_endpoint = format!("{}", WSS_BASE_ENDPOINT);

        Bitstamp {
            socket: exchange::connect(bitstamp_endpoint.as_str()),
            symbol,
        }
    }

    fn stream(&mut self) -> Result<Option<Summary>, OrderbookError> {
        let socket = match &mut self.socket {
            Some(socket) => socket,
            None => {
                panic!("No existing connection to Binance exchange");
            }
        };

        let subscribe_message = json!(
        {
            "event": "bts:subscribe",
            "data": {
                "channel": format!("order_book_{}", self.symbol)
            }
        });

        match socket.write_message(Message::Text(subscribe_message.to_string())) {
            Err(_) => {
                return Err(OrderbookError::SubscriptionError);
            },
            _ => ()
        };

        let mut last_sent_order: Option<BitstampOrder> = None;

        let is_new_order = |order: &BitstampOrder| -> bool {
            if last_sent_order.is_some() {
                let last_order = last_sent_order.unwrap();
                return order.timestamp != last_order.timestamp &&
                       order.microtimestamp != last_order.microtimestamp
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

            let bitstamp_msg: BitstampMsg = match serde_json::from_str(&msg) {
                Ok(bitstamp_msg) => bitstamp_msg,
                Err(_) => {
                    return Err(OrderbookError::JsonParseError);
                }
            };

            if bitstamp_msg.event == "data" {
                if let Data::Order(order) = bitstamp_msg.data {
                    if is_new_order(&order) {
                        last_sent_order = Some(order.clone());
                        return Ok(Some(summarise_order(order)));
                    }
                    else {
                        return Ok(None);
                    }
                }
            }
        }
    }
}
