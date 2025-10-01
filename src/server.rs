use clap::Parser;
use log::info;
use tokio::net::TcpListener;

use utils::load;
pub mod utils;

#[derive(Parser)]
#[command(version)]
struct Args {
    #[arg(short, long)] // 支持 -c 和 --config
    pub config: Option<String>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let cfg: server::config::Config;
    #[cfg(debug_assertions)]
    if cfg!(debug_assertions) {
        cfg = load("server", Some("conf/server.local.toml".to_string()));
    } else {
        cfg = load("server", args.config);
    }
    let listener = TcpListener::bind(&cfg.http.listen).await.unwrap();
    let addr = listener.local_addr().unwrap();
    info!("Listening on {}", addr);

    #[cfg(debug_assertions)]
    println!("listening on {}", addr);
    server::serve(cfg, listener).await;
    info!("Server Shutdown");
}
