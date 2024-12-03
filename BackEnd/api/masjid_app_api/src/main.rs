mod init;
mod features;
mod shared;

use crate::init::run;

#[tokio::main]
async fn main() {
    run().await;
}
