mod cli;
mod server;
mod system_info;

use cli::Args;
use clap::Parser;

#[tokio::main]
async fn main() {
    let args = Args::parse();

    server::start_server(args.port).await;
}