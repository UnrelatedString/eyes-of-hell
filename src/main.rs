mod meat;
use meat::run;

// what if. Mystic Eyes of Depth Perception

#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() {
    run().await;
}
