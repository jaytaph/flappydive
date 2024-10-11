use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use crate::{GameState, Renderable, TTF};
use crate::theme::Theme;

pub struct Score<'a> {
    ttf: &'a TTF<'a>,
}

impl<'a> Score<'a> {
    pub fn new(ttf: &'a TTF) -> Self {
        Self { ttf }
    }
}

impl<'a> Renderable for Score<'a> {
    fn render(&self, state: &GameState, canvas: &mut WindowCanvas) -> Result<(), String> {
        let theme = state.theme.current();

        // Print score
        let surface = self.ttf.font
            .render(format!("Score: {:06}   Hi-Score: {:06}", state.fc, state.high_score).as_str())
            .blended(Color::RGBA(theme.text.0, theme.text.1, theme.text.2, 255))
            .map_err(|e| e.to_string())?;

        let creator = canvas.texture_creator();
        let texture = creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        canvas.copy(&texture, None, Rect::new(20, 10, 300, 30))?;

        Ok(())
    }

    fn update(&mut self, _state: &GameState) {
    }

    fn switch_theme(&mut self, _theme: &Theme) {
    }

    fn reset(&mut self) {
    }
}