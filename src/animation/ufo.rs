use crate::render::TerminalRenderer;
use crossterm::style::Color;
use rand::prelude::*;
use std::io;

#[derive(Clone)]
struct Ufo {
    x: f32,
    y: f32,
    speed: f32,
    wobble_phase: f32,
}

pub struct UfoSystem {
    ufos: Vec<Ufo>,
    terminal_width: u16,
    terminal_height: u16,
    spawn_cooldown: u16,
}

impl UfoSystem {
    pub fn new(terminal_width: u16, terminal_height: u16) -> Self {
        Self {
            ufos: Vec::with_capacity(1),
            terminal_width,
            terminal_height,
            spawn_cooldown: 0,
        }
    }

    pub fn update(&mut self, terminal_width: u16, terminal_height: u16, rng: &mut impl Rng) {
        self.terminal_width = terminal_width;
        self.terminal_height = terminal_height;

        for ufo in &mut self.ufos {
            ufo.x += ufo.speed;
            ufo.wobble_phase += 0.1;
            ufo.y += ufo.wobble_phase.sin() * 0.2; // vertical wobble
        }

        self.ufos
            .retain(|p| p.x < terminal_width as f32 && p.x > -20.0);

        self.spawn_cooldown = self.spawn_cooldown.saturating_sub(1);
        if self.spawn_cooldown == 0 && rng.random::<f32>() < 0.005 {
            self.spawn_ufo(rng);
            self.spawn_cooldown = 300 + (rng.random::<u16>() % 300); // Let it spawn relatively quickly for fun!
        }
    }

    fn spawn_ufo(&mut self, rng: &mut impl Rng) {
        let y = (rng.random::<u16>() % (self.terminal_height / 3)) as f32;
        let speed = 0.5 + (rng.random::<f32>() * 0.5); // A bit faster than airplanes

        self.ufos.push(Ufo {
            x: -15.0,
            y,
            speed,
            wobble_phase: 0.0,
        });
    }

    pub fn render(&self, renderer: &mut TerminalRenderer) -> io::Result<()> {
        let ufo_art = [
            "    .---.    ",
            "  _/__~0_\\_  ",
            " (_________) ",
            "   *  *  *   ",
        ];

        for ufo in &self.ufos {
            let x = ufo.x as i16;
            let y = ufo.y as i16;

            for (line_offset, line) in ufo_art.iter().enumerate() {
                let render_y = y + line_offset as i16;
                if render_y < 0 || render_y >= self.terminal_height as i16 {
                    continue;
                }

                for (char_offset, ch) in line.chars().enumerate() {
                    let render_x = x + char_offset as i16;
                    if render_x < 0 || render_x >= self.terminal_width as i16 {
                        continue;
                    }

                    if ch != ' ' {
                        let color = match ch {
                            '~' | '0' => Color::Green,
                            '*' => Color::Cyan, // Lights beneath
                            '-' | '_' | '\\' | '/' | '(' | ')' | '.' => Color::Grey,
                            _ => Color::White,
                        };
                        renderer.render_char(render_x as u16, render_y as u16, ch, color)?;
                    }
                }
            }
        }
        Ok(())
    }
}
