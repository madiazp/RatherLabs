use reqwest;
use serde::Deserialize;

use crate::config;
// Response structs, only used attributes are public.
#[derive(Deserialize, Debug)]
pub struct ApiResponse {
    _id: String,
    dist: f64,
    lid:  i32,
    fid:  i32,
    pub name: String,
    pub province: String,
    lat: String,
    lon: String,
    zoom: String,
    updated: i32,
    pub weather: Weather,
}
#[derive(Deserialize, Debug)]
pub struct Weather {
    day: i32,
    pub morning_temp: i32,
    morning_id: i32,
    pub morning_desc: String,
    pub afternoon_temp: i32,
    afternoon_id: i32,
    pub afternoon_desc: String,
}

// api call function, it parse the response to an ApiResponse struct
pub async fn api_call(forecast: String) -> Result<Vec<ApiResponse>, reqwest::Error> {
    let wrapped_response = reqwest::get(format!("{}{}", config::API_URL, forecast))
        .await;
    let wrapped_parsed_response = match wrapped_response {
        Ok(response) => response
            .json::<Vec::<ApiResponse>>()
            .await,
        Err(e) => { return Err(e); },
    };
    let response = match wrapped_parsed_response {
        Ok(r) => r,
        Err(e) => { return Err(e); }
    };
    Ok(response)
}
