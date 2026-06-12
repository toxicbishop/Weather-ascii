use crate::error::{NetworkError, WeatherError};
use crate::weather::types::{AirQualityData, WeatherLocation};
use serde::Deserialize;
use std::time::Duration;

const OPEN_METEO_AQI_BASE_URL: &str = "https://air-quality-api.open-meteo.com/v1/air-quality";

pub struct OpenMeteoAqiProvider {
    client: reqwest::Client,
    base_url: String,
}

#[derive(Debug, Deserialize)]
struct OpenMeteoAqiResponse {
    current: CurrentAqi,
}

#[derive(Debug, Deserialize)]
struct CurrentAqi {
    european_aqi: f64,
    pm10: f64,
    pm2_5: f64,
    ozone: f64,
}

impl OpenMeteoAqiProvider {
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .connect_timeout(Duration::from_secs(10))
            .build()
            .unwrap_or_else(|e| {
                eprintln!("Warning: Failed to create custom HTTP client for AQI: {}", e);
                reqwest::Client::new()
            });

        Self {
            client,
            base_url: OPEN_METEO_AQI_BASE_URL.to_string(),
        }
    }

    fn build_url(&self, location: &WeatherLocation) -> String {
        format!(
            "{}?latitude={}&longitude={}&current=european_aqi,pm10,pm2_5,ozone",
            self.base_url,
            location.latitude,
            location.longitude
        )
    }

    pub async fn get_current_aqi(
        &self,
        location: &WeatherLocation,
    ) -> Result<AirQualityData, WeatherError> {
        let url = self.build_url(location);
        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| WeatherError::Network(NetworkError::from_reqwest(e, &url, 30)))?;

        let data: OpenMeteoAqiResponse = response
            .json()
            .await
            .map_err(|e| WeatherError::Network(NetworkError::from_reqwest(e, &url, 30)))?;

        Ok(AirQualityData {
            aqi: data.current.european_aqi,
            pm2_5: data.current.pm2_5,
            pm10: data.current.pm10,
            ozone: data.current.ozone,
        })
    }
}

impl Default for OpenMeteoAqiProvider {
    fn default() -> Self {
        Self::new()
    }
}
