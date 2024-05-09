#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_assignments)]
#![allow(unused_mut)]

use crate::server::execute;

mod adapter;
mod entities;
mod errors;
mod server;

#[tokio::main]
async fn main() {
    execute().await
}
