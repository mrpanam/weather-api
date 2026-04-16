
use crate::model::{WeatherData, OpenMeteoResponse, Location, WeatherCode};
use surrealdb_types::Datetime;
use tokio::time::{sleep, Duration};


pub async fn get_current_weather(location: Location) -> Result<WeatherData, Box<dyn std::error::Error + Send + Sync>> {
    let url = format!(
        "https://api.open-meteo.com/v1/forecast\
        ?latitude={}\
        &longitude={}\
        &current=temperature_2m,wind_speed_10m,precipitation,surface_pressure,weather_code",
        location.lat, location.lon
    );

    let log_error = |msg: &str, error: &dyn std::error::Error| {
        tracing::error!(
            location = %location.name,
            latitude = location.lat,
            longitude = location.lon,
            error = %error,
            msg
        );
    };

    let response = reqwest::get(&url).await.map_err(|e| {
        log_error("Failed to fetch from OpenMeteo API", &e);
        e
    })?;

    let text = response.text().await.map_err(|e| {
        log_error("Failed to read response body from OpenMeteo API", &e);
        e
    })?;

let data: OpenMeteoResponse = match serde_json::from_str(&text) {
    Ok(d) => d,
    Err(e) => {
        tracing::error!(
            location = %location.name,
            latitude = location.lat,
            longitude = location.lon,
            api_response = %text,
            error = %e,
            "Failed to parse JSON from OpenMeteo API"
        );
        return Err(e.into());
    }
};

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
) -> Vec<Result<WeatherData, Box<dyn std::error::Error + Send + Sync>>> {
    let mut results = Vec::new();
    for loc in locations {
        let result = get_current_weather(loc).await;
        results.push(result);
        // Add delay to avoid rate limiting (200ms between requests)
        sleep(Duration::from_millis(200)).await;
    }
    results
}
