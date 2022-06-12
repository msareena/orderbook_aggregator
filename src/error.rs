#[derive(Debug)]
pub enum OrderbookError {
    JsonParseError,
    NoConnectionError,
    SocketReadError,
    SubscriptionError,
}

impl std::error::Error for OrderbookError {}

impl std::fmt::Display for OrderbookError {
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
        OrderbookError::JsonParseError => {
            write!(f, "Error when parsing json data")
        }
        OrderbookError::NoConnectionError => {
            write!(f, "No client connection exists")
        },
        OrderbookError::SocketReadError => {
            write!(f, "Error when reading from socket connection")
        },
        OrderbookError::SubscriptionError => {
            write!(f, "Error subscribing to exchange orderbook")
        },
    }
}
}