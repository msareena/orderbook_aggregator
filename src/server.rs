use std::sync::Arc;

use crate::client::Client;
use crate::worker::client_worker;
use orderbook_aggregator::{Summary, orderbook_aggregator_server::{OrderbookAggregator, OrderbookAggregatorServer}, Symbol};
use tokio::sync::{mpsc, Mutex};
use tokio_stream::wrappers::{ReceiverStream,};
use tonic::{Request, Response, Status, transport::Server};

pub mod orderbook_aggregator {
    tonic::include_proto!("orderbook");
}

mod aggregator;
mod binance;
mod bitstamp;
mod client;
mod error;
mod exchange;
mod order;
mod worker;

#[derive(Debug)]
struct OrderbookAggregatorService {
    client: Arc<Mutex<Client>>,
}

#[tonic::async_trait]
impl OrderbookAggregator for OrderbookAggregatorService {
    type BookSummaryStream = ReceiverStream<Result<Summary, Status>>;

    async fn book_summary(
        &self,
        request: Request<Symbol>,
    ) -> Result<Response<Self::BookSummaryStream>, Status> {
        let (sender, receiver) = mpsc::channel(1);

        let mut locked_client = self.client.lock().await;
        locked_client.sender = Some(sender);
        locked_client.symbol = request.get_ref().symbol.clone();

        let client = self.client.clone();
        tokio::spawn(async move {
            client_worker(client).await;
        });

        Ok(Response::new(ReceiverStream::new(receiver)))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:8888".parse().unwrap();
    let orderbook_aggregator = OrderbookAggregatorService {
        client: Arc::new(Mutex::new(Client {
            sender: None,
            symbol: String::from(""),
        })),
    };

    let orderbook_aggregator_service = OrderbookAggregatorServer::new(orderbook_aggregator);

    Server::builder()
        .add_service(orderbook_aggregator_service)
        .serve(addr)
        .await?;

    Ok(())
}