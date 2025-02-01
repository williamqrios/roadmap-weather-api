use std::process; 
use weather_api::run; 

#[tokio::main]
async fn main() {
    if let Err(error) = run().await {
        println!("Application error: {error}"); 
        process::exit(1); 
    }
}
