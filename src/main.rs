pub mod backend;
pub mod common;
pub mod frontend;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    Ok(())
}
