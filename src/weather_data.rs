use reqwest::{Error, Response};
use crate::weather_info::WeatherResponse; 
//get weather information

pub fn get_weather_data(
    city: &str,
    country_code: &str,
    api_key: &str,
) -> Result<WeatherResponse, reqwest::Error> {
    let url: String = format!("http://api.openweathermap.org/data/2.5/weather?q={},{}&units=metric&appid={}",city, country_code, api_key);

    // Send GET request
    let response = reqwest::blocking::get(&url)?;

    
    // Parse JSON to WeatherResponse struct
    let response_json: WeatherResponse = response.json::<WeatherResponse>()?;
    println!("{:?}", response_json);

    Ok(response_json)
}
