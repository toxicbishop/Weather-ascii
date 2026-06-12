use crate::render::TerminalRenderer;
use crate::weather::WeatherConditions;
use crossterm::style::Color;
use rand::{Rng, RngExt};
use std::io;

struct SmogParticle {
    x: f32,
    y: f32,
    speed: f32,
    char_idx: usize,
}

const SMOG_CHARS: [char; 3] = ['~', '-', '.'];

pub struct Smog {
    particles: Vec<SmogParticle>,
    width: u16,
    height: u16,
}

impl Smog {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            particles: Vec::new(),
            width,
            height,
        }
    }

    pub fn resize(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
        self.particles.retain(|p| p.x < width as f32 && p.y < height as f32);
    }

    pub fn update(
        &mut self,
        should_show: bool,
        speed_multiplier: f32,
        _conditions: &WeatherConditions,
        rng: &mut impl Rng,
    ) {
        if !should_show {
            self.particles.clear();
            return;
        }

        let target_particles = (self.width as usize * self.height as usize) / 80;
        if self.particles.len() < target_particles {
            if rng.random_bool(0.3) {
                self.particles.push(SmogParticle {
                    x: rng.random_range(0.0..self.width as f32),
                    y: rng.random_range(0.0..self.height as f32),
                    speed: rng.random_range(0.05..0.15) * speed_multiplier,
                    char_idx: rng.random_range(0..SMOG_CHARS.len()),
                });
            }
        }

        for p in &mut self.particles {
            p.x += p.speed;
            
            if rng.random_bool(0.1) {
                p.y += rng.random_range(-0.1..0.1);
            }

            if p.x >= self.width as f32 {
                p.x = 0.0;
                p.y = rng.random_range(0.0..self.height as f32);
            }
            if p.y < 0.0 {
                p.y = self.height as f32 - 1.0;
            }
            if p.y >= self.height as f32 {
                p.y = 0.0;
            }
        }
    }

    pub fn render(
        &self,
        renderer: &mut TerminalRenderer,
        should_show: bool,
        is_day: bool,
    ) -> io::Result<()> {
        if !should_show {
            return Ok(());
        }

        let (r, g, b) = if is_day {
            (140, 130, 110)
        } else {
            (80, 75, 65)
        };

        for p in &self.particles {
            let ch = SMOG_CHARS[p.char_idx];
            renderer.render_char(
                p.x as u16,
                p.y as u16,
                ch,
                Color::Rgb { r, g, b },
            )?;
        }

        Ok(())
    }
}
