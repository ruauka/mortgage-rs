use crate::server::execute;

mod adapter;
mod config;
mod domain;
mod errors;
mod schema;
mod server;

#[tokio::main]
async fn main() {
    execute().await
}
