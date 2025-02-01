use crate::models::{AppState, Weather}; 
use crate::client::weather_request;
use crate::utils::AppError; 
use redis::JsonCommands;
use axum::{Json, extract::State}; 


pub async fn hello(state: State<AppState>) -> Result<Json<Weather>, AppError> {
    let mut con = state.redis_client
        .get_connection()?;

    let location = "London,UK"; 
    // Attempt to get cached results, otherwise call the weather api 
    let api_result = if let Ok(json_string) = con.json_get::<&str, &str, String>(location, "$") {
        // Deserializing the outer JSON string to get an array of Strings
        let json_array: Vec<String> = serde_json::from_str(&json_string)?;
        // Extracting the first element from the array
        let inner_json_string = json_array.get(0)
            .ok_or("No data")?;
        // Deserializing the inner JSON string to get the Weather struct
        let result: Weather = serde_json::from_str(inner_json_string)?;
        result
    } else {
        // Calling weather api. Many errors can occur here. 
        let result = weather_request(location, &state.config.api_key).await?; 
        // &json!(result).to_string()
        // Storing the key and Json information in case the request is successful 
        let _: () = con
            .json_set(location, "$", &result)?;
        serde_json::from_str(&result)?
    };
    Ok(Json(api_result))
}