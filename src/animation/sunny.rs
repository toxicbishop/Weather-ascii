use super::Animation;
use crossterm::style::Color;

pub struct SunnyAnimation {
    frames: Vec<Vec<String>>,
}

impl SunnyAnimation {
    pub fn new() -> Self {
        let frames = vec![Self::create_frame_1(), Self::create_frame_2()];

        Self { frames }
    }

    fn create_frame_1() -> Vec<String> {
        vec![
            "      ;   :   ;".to_string(),
            "   .   \\_,!,_/   ,".to_string(),
            "    `.,'     `.,'".to_string(),
            "     /         \\".to_string(),
            "~ -- :         : -- ~".to_string(),
            "     \\         /".to_string(),
            "    ,'`._   _.'`.".to_string(),
            "   '   / `!` \\   `".to_string(),
            "      ;   :   ;".to_string(),
        ]
    }

    fn create_frame_2() -> Vec<String> {
        vec![
            "      .   |   .".to_string(),
            "   ;   \\_,|,_/   ;".to_string(),
            "    `.,'     `.,'".to_string(),
            "     /         \\".to_string(),
            "~ -- |         | -- ~".to_string(),
            "     \\         /".to_string(),
            "    ,'`._   _.'`.".to_string(),
            "   ;   / `|` \\   ;".to_string(),
            "      .   |   .".to_string(),
        ]
    }
}

impl Animation for SunnyAnimation {
    fn get_frame(&self, frame_number: usize) -> &[String] {
        &self.frames[frame_number % self.frames.len()]
    }

    fn frame_count(&self) -> usize {
        self.frames.len()
    }

    fn get_color(&self) -> Color {
        Color::Yellow
    }
}

impl Default for SunnyAnimation {
    fn default() -> Self {
        Self::new()
    }
}
