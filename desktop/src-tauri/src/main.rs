#[tokio::main]
async fn main() {
    // Replace 'desktop' with your actual crate name from Cargo.toml
    desktop_lib::run().await;
}
