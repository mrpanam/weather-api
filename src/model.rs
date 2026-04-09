use serde::{Deserialize, Serialize};
use surrealdb_types::{Datetime, SurrealValue};


#[derive(Debug, Clone, Serialize, Deserialize, SurrealValue, PartialEq)]
pub struct Location{
    pub name: String,   
    pub lat: f64,
    pub lon: f64,
}


#[derive(Debug, Clone, Serialize, Deserialize, SurrealValue)]
pub struct WeatherData {
    pub location_name: String,
    pub time: Datetime,
    pub temperature_2m: f64,
    pub wind_speed_10m: f64,
    pub precipitation: f64,
    pub weather_code: String,
    pub surface_pressure: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize, SurrealValue)]
pub struct OpenMeteoResponse {
    pub current: CurrentWeather,
}

#[derive(Debug, Clone, Serialize, Deserialize, SurrealValue)]
pub struct CurrentWeather {
    pub time: String,
    pub temperature_2m: f64,
    pub wind_speed_10m: f64,
    pub precipitation: f64,
    pub surface_pressure: f64,
    pub weather_code: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, SurrealValue, PartialEq)]
#[serde(from = "i64", into = "i64")]
pub enum WeatherCode {
    ClearSky,
    PartlyCloudy,
    Overcast,
    Fog,
    Drizzle,
    Rain,
    Snow,
    Thunderstorm,
    Unknown(i32),
}

impl From<i64> for WeatherCode {
    fn from(code: i64) -> Self {
        let code = code as i32;
        Self::from(code)
    }
}

impl From<i32> for WeatherCode {
    fn from(code: i32) -> Self {
        match code {
            0 => Self::ClearSky,
            1 | 2 => Self::PartlyCloudy,
            3 => Self::Overcast,
            45 | 48 => Self::Fog,
            51..=55 => Self::Drizzle,
            61..=65 => Self::Rain,
            71..=75 => Self::Snow,
            80..=82 => Self::Rain,
            95..=99 => Self::Thunderstorm,
            _ => Self::Unknown(code),
        }
    }
}

impl From<WeatherCode> for i64 {
    fn from(code: WeatherCode) -> Self {
        code.code() as i64
    }
}

impl WeatherCode {
    const fn code(&self) -> i32 {
        match self {
            Self::ClearSky => 0,
            Self::PartlyCloudy => 1,
            Self::Overcast => 3,
            Self::Fog => 45,
            Self::Drizzle => 51,
            Self::Rain => 61,
            Self::Snow => 71,
            Self::Thunderstorm => 95,
            Self::Unknown(c) => *c,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::ClearSky => "Clear sky ☀️",
            Self::PartlyCloudy => "Partly cloudy ⛅",
            Self::Overcast => "Overcast ☁️",
            Self::Fog => "Fog 🌫️",
            Self::Drizzle => "Drizzle 🌦️",
            Self::Rain => "Rain 🌧️",
            Self::Snow => "Snow ❄️",
            Self::Thunderstorm => "Thunderstorm ⛈️",
            Self::Unknown(_) => "Unknown",
        }
    }
}
