use std::rc::Rc;
use rand::Rng;
use sdl2::rect::Rect;
use sdl2::image::LoadTexture;
use sdl2::render::{Texture, WindowCanvas};
use crate::{GameState, Renderable};

/// Single bubble
struct Bubble<'a> {
    x: f32,
    y: f32,
    velocity_y: f32,
    texture: Rc<Texture<'a>>,
}

impl<'a> Bubble<'a> {
    fn new(x: f32, y: f32, velocity_y: f32, texture: Rc<Texture<'a>>) -> Self {
        Self {
            x,
            y,
            velocity_y,
            texture,
        }
    }

    fn finished(&self) -> bool {
        self.y < 0.0 || self.x < 0.0
    }
}

/// Bubbles is a collection of Bubble objects that are generated, rendered and updated
pub struct Bubbles<'a> {
    bubbles: Vec<Bubble<'a>>,
    generator: BubbleGenerator<'a>,
    max_bubbles: usize,
}

impl<'a> Bubbles<'a> {
    pub fn new(canvas: &WindowCanvas, max_bubbles: usize) -> Self {
        let texture_creator = canvas.texture_creator();

        let b_sm_texture = Rc::new(texture_creator.load_texture("images/bubble-sm.png").unwrap());
        let b_md_texture = Rc::new(texture_creator.load_texture("images/bubble-md.png").unwrap());
        let b_lg_texture = Rc::new(texture_creator.load_texture("images/bubble-lg.png").unwrap());

        let textures = vec![b_sm_texture, b_md_texture, b_lg_texture];

        let mut bubbles = Self{
            bubbles: Vec::new(),
            generator: BubbleGenerator::new(textures),
            max_bubbles,
        };

        let (w, h) = canvas.output_size().unwrap();

        for _ in 0..max_bubbles {
            bubbles.bubbles.push(bubbles.generator.generate(w as i32, h as i32));
        }

        bubbles
    }
}

impl<'a> Renderable for Bubbles<'a> {
    fn render(&self, state: &GameState, canvas: &mut WindowCanvas) -> Result<(), String> {
        for bubble in self.bubbles.iter() {
            bubble.render(state, canvas);
        }

        Ok(())
    }

    fn update(&mut self, state: &GameState) {
        self.bubbles.retain(|bubble| !bubble.finished());

        if self.bubbles.len() < self.max_bubbles {
            self.bubbles.push(self.generator.generate(state.window_width as i32, state.window_height as i32));
        }

        for bubble in self.bubbles.iter_mut() {
            bubble.update(&state);
        }
    }
}

impl<'a> Renderable for Bubble<'a> {
    fn render(&self, _state: &GameState, canvas: &mut WindowCanvas) -> Result<(), String> {
        let q = self.texture.query();
        let _ = canvas.copy(&self.texture, None, Rect::new(self.x as i32, self.y as i32, q.width, q.height));

        Ok(())
    }

    fn update(&mut self, state: &GameState) {
        self.x += state.x_speed as f32;
        self.y += self.velocity_y;
    }
}


/// Generator that can generate a new bubble
struct BubbleGenerator<'a> {
    textures: Vec<Rc<Texture<'a>>>,
}

impl<'a> BubbleGenerator<'a> {
    fn new(textures: Vec<Rc<Texture<'a>>>) -> Self {
        Self {
            textures,
        }
    }

    fn generate(&self, w: i32, h: i32) -> Bubble {
        let mut rng = rand::thread_rng();
        let texture_idx = rng.gen_range(0..self.textures.len());

        Bubble::new(
            rng.gen_range(0..w + 600) as f32,
            (h + rng.gen_range(10..100)) as f32,
            -3.0,
            Rc::clone(&self.textures[texture_idx]),
        )
    }
}