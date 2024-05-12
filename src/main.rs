use crate::server::execute;

mod adapter;
mod config;
mod entities;
mod errors;
mod server;

#[tokio::main]
async fn main() {
    execute().await
}
