use crate::utils::AppError;
const API_URL: &'static str = "https://weather.visualcrossing.com/VisualCrossingWebServices/rest/services/timeline";  

/// Makes the request to the visual crossing weather API. 
pub async fn weather_request(location: &str, api_key: &str) -> Result<String, AppError> {
    let endpoint = format!("{API_URL}/{location}?key={api_key}&unitGroup=metric");
    let client = reqwest::Client::new(); 
    let resp = client.get(endpoint)
        .send()
        .await?; 

    match resp.status().as_u16() {
        200 => {},
        400 => return Err("Invalid parameter values (bad request).".into()),
        401 => return Err("API key invalid or inaccessible feature (unauthorized).".into()),
        404 => return Err("Not found.".into()),
        429 => return Err("Maximum number of requests have been exceeded (too many requests).".into()),
        500 => return Err("Error processing the request (internal server error).".into()),
        _ => {
            let error_message = format!("Weather API request error. Status code: {}.", resp.status()); 
            return Err(error_message.as_str().into());
        }
    }
    let body = resp.text().await?;
    Ok(body)
}
