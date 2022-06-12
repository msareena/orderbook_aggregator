fn main() {
    tonic_build::compile_protos("proto/orderbook_aggregator.proto")
        .unwrap_or_else(
            |err| panic!("Failed to compile proto file. Err: {:?}", err)
        );
}