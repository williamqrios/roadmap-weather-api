use std::{env, fmt, str::FromStr}; 
use dotenv::dotenv;
use serde::{Serialize, Deserialize, Deserializer, de}; 
use redis::Client; 


#[derive(Serialize, Deserialize, Debug)]
pub struct Weather {
    pub latitude: f64, 
    pub longitude: f64,
    #[serde(alias = "resolvedAddress")]
    pub resolved_address: String, 
    pub description: Option<String>,
    #[serde(alias = "currentConditions")]
    pub current_conditions: Option<DayStats>,
    pub days: Option<Vec<DayStats>>,
}

/// Fields of interest to deserialize from the Weather API response. Optional fields represent keys that are only present in the "days" object but not in the "currentConditions" object. 
#[derive(Serialize, Deserialize, Debug)] 
pub struct DayStats {
    pub datetime: String,
    pub temp: f64, 
    pub tempmin: Option<f64>, 
    pub tempmax: Option<f64>, 
    pub feelslike: f64,
    pub feelslikemin: Option<f64>, 
    pub feelslikemax: Option<f64>, 
    pub dew: f64, 
    pub humidity: f64, 
    pub precip: f64, 
    pub precipprob: f64, 
    pub windspeed: f64, 
    pub pressure: f64, 
    pub uvindex: f64, 
    pub conditions: String,
    pub description: Option<String>,
    pub icon: String,
}

#[derive(Clone)]
pub struct Config {
    pub api_key: String, 
    pub redis_host: String,
}

impl Config {
    pub fn new() -> Result<Self, env::VarError>  {
        dotenv().ok();
        let api_key = env::var("API_KEY")?;
        let host = env::var("REDIS_HOST")?;
        // placing host in the format expected by the redis-rs crate
        let redis_host = format!("redis://{host}/"); 
        let config = Config { api_key, redis_host };
        Ok(config)
    }
}

#[derive(Clone)]
pub struct AppState {
    pub config: Config, 
    pub redis_client: Client,
}

/// Struct representing query params (in the format start_date=value&end_date=value)
#[derive(Debug, Deserialize)] 
pub struct DateParams {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub start_date: Option<String>, 
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub end_date: Option<String>, 
}

/// Function to map an empty string as none (taken from Axum example)
fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: fmt::Display,
{
    let opt = Option::<String>::deserialize(de)?;
    match opt.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => FromStr::from_str(s).map_err(de::Error::custom).map(Some),
    }
}