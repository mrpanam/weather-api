use sqlx::{PgPool, Error};
use std::sync::OnceLock;
use chrono::{DateTime, Utc};

use crate::model::WeatherData;

pub static PG_POOL: OnceLock<PgPool> = OnceLock::new();

pub async fn init_postgres(database_url: &str) -> Result<(), Error> {
    let pool = PgPool::connect(database_url).await?;
    PG_POOL.set(pool).expect("Failed to set PostgreSQL pool");
    Ok(())
}

pub fn get_pg_pool() -> &'static PgPool {
    PG_POOL.get().expect("PostgreSQL pool not initialized")
}

pub async fn save_weather_pg(data: &WeatherData) -> Result<(), Error> {
    let pool = get_pg_pool();
    
    sqlx::query(
        r#"
        INSERT INTO weather_data 
            (location_name, time, temperature_2m, wind_speed_10m, precipitation, weather_code, surface_pressure)
        VALUES 
            ($1, $2, $3, $4, $5, $6, $7)
        "#
    )
    .bind(&data.location_name)
    .bind(data.time.to_string().parse::<DateTime<Utc>>().unwrap_or_else(|_| Utc::now()))
    .bind(data.temperature_2m)
    .bind(data.wind_speed_10m)
    .bind(data.precipitation)
    .bind(&data.weather_code)
    .bind(data.surface_pressure)
    .execute(pool)
    .await?;
    
    Ok(())
}
