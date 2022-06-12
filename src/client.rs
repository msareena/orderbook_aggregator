use tokio::sync::mpsc;
use tonic::Status;

use crate::orderbook_aggregator::Summary;

#[derive(Debug, Clone)]
pub struct Client {
    pub sender: Option<mpsc::Sender<Result<Summary, Status>>>,
    pub symbol: String,
}

