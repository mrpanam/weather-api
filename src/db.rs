

use surrealdb::{Surreal};
use surrealdb::Error;
use std::sync::OnceLock;
use surrealdb::engine::any::{connect, Any};

use crate::model::WeatherData;

pub static DB: OnceLock<Surreal<Any>> = OnceLock::new();

// Initialize once at startup
pub async fn init_db() -> Result<(), Error> {
    let db = connect("ws://localhost:8000").await?;

    db.use_ns("main").use_db("weather").await?;

    // Sign in with credentials
    db.signin(surrealdb::opt::auth::Root {
        username: "root".to_string(),
        password: "root".to_string(),
    })
    .await?;    

    DB.set(db).expect("Failed to set database");

    Ok(())
}

// Use anywhere in your app
pub fn get_db() -> &'static Surreal<Any> {
    DB.get().expect("Database not initialized")
}

pub async fn save_weather(data: WeatherData) -> Result<(), Error> {
    let db = get_db();
    let location = data.location_name.clone();
    tracing::info!(
        location = %location,
        "Saving weather data to SurrealDB"
    );
    let _: Option<WeatherData> = db.create("weather").content(data).await?;
    tracing::info!(
        location = %location,
        "Successfully saved weather data to SurrealDB"
    );
    Ok(())
}


