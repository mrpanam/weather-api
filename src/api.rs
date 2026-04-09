
use crate::model::{WeatherData, OpenMeteoResponse, Location, WeatherCode};
use surrealdb_types::Datetime;


pub async fn get_current_weather(location: Location) -> Result<WeatherData, reqwest::Error> {
    


    let url = format!(
        "https://api.open-meteo.com/v1/forecast\
        ?latitude={}\
        &longitude={}\
        &current=temperature_2m,wind_speed_10m,precipitation,surface_pressure,weather_code",
        location.lat, location.lon
    );

    let response = reqwest::get(&url).await?;
    let data: OpenMeteoResponse = response.json().await?;

    Ok(WeatherData {
        location_name: location.name,
        time: data.current.time.parse().unwrap_or_else(|_| Datetime::default()),
        temperature_2m: data.current.temperature_2m,
        wind_speed_10m: data.current.wind_speed_10m,
        precipitation: data.current.precipitation,
        surface_pressure: data.current.surface_pressure,
        weather_code: WeatherCode::from(data.current.weather_code as i32).label().to_string(),
    })
}

pub async fn get_weather_for_locations(
    locations: Vec<Location>,
) -> Vec<Result<WeatherData, reqwest::Error>> {
    let handles: Vec<tokio::task::JoinHandle<Result<WeatherData, reqwest::Error>>> = locations
        .into_iter()
        .map(|loc| tokio::spawn(get_current_weather(loc)))
        .collect();

    let mut results = Vec::new();
    for handle in handles {
        match handle.await {
            Ok(res) => results.push(res),
            Err(_) => {
                let err = reqwest::get("invalid://url").await.unwrap_err();
                results.push(Err(err));
            }
        }
    }
    results
}
