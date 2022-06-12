### Orderbook Aggregator gRPC Server and Client

- Supports only one client currently
- Symbol may be configured using Symbol parameter to BookSummary
- Aggregates top quotes by sorting quote price and corresponding amounts -
highest amount comes first for the same price

### How to Run

Run the server using the following command:

```
cargo run --bin orderbook_aggregator_server
```

Run the client using the following command:

```
cargo run --bin orderbook_aggregator_client
```

Configure the symbol by changing the `Symbol` request in `grpc_client.rs`.

Example

```
    let request = tonic::Request::new(
        Symbol {
            symbol: String::from("btcusdt"),   <--- Change the symbol here
        },
    );
```
