use crate::animation_manager::AnimationManager;
use crate::app_state::AppState;
use crate::config::Config;
use crate::error::WeatherError;
use crate::render::TerminalRenderer;
use crate::scene::WorldScene;
use crate::weather::{
    OpenMeteoProvider, WeatherClient, WeatherCondition, WeatherData, WeatherLocation,
};
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use std::io;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;

type UpdateTuple = (
    Result<WeatherData, WeatherError>,
    Option<Result<crate::weather::AirQualityData, WeatherError>>,
);

const REFRESH_INTERVAL: Duration = Duration::from_secs(300);
const INPUT_POLL_FPS: u64 = 60;

fn generate_offline_weather(rng: &mut impl rand::Rng) -> WeatherData {
    use chrono::{Local, Timelike};
    use rand::RngExt;

    let now = Local::now();
    let hour = now.hour();
    let is_day = (6..18).contains(&hour);

    let conditions = [
        WeatherCondition::Clear,
        WeatherCondition::PartlyCloudy,
        WeatherCondition::Cloudy,
        WeatherCondition::Rain,
    ];

    let condition = conditions[rng.random_range(0..conditions.len())];

    WeatherData {
        condition,
        temperature: rng.random_range(10.0..25.0),
        apparent_temperature: rng.random_range(10.0..25.0),
        humidity: rng.random_range(40.0..80.0),
        precipitation: if condition.is_raining() {
            rng.random_range(1.0..5.0)
        } else {
            0.0
        },
        wind_speed: rng.random_range(5.0..15.0),
        wind_direction: rng.random_range(0.0..360.0),
        cloud_cover: rng.random_range(20.0..80.0),
        pressure: rng.random_range(1000.0..1020.0),
        visibility: Some(10000.0),
        is_day,
        moon_phase: Some(0.5),
        timestamp: now.format("%Y-%m-%dT%H:%M:%S").to_string(),
        hourly_forecast: None,
    }
}

pub struct App {
    state: AppState,
    animations: AnimationManager,
    scene: WorldScene,
    weather_receiver: mpsc::Receiver<UpdateTuple>,
    hide_hud: bool,
    show_aqi: bool,
}

impl App {
    pub fn new(
        config: &Config,
        simulate_condition: Option<String>,
        simulate_night: bool,
        show_leaves: bool,
        term_width: u16,
        term_height: u16,
    ) -> Self {
        let location = WeatherLocation {
            latitude: config.location.latitude,
            longitude: config.location.longitude,
            elevation: None,
            name: config.location.name.clone(),
        };

        let mut state = AppState::new(location.clone(), config.location.hide, config.units);
        let mut animations = AnimationManager::new(term_width, term_height, show_leaves);
        let scene = WorldScene::new(term_width, term_height);

        let (tx, rx) = mpsc::channel(1);
        let show_aqi = config.show_aqi;

        if let Some(ref condition_str) = simulate_condition {
            let simulated_condition =
                condition_str
                    .parse::<WeatherCondition>()
                    .unwrap_or_else(|e| {
                        eprintln!("{}", e);
                        WeatherCondition::Clear
                    });

            let weather = WeatherData {
                condition: simulated_condition,
                temperature: 20.0,
                apparent_temperature: 19.0,
                humidity: 65.0,
                precipitation: if simulated_condition.is_raining() {
                    2.5
                } else {
                    0.0
                },
                wind_speed: if simulated_condition.is_thunderstorm() {
                    45.0
                } else {
                    10.0
                },
                wind_direction: 225.0,
                cloud_cover: 50.0,
                pressure: 1013.0,
                visibility: Some(10000.0),
                is_day: !simulate_night,
                moon_phase: Some(0.5),
                timestamp: "simulated".to_string(),
                hourly_forecast: None,
            };

            let rain_intensity = weather.condition.rain_intensity();
            let snow_intensity = weather.condition.snow_intensity();

            let wind_speed = weather.wind_speed;
            let wind_direction = weather.wind_direction;

            state.update_weather(weather);
            animations.update_rain_intensity(rain_intensity);
            animations.update_snow_intensity(snow_intensity);
            animations.update_wind(wind_speed as f32, wind_direction as f32);
        } else {
            let provider = Arc::new(OpenMeteoProvider::new());
            let weather_client = WeatherClient::new(provider, REFRESH_INTERVAL);
            let aqi_provider = Arc::new(crate::weather::OpenMeteoAqiProvider::new());
            let units = config.units;

            tokio::spawn(async move {
                loop {
                    let weather_future = weather_client.get_current_weather(&location, &units);
                    
                    if show_aqi {
                        let aqi_future = aqi_provider.get_current_aqi(&location);
                        let (weather_res, aqi_res) = tokio::join!(weather_future, aqi_future);
                        if tx.send((weather_res, Some(aqi_res))).await.is_err() {
                            break;
                        }
                    } else {
                        let weather_res = weather_future.await;
                        if tx.send((weather_res, None)).await.is_err() {
                            break;
                        }
                    }

                    tokio::time::sleep(REFRESH_INTERVAL).await;
                }
            });
        }

        Self {
            state,
            animations,
            scene,
            weather_receiver: rx,
            hide_hud: config.hide_hud,
            show_aqi: config.show_aqi,
        }
    }

