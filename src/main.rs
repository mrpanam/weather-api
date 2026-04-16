
mod db;
mod model;
mod api;
mod location;
mod postgres;



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .json()
        .with_target(false)
        .init();

    tracing::info!("Starting weather API");
    db::init_db().await?;

    let locations = location::get_locations();
    
    let results = api::get_weather_for_locations(locations).await;
    postgres::init_postgres("postgres://postgres:root@localhost/Xone").await?;

    for result in results {
        match result {
            Ok(weather) => {
                tracing::info!(
                    location = %weather.location_name,
                    temperature = weather.temperature_2m,
                    "Successfully fetched weather"
                );
                postgres::save_weather_pg(&weather).await?;
                db::save_weather(weather).await?;
            }
            Err(_) => {} // Error already logged in api.rs
        }
    }

    Ok(())
}
