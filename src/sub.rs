use std::rc::Rc;
use sdl2::image::LoadTexture;
use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};
use crate::{GameState, Renderable};


/// Submarine drawable object
pub struct Sub<'a> {
    x: i32,
    y: i32,
    angle: f32,
    pub velocity: f32,
    gravity: f32,
    pub jump_strength: f32,
    texture: Rc<Texture<'a>>,
}

impl<'a> Sub<'a> {
    pub fn new(canvas: &WindowCanvas, x: i32, y: i32) -> Self {
        let texture_creator = canvas.texture_creator();
        let texture = Rc::new(texture_creator.load_texture("images/sub-large.png").unwrap());

        Sub {
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
        let q = self.texture.query();

        let y = self.y + (self.angle.sin() * 10.0) as i32;
        let _ = canvas.copy(&self.texture, None, Rect::new(self.x, y, q.width, q.height));

        Ok(())
    }

    fn update(&mut self, state: &GameState) {
        if !state.game_started && ! state.game_over {
            // bobbing in pre-game
            self.angle += 0.04;
            self.angle %= 2.0 * std::f32::consts::PI;
        }

        if state.game_over {
            // dead
        }

        if state.game_started && !state.game_over {
            self.angle = 0.0;
            // alive

            self.velocity += self.gravity;
            self.y += self.velocity as i32;

            if self.y > state.window_height as i32 {
                self.y = state.window_height as i32;
                self.velocity = 0.0;
            }
        }
    }
}