use clap::Parser;
use log::info;
use tokio::net::TcpListener;

mod utils;

#[derive(Parser)]
#[command(version)]
struct Args {
    #[arg(short, long)] // 支持 -c 和 --config
    pub config: Option<String>
}

#[tokio::main]
async fn main(){
    let args = Args::parse();
    let cfg :server::config::Config = utils::load("server",args.config);
    let listener = TcpListener::bind(&cfg.http.listen)
        .await
        .unwrap();
    let addr = listener.local_addr().unwrap();
    info!("Listening on {}", addr);
    println!("Listening on {}", addr);
    server::serve(cfg,listener).await;
    info!("Server Shutdown");
}