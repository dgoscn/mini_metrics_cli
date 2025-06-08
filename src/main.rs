use axum::{routing::get, Router, response::IntoResponse};
use clap::Parser;
use std::{net::SocketAddr};
use sysinfo::{System, RefreshKind, CpuRefreshKind, MemoryRefreshKind};
use tokio::net::TcpListener;

#[derive(Parser, Debug)]
struct Args {
    /// Port to bind the metrics server
    #[arg(short, long, default_value_t = 8080)]
    port: u16,
}

async fn metrics_handler() -> impl IntoResponse {
    let refresh = RefreshKind::new()
        .with_cpu(CpuRefreshKind::everything())
        .with_memory(MemoryRefreshKind::everything());

    let sys = System::new_with_specifics(refresh);

    let uptime = System::uptime(); 
    let total_memory = sys.total_memory();
    let used_memory = sys.used_memory();
    let cpu_usage = sys.global_cpu_info().cpu_usage();

    format!(
        "\
uptime_seconds {}\n\
memory_total_bytes {}\n\
memory_used_bytes {}\n\
cpu_usage_percent {}\n",
        uptime, total_memory, used_memory, cpu_usage
    )
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let app = Router::new().route("/metrics", get(metrics_handler));

    let addr = SocketAddr::from(([0, 0, 0, 0], args.port));
    println!("Server running at http://{}/metrics", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
