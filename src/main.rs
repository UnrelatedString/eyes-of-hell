mod meat;
use meat::run;

// what if. Mystic Eyes of Depth Perception

#[cfg(not(target_family = "wasm"))]
#[tokio::main]
async fn main() {
    run().await;
}
