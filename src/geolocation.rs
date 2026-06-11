use crate::cache;
use crate::error::{GeolocationError, NetworkError};
use serde::{Deserialize, Serialize};
use std::time::Duration;

const IPINFO_URL: &str = "https://ipinfo.io/json";
const GEOCODING_API_URL: &str = "https://geocoding-api.open-meteo.com/v1/search";
const MAX_RETRIES: u32 = 3;
const INITIAL_RETRY_DELAY_MS: u64 = 500;

#[derive(Deserialize, Debug)]
struct IpInfoResponse {
    loc: String,
    city: Option<String>,
}

#[derive(Deserialize, Debug)]
struct GeocodingResponse {
    results: Option<Vec<GeocodingResult>>,
}

#[derive(Deserialize, Debug)]
struct GeocodingResult {
    name: String,
    latitude: f64,
    longitude: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoLocation {
    pub latitude: f64,
    pub longitude: f64,
    pub city: Option<String>,
}

pub async fn detect_location() -> Result<GeoLocation, GeolocationError> {
    if let Some(cached) = cache::load_cached_location().await {
        return Ok(cached);
    }

    detect_location_with_retry().await
}

async fn detect_location_with_retry() -> Result<GeoLocation, GeolocationError> {
    let mut last_error = None;

    for attempt in 1..=MAX_RETRIES {
        match fetch_location().await {
            Ok(location) => return Ok(location),
            Err(e) => {
                let should_retry = matches!(
                    e,
                    GeolocationError::Unreachable(ref net_err) if net_err.is_retryable()
                );

                if !should_retry || attempt == MAX_RETRIES {
                    return Err(e);
                }

                let delay_ms = INITIAL_RETRY_DELAY_MS * 2_u64.pow(attempt - 1);
                tokio::time::sleep(Duration::from_millis(delay_ms)).await;
                last_error = Some(e);
            }
        }
    }

    Err(
        last_error.unwrap_or_else(|| GeolocationError::RetriesExhausted {
            attempts: MAX_RETRIES,
        }),
    )
}

async fn fetch_location() -> Result<GeoLocation, GeolocationError> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .connect_timeout(Duration::from_secs(5))
        .build()
        .map_err(|e| GeolocationError::Unreachable(NetworkError::ClientCreation(e)))?;

    let response = client.get(IPINFO_URL).send().await.map_err(|e| {
        GeolocationError::Unreachable(NetworkError::from_reqwest(e, IPINFO_URL, 10))
    })?;

    let ip_info: IpInfoResponse = response.json().await.map_err(|e| {
        GeolocationError::Unreachable(NetworkError::from_reqwest(e, IPINFO_URL, 10))
    })?;

    let coords: Vec<&str> = ip_info.loc.split(',').collect();
    if coords.len() != 2 {
        return Err(GeolocationError::ParseError(
            "Invalid location format from ipinfo.io".to_string(),
        ));
    }

    let latitude = coords[0]
        .parse::<f64>()
        .map_err(|_| GeolocationError::ParseError("Invalid latitude format".to_string()))?;

    let longitude = coords[1]
        .parse::<f64>()
        .map_err(|_| GeolocationError::ParseError("Invalid longitude format".to_string()))?;

    let location = GeoLocation {
        latitude,
        longitude,
        city: ip_info.city,
    };

    cache::save_location_cache(&location);

    Ok(location)
}

pub async fn geocode_city(city: &str) -> Result<GeoLocation, GeolocationError> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .connect_timeout(Duration::from_secs(5))
        .build()
        .map_err(|e| GeolocationError::Unreachable(NetworkError::ClientCreation(e)))?;

    let url = format!("{}?name={}&count=1&format=json", GEOCODING_API_URL, city);
    let response = client.get(&url).send().await.map_err(|e| {
        GeolocationError::Unreachable(NetworkError::from_reqwest(e, &url, 10))
    })?;

    let geocoding_response: GeocodingResponse = response.json().await.map_err(|e| {
        GeolocationError::Unreachable(NetworkError::from_reqwest(e, &url, 10))
    })?;

    if let Some(results) = geocoding_response.results {
        if let Some(first_result) = results.into_iter().next() {
            return Ok(GeoLocation {
                latitude: first_result.latitude,
                longitude: first_result.longitude,
                city: Some(first_result.name),
            });
        }
    }

    Err(GeolocationError::ParseError(format!("City not found: {}", city)))
}
