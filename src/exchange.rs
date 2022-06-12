use tungstenite::{WebSocket, stream::MaybeTlsStream};
use url::Url;

use crate::{orderbook_aggregator::Summary, error::OrderbookError};

pub type Socket = Option<WebSocket<MaybeTlsStream<std::net::TcpStream>>>;

pub trait Exchange {
    fn new(symbol: String) -> Self;

    fn stream(&mut self) -> Result<Option<Summary>, OrderbookError>;
}

/// Connect to the exchange specified by the websocket endpoint.
/// If connection is successful a tcp streaming websocket is returned to the
/// caller. Otherwise `None` is returned.
pub fn connect(ws_endpoint: &str) ->
    Option<WebSocket<MaybeTlsStream<std::net::TcpStream>>> {
    let url = match Url::parse(&ws_endpoint) {
        Ok(url) => url,
        _ => {
            println!("Could not parse the WSS endpoint {}", ws_endpoint);
            return None;
        }
    };

    let (socket, _) = match tungstenite::client::connect(url.clone()) {
        Ok(result) => result,
        _ => {
            println!("Could not connect to URL: {}", url);
            return None;
        }
    };

    println!("Connected to Websocket URL {}", url);

    Some(socket)
}