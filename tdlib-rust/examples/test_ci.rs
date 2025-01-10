// cargo run --example test_ci --features default
// cargo run --example test_ci --features download-tdlib
// cargo run --example test_ci --features pkg-config

#[tokio::main]
async fn main() {
    // Create the client object for testing
    let _client_id = tdlib_rust::create_client();

    // Exit 0
    std::process::exit(0);
}
