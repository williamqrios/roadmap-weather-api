use std::{error::Error, net::SocketAddr, sync::Arc};
use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer};
use crate::routes::start; 

mod models; 
mod routes; 
mod client; 
mod handlers; 
mod utils; 
use models::{AppState, Config};

/// Starts the API service 
pub async fn run() -> Result<(), Box<dyn Error>> {
    // The config struct stores env variables (api key, redis connection string)
    let config = Config::new()?; 
    // Connect to redis service
    let client = redis::Client::open(config.redis_host.clone())?;
    // The app state struct stores config and the redis client, so that it can be reused. 
    let app_state = AppState { redis_client: client, config: config.clone() }; 
    // Rate limiting config. Taken straight from the example: "allow bursts with up to five requests per IP address and replenish one element every two seconds"
    let governor_conf = Arc::new(
        GovernorConfigBuilder::default()
            .per_second(2)
            .burst_size(5)
            .finish()
            .unwrap(),
    );

    // Start the API
    let app = start(app_state).layer( GovernorLayer { config:  governor_conf } ); 
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("Listening for requests on {}", listener.local_addr().unwrap());
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await?;

    Ok(())
}

