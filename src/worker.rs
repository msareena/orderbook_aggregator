use std::f64::NAN;
use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{
    Client,
    binance::Binance,
    bitstamp::Bitstamp,
    exchange::Exchange,
    orderbook_aggregator::Summary
};

use crate::aggregator::{Aggregator, QuoteType};

pub async fn client_worker(client: Arc<Mutex<Client>>) {
    let connected_client = client.clone();

    let mut binance_exchange = Binance::new(
        connected_client.lock().await.symbol.clone()
    );
    let mut bitstamp_exchange = Bitstamp::new(
        connected_client.lock().await.symbol.clone()
    );

    let num_top_orders = 10;

    loop {
        let bitstamp_summary = match bitstamp_exchange.stream() {
            Ok(summary) => summary,
            Err(e) => {
                println!("Bitstamp Error: {}", e);
                break;
            }
        };

        let binance_summary = match binance_exchange.stream() {
            Ok(summary) => summary,
            Err(e) => {
                println!("Binance Error: {}", e);
                break;
            }
        };

        let aggregate_asks = Aggregator::aggregate_top(
            num_top_orders,
            bitstamp_summary.clone(),
            binance_summary.clone(),
            QuoteType::ASKS,
        );
        let aggregate_bids = Aggregator::aggregate_top(
            num_top_orders,
            bitstamp_summary,
            binance_summary,
            QuoteType::BIDS,
        );

        // If no summary data from both connected exchanges, we have nothing
        // to send ot the client.
        if aggregate_asks.is_empty() && aggregate_bids.is_empty() {
            continue;
        }

        let mut spread = NAN;
        if aggregate_asks.len() > 0 && aggregate_bids.len() > 0 {
            spread = aggregate_bids[0].price - aggregate_asks[0].price;
        }

        let aggregate_summary = Summary {
            bids: aggregate_bids,
            asks: aggregate_asks,
            spread,
        };

        match connected_client.lock().await.sender.as_ref() {
            Some(sender) => {
                match sender.send(Ok(aggregate_summary)).await {
                    Ok(_) => (),
                    Err(e) => {
                        println!("Failed to send data to client: {}", e);
                        break;
                    }
                }
            },
            _ => {
                println!("Client connection does not exists");
                break;
            }
        };
    }
}

