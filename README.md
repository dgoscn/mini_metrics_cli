# mini\_metrics\_cli

`mini_metrics_cli` is a lightweight Rust-based HTTP server that exposes real-time system metrics — like uptime, memory usage, and CPU utilization — in a Prometheus-compatible format.

This project was designed as an experiment in learning how to combine system introspection with a minimal async web API. It’s ideal as a learning tool or a small utility in dev environments.

---

## What It Does

Once running, it exposes the following system metrics via:

```
GET /metrics
```

Sample output:

```
uptime_seconds 6542
memory_total_bytes 17179869184
memory_used_bytes 8234375168
cpu_usage_percent 4.25
```

---

## Tech Stack

* [Rust](https://www.rust-lang.org/) 1.77+
* [Axum](https://github.com/tokio-rs/axum) — minimal web server
* [Sysinfo](https://github.com/GuillaumeGomez/sysinfo) — system information
* [Clap](https://github.com/clap-rs/clap) — CLI argument parser
* [Tokio](https://tokio.rs/) — async runtime

---

## Getting Started

### Prerequisites

* Rust (install via [rustup](https://rustup.rs))
* Cargo (comes with Rust)

### Build the Project

```bash
cargo build
```

Or to clean previous builds:

```bash
cargo clean
```

### Run the Server

```bash
cargo run -- --port 8080
```

Then access:

```
http://localhost:8080/metrics
```

---

## Development Tips

### Project structure

```
src/
├── main.rs      # main application logic
Cargo.toml       # dependencies and metadata
```

### Generate Documentation

```bash
cargo doc --open
```

---

## Optional: Run with Docker

You can run this as a container:

```Dockerfile
# Dockerfile
FROM rust:1.77 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:buster-slim
COPY --from=builder /app/target/release/mini_metrics_cli /usr/local/bin/mini_metrics_cli
ENTRYPOINT ["/usr/local/bin/mini_metrics_cli"]
```

```bash
docker build -t mini-metrics .
docker run -p 8080:8080 mini-metrics --port 8080
```

---

## Future Improvements

* Add `/healthz` and `/metrics.json` endpoints
* Support more granular memory and CPU metrics
* Container metrics (PID, cgroup, disk IO)
* Kubernetes-ready sidecar mode

---