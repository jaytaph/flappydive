use rand::Rng;
use sdl2::rect::Rect;
use sdl2::image::LoadTexture;
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;
use crate::{GameState, Renderable};
use crate::theme::Theme;

/// Single bubble
struct Bubble {
    x: f32,
    y: f32,
    max_y: f32,
    velocity_y: f32,
    texture_idx: usize,
}

impl Bubble {
    fn new(x: f32, y: f32, max_y: f32, velocity_y: f32, texture_idx: usize) -> Self {
        Self {
            x,
            y,
            velocity_y,
            max_y,
            texture_idx,
        }
    }

    fn finished(&self) -> bool {
        self.y < self.max_y || self.x < 0.0
    }

    fn update(&mut self, speed: f32) {
        self.x -= speed;
        self.y += self.velocity_y;
    }
}

/// Bubbles is a collection of Bubble objects that are generated, rendered and updated
pub struct Bubbles<'a> {
    bubbles: Vec<Bubble>,
    max_bubbles: usize,
    textures: Vec<Texture<'a>>,
}

impl<'a> Bubbles<'a> {
    pub fn new(max_bubbles: usize, texture_creator: &'a TextureCreator<WindowContext>, w: u32, h: u32) -> Self {
        let b_sm_texture = texture_creator.load_texture("images/bubble-sm.png").unwrap();
        let b_md_texture = texture_creator.load_texture("images/bubble-md.png").unwrap();
        let b_lg_texture = texture_creator.load_texture("images/bubble-lg.png").unwrap();
        let textures = vec![b_sm_texture, b_md_texture, b_lg_texture];

        let mut bubbles = Self{
            bubbles: Vec::new(),
            max_bubbles,
            textures,
        };

        for _ in 0..max_bubbles {
            bubbles.bubbles.push(bubbles.generate(w as i32, h as i32));
        }

        bubbles
    }

    fn generate(&self, w: i32, h: i32) -> Bubble {
        let mut rng = rand::thread_rng();

        let x = rng.gen_range(0..(w + 300));
        let y = h as f32;

        // Maximum height of the bubble before it pops. If less than screen, than cap to top of screen (about -20)
        let mut max_y = rng.gen_range(-200..(y / 2.0) as i32);
        if max_y < -20 {
            max_y = -20;
        }
        let velocity_y = rng.gen_range(-3.0..-0.5);
        let texture_idx = rng.gen_range(0..self.textures.len());

        Bubble::new(x as f32, y, max_y as f32, velocity_y, texture_idx)
    }
}

impl<'a> Renderable for Bubbles<'a> {
    fn render(&self, _state: &GameState, canvas: &mut WindowCanvas) -> Result<(), String> {
        for bubble in self.bubbles.iter() {
            let q = &self.textures[bubble.texture_idx].query();
            canvas.copy(&self.textures[bubble.texture_idx], None, Rect::new(bubble.x as i32, bubble.y as i32, q.width, q.height))?;
        }

        Ok(())
    }

    fn update(&mut self, state: &GameState) {
        if self.bubbles.len() < self.max_bubbles {
            let bubble = self.generate(state.window_width as i32, state.window_height as i32);
            self.bubbles.push(bubble);
        }

        for bubble in self.bubbles.iter_mut() {
            bubble.update(state.x_speed as f32);
        }

        self.bubbles.retain(|bubble| !bubble.finished());
    }

    fn switch_theme(&mut self, theme: &Theme) {
        for texture in self.textures.iter_mut() {
            texture.set_color_mod(theme.bubbles.0, theme.bubbles.1, theme.bubbles.2);
        }
    }

    fn reset(&mut self) {
        // No need to do anything. Bubbles will flow
    }
}
