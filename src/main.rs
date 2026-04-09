
mod db;
mod model;
mod api;
mod location;
mod postgres;



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, Open Meteo API!");
    db::init_db().await?;

    let locations = location::get_locations();
    
    let results = api::get_weather_for_locations(locations).await;
    postgres::init_postgres("postgres://postgres:root@localhost/Xone").await?;

    for result in results {
        match result {
            Ok(weather) => {
                postgres::save_weather_pg(&weather).await?;
                db::save_weather(weather).await?;
            }
            Err(e) => eprintln!("Failed to fetch weather: {}", e),
        }
    }

    Ok(())
}
