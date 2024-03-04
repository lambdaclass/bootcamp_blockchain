mod verifier;

use clap::Parser;
use tendermint_abci::ServerBuilder;
use tracing_subscriber::filter::LevelFilter;
use verifier::Verifier;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = false)]
    liar: bool,
}

fn main() {
    let args = Args::parse();

    let read_buf_size = 1048576;
    let host = "127.0.0.1";
    let port = 26658;

    let addr = format!("{}:{}", host, port);

    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .init();

    let app = Verifier::new(args.liar);
    let server = ServerBuilder::new(read_buf_size).bind(addr, app).unwrap();

    server.listen().unwrap();
}