    pub async fn run(&mut self, renderer: &mut TerminalRenderer) -> io::Result<()> {
        let mut rng = rand::rng();
        loop {
            if let Ok((weather_result, aqi_result_opt)) = self.weather_receiver.try_recv() {
                if let Some(aqi_result) = aqi_result_opt {
                    match aqi_result {
                        Ok(aqi_data) => self.state.update_aqi(aqi_data),
                        Err(_e) => {} // Fail silently if AQI is down
                    }
                }

                match weather_result {
                    Ok(weather) => {
                        let rain_intensity = weather.condition.rain_intensity();
                        let snow_intensity = weather.condition.snow_intensity();
                        let fog_intensity = weather.condition.fog_intensity();
                        let wind_speed = weather.wind_speed;
                        let wind_direction = weather.wind_direction;

                        self.state.update_weather(weather);
                        self.animations.update_rain_intensity(rain_intensity);
                        self.animations.update_snow_intensity(snow_intensity);
                        self.animations.update_fog_intensity(fog_intensity);
                        self.animations
                            .update_wind(wind_speed as f32, wind_direction as f32);
                    }
                    Err(error) => {
                        let _error_msg = match &error {
                            WeatherError::Network(net_err) => net_err.user_friendly_message(),
                            _ => format!("Failed to fetch weather: {}", error),
                        };

                        if self.state.current_weather.is_none() {
                            let offline_weather = generate_offline_weather(&mut rng);
                            let rain_intensity = offline_weather.condition.rain_intensity();
                            let snow_intensity = offline_weather.condition.snow_intensity();
                            let fog_intensity = offline_weather.condition.fog_intensity();
                            let wind_speed = offline_weather.wind_speed;
                            let wind_direction = offline_weather.wind_direction;

                            self.state.update_weather(offline_weather);
                            self.state.set_offline_mode(true);
                            self.animations.update_rain_intensity(rain_intensity);
                            self.animations.update_snow_intensity(snow_intensity);
                            self.animations.update_fog_intensity(fog_intensity);
                            self.animations
                                .update_wind(wind_speed as f32, wind_direction as f32);
                        } else {
                            self.state.set_offline_mode(true);
                        }
                    }
                }
            }

            renderer.clear()?;

            let (term_width, term_height) = renderer.get_size();

            self.animations.render_background(
                renderer,
                &self.state.weather_conditions,
                &self.state,
                term_width,
                term_height,
                &mut rng,
            )?;

            self.scene
                .render(renderer, &self.state.weather_conditions)?;

            self.animations.render_chimney_smoke(
                renderer,
                &self.state.weather_conditions,
                term_width,
                term_height,
                &mut rng,
            )?;

            self.animations.render_foreground(
                renderer,
                &self.state.weather_conditions,
                &self.state,
                term_width,
                term_height,
                &mut rng,
            )?;

            self.render_hourly_forecast(renderer, term_width, term_height)?;

            self.state.update_loading_animation();
            self.state.update_cached_info();

            if !self.hide_hud {
                renderer.render_line_colored(
                    2,
                    1,
                    &self.state.cached_weather_info,
                    crossterm::style::Color::Cyan,
                )?;
            }

            if self.show_aqi {
                if let Some(ref aqi) = self.state.current_aqi {
                    self.render_aqi_panel(renderer, aqi, term_width, term_height)?;
                }
            }

            renderer.flush()?;

            let current_poll_fps = INPUT_POLL_FPS as f32 * self.state.speed_multiplier;
            let current_frame_duration = Duration::from_millis((1000.0 / current_poll_fps.max(1.0)) as u64);

            if event::poll(current_frame_duration)? {
                match event::read()? {
                    Event::Resize(width, height) => {
                        renderer.manual_resize(width, height)?;
                    }
                    Event::Key(key_event) => match key_event.code {
                        KeyCode::Char('q') | KeyCode::Char('Q') => break,
                        KeyCode::Char('c')
                            if key_event.modifiers.contains(KeyModifiers::CONTROL) =>
                        {
                            break;
                        }
                        KeyCode::Char('+') | KeyCode::Char('=') => {
                            self.state.speed_multiplier = (self.state.speed_multiplier + 0.25).min(4.0);
                            self.state.weather_info_needs_update = true;
                        }
                        KeyCode::Char('-') => {
                            self.state.speed_multiplier = (self.state.speed_multiplier - 0.25).max(0.25);
                            self.state.weather_info_needs_update = true;
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }

            let (term_width, term_height) = renderer.get_size();
            self.scene.update_size(term_width, term_height);

            self.animations
                .update_sunny_animation(&self.state.weather_conditions);
        }

        Ok(())
    }

    fn render_hourly_forecast(
        &self,
        renderer: &mut TerminalRenderer,
        term_width: u16,
        term_height: u16,
    ) -> io::Result<()> {
        if let Some(weather) = &self.state.current_weather {
            if let Some(hourly) = &weather.hourly_forecast {
                if hourly.is_empty() {
                    return Ok(());
                }

                let panel_height = 6;
                if term_height <= panel_height + 15 {
                    return Ok(()); // Terminal too small to show both scenes and panel
                }

                let start_y = term_height - panel_height;

                let min_temp = hourly.iter().map(|h| h.temperature).fold(f64::INFINITY, f64::min);
                let max_temp = hourly.iter().map(|h| h.temperature).fold(f64::NEG_INFINITY, f64::max);
                let temp_range = (max_temp - min_temp).max(1.0);

                let chart_height = 4;

                let total_width = hourly.len() * 6;
                let start_x = if term_width as usize > total_width {
                    (term_width as usize - total_width) / 2
                } else {
                    0
                };

                let blocks = [' ', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

                for (i, forecast) in hourly.iter().enumerate() {
                    let col_x = start_x + i * 6;
                    if col_x + 5 >= term_width as usize {
                        break;
                    }

                    let normalized = (forecast.temperature - min_temp) / temp_range;
                    let bar_levels = (normalized * (chart_height * 8) as f64).round() as usize;

                    for h in 0..chart_height {
                        let row_y = start_y + (chart_height - 1 - h) as u16;
                        let block_idx = if bar_levels >= (h + 1) * 8 {
                            7
                        } else if bar_levels > h * 8 {
                            bar_levels - h * 8 - 1
                        } else {
                            0
                        };
                        
                        let ch = blocks[block_idx];
                        if ch != ' ' {
                            let color = crossterm::style::Color::DarkCyan;
                            for dx in 0..4 {
                                renderer.render_char((col_x + dx) as u16, row_y, ch, color)?;
                            }
                        }
                    }

                    let time_str = forecast.time.split('T').last().unwrap_or("00:00");
                    renderer.render_line_colored(
                        col_x as u16,
                        start_y + chart_height as u16,
                        time_str,
                        crossterm::style::Color::White,
                    )?;

                    let temp_str = format!("{:.0}°", forecast.temperature);
                    renderer.render_line_colored(
                        col_x as u16,
                        start_y + chart_height as u16 + 1,
                        &temp_str,
                        crossterm::style::Color::Yellow,
                    )?;
                }
            }
        }
        Ok(())
    }

    fn render_aqi_panel(
        &self,
        renderer: &mut TerminalRenderer,
        aqi: &crate::weather::AirQualityData,
        term_width: u16,
        term_height: u16,
    ) -> io::Result<()> {
        let category = crate::weather::AqiCategory::from_european_aqi(aqi.aqi);
        let color = match category {
            crate::weather::AqiCategory::Good => crossterm::style::Color::Green,
            crate::weather::AqiCategory::Fair => crossterm::style::Color::Yellow,
            crate::weather::AqiCategory::Moderate => crossterm::style::Color::DarkYellow,
            crate::weather::AqiCategory::Poor => crossterm::style::Color::Red,
            crate::weather::AqiCategory::VeryPoor => crossterm::style::Color::DarkRed,
            crate::weather::AqiCategory::ExtremelyPoor => crossterm::style::Color::Magenta,
        };

        let lines = [
            format!(" AQI: {:.0} ({}) ", aqi.aqi, category.to_string()),
            format!(" PM2.5: {:.0} μg/m³ ", aqi.pm2_5),
            format!(" PM10 : {:.0} μg/m³ ", aqi.pm10),
            format!(" Ozone: {:.0} μg/m³ ", aqi.ozone),
        ];

        let max_len = lines.iter().map(|l| l.chars().count()).max().unwrap_or(0);
        let panel_width = max_len as u16 + 2; // +2 for borders
        let panel_height = lines.len() as u16 + 2;

        if term_width < panel_width + 4 || term_height < panel_height + 4 {
            return Ok(());
        }

        // Top right corner
        let start_x = term_width - panel_width - 2;
        let start_y = 2; // Below HUD

        // Draw top border
        let top_border = format!("┌{}┐", "─".repeat(max_len));
        renderer.render_line_colored(start_x, start_y, &top_border, color)?;

        // Draw lines
        for (i, line) in lines.iter().enumerate() {
            let padded_line = format!("│{:<width$}│", line, width = max_len);
            renderer.render_line_colored(start_x, start_y + 1 + i as u16, &padded_line, color)?;
        }

        // Draw bottom border
        let bottom_border = format!("└{}┘", "─".repeat(max_len));
        renderer.render_line_colored(start_x, start_y + panel_height - 1, &bottom_border, color)?;

        Ok(())
    }
}
