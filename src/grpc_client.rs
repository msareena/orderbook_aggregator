use orderbook_aggregator::{orderbook_aggregator_client::OrderbookAggregatorClient, Symbol, Summary};


pub mod orderbook_aggregator {
    tonic::include_proto!("orderbook");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let channel = tonic::transport::Channel::from_static("http://127.0.0.1:8888")
        .connect()
        .await?;

    let mut client = OrderbookAggregatorClient::new(channel);

    let request = tonic::Request::new(
        Symbol {
            symbol: String::from("btcusdt"),
        },
    );

    let mut stream = client.book_summary(request).await?.into_inner();

    while let Some(summary) = stream.message().await? {
        print_summary(&summary);
    }

    Ok(())
}

fn print_summary(summary: &Summary) {
    let bids = &summary.bids;
    let asks = &summary.asks;
    let spread = summary.spread;
    println!("{:=^1$}", "Aggregate Summary", 111);
    println!("{:>15}: {:>15}", "Spread", spread);
    println!("{:>15} {:>15} {:>20}       {:>15} {:>15} {:>20}",
             "Bid Price", "Bid Amount", "Bid Exchange",
             "Ask Price", "Ask Amount", "Ask Exchange");
    for (bid, ask) in bids.iter().zip(asks.iter()) {
        println!("{:>15} {:>15} {:>20}       {:>15} {:>15} {:>20}",
                 bid.price, bid.amount, bid.exchange,
                 ask.price, ask.amount, ask.exchange);
    }
    println!();
}