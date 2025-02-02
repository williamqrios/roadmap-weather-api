use crate::models::{AppState, Weather, DateParams}; 
use crate::client::weather_request;
use crate::utils::AppError; 
use redis::JsonCommands;
use axum::{Json, extract::{State, Path, Query}}; 

pub async fn weather(Path(loc): Path<String>, Query(params): Query<DateParams>, state: State<AppState>) -> Result<Json<Weather>, AppError> {
    let mut con = state.redis_client
        .get_connection()?;
    // Create a key for caching the result 
    let redis_key = match params {
        DateParams { start_date: None, end_date: None } => format!("{loc}"), 
        DateParams { start_date: Some(ref date1), end_date: None } => format!("{loc}{date1}"), 
        DateParams { start_date: None, end_date: Some(ref date2) } => format!("{loc}{date2}"),
        DateParams { start_date: Some(ref date1), end_date: Some(ref date2) } => format!("{loc}{date1}{date2}"),
    };

    // Attempt to get cached results, otherwise call the weather api 
    let api_result = if let Ok(json_string) = con.json_get::<&str, &str, String>(&redis_key, "$") {
        // Deserializing the outer JSON string to get an array of Strings 
        let json_array = serde_json::from_str::<Vec<String>>(&json_string)?;
        // Extracting the first element from the array
        let inner_json_string = json_array.get(0)
            .ok_or("No data")?;
        // Deserializing the inner JSON string to get the Weather struct
        let result = serde_json::from_str::<Weather>(inner_json_string)?;
        result
    } else {
        // Calling weather api. Many errors can occur here. 
        let result = weather_request(&loc, params.start_date.as_ref().map(String::as_str), params.end_date.as_ref().map(String::as_str), &state.config.api_key).await?; 
        // Storing the key and Json information in case the request is successful 
        let _: () = con
            .json_set(&redis_key, "$", &result)?;
        serde_json::from_str(&result)?
    };
    Ok(Json(api_result))
}
