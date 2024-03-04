mod echo;

use echo::EchoApp;
use tendermint_abci::ServerBuilder;
use tracing_subscriber::filter::LevelFilter;

fn main() {
    let read_buf_size = 1048576;
    let host = "localhost";
    let port = 26658;

    let addr = format!("{}:{}", host, port);

    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .init();

    let app = EchoApp::new();
    let server = ServerBuilder::new(read_buf_size).bind(addr, app).unwrap();

    server.listen().unwrap();
}
