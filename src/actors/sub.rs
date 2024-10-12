use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;
use crate::{Collidable, GameState, Renderable};
use crate::theme::Theme;

/// Submarine drawable object
pub struct Sub<'a> {
    initial_x: i32,
    initial_y: i32,
    x: i32,
    y: i32,
    angle: f32,
    pub velocity: f32,
    gravity: f32,
    pub jump_strength: f32,
    texture: Texture<'a>,
}

impl<'a> Sub<'a> {
    pub fn new(x: i32, y: i32, texture_creator: &'a TextureCreator<WindowContext>) -> Self {
        let texture = texture_creator.load_texture("images/sub-large.png").unwrap();

        Sub {
            initial_x: x,
            initial_y: y,
            x,
            y,
            angle: 0.0,
            velocity: 0.0,
            gravity: 0.2,
            jump_strength: -5.0,
            texture,
        }
    }
}

impl<'a> Renderable for Sub<'a> {
    fn render(&self, _state: &GameState, canvas: &mut WindowCanvas) -> Result<(), String> {
        let y = self.y + (self.angle.sin() * 10.0) as i32;

        canvas.copy_ex(&self.texture, None, Rect::new(self.x, y, 50, 45), self.velocity as f64, None, false, false)?;

        // // Draw bounding box
        // canvas.set_draw_color(Color::RED);
        // for bb in self.get_bounding_boxes() {
        //     canvas.draw_rect(bb)?;
        // }

        Ok(())
    }

    fn update(&mut self, state: &GameState) {
        if !state.game_started && ! state.game_over {
            // bobbing in pre-game
            self.angle += 0.04;
            self.angle %= 2.0 * std::f32::consts::PI;
        }

        if state.game_over {
            // dead, upside down
            self.angle = 180.0;
        }

        if state.game_started && !state.game_over {
            self.angle = 0.0;

            self.velocity += self.gravity;
            self.y += self.velocity as i32;

            if self.velocity < -10.0 {
                self.velocity = -10.0;
            }
            if self.velocity > 10.0 {
                self.velocity = 10.0;
            }

            if self.y > state.window_height as i32 {
                self.y = state.window_height as i32;
                self.velocity = 0.0;
            }
        }
    }

    fn switch_theme(&mut self, theme: &Theme) {
        self.texture.set_color_mod(theme.sub.0, theme.sub.1, theme.sub.2);
    }

    fn reset(&mut self) {
        self.x = self.initial_x;
        self.y = self.initial_y;
        self.angle = 0.0;
        self.velocity = 0.0;
    }
}

impl<'a> Collidable for Sub<'a> {
    fn get_bounding_boxes(&self) -> Vec<Rect> {
        let y = self.y + (self.angle.sin() * 10.0) as i32;

        vec![Rect::new(self.x, y, 50, 45)]
    }
}