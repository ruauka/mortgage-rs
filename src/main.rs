use crate::server::execute;

mod adapter;
mod config;
mod errors;
mod logic;
mod schema;
mod server;

#[tokio::main]
async fn main() {
    execute().await
}
