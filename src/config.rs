use clap::Parser;
use std::net::IpAddr;

/// Cli-конфиг сервиса.
#[derive(Parser, Debug)]
pub struct Cli {
    // app host
    #[arg(long, default_value = "127.0.0.1")]
    pub host: IpAddr,
    // app port
    #[arg(long, default_value = "8080")]
    pub port: u16,
}
