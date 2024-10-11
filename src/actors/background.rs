use rand::Rng;
use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;
use crate::{GameState, Renderable};
use crate::theme::Theme;

#[allow(dead_code)]
pub struct BackgroundObject {
    x: i32,
    y: i32,
    texture_idx: usize,
}

impl BackgroundObject {
    pub fn new(x: i32, y: i32, texture_idx: usize) -> Self {
        Self { x, y, texture_idx }
    }

    pub fn update(&mut self, x_speed: i32) {
        self.x -= x_speed;
    }

    pub fn is_finished(&self) -> bool {
        self.x < -100
    }
}

pub struct Background<'a> {
    objects: Vec<BackgroundObject>,
    new_object_at_fc: i64,
    textures: Vec<Texture<'a>>,
    sand_highlights: Vec<(i32, i32)>
}

impl<'a> Background<'a> {
    pub fn new(state: &GameState, texture_creator: &'a TextureCreator<WindowContext>) -> Self {
        let mut rng = rand::thread_rng();

        let theme = state.theme.current();

        let mut texture_axolotl = texture_creator.load_texture("images/axolotl.png").unwrap();
        texture_axolotl.set_color_mod(theme.fauna_color_1.0, theme.fauna_color_1.1, theme.fauna_color_1.2);
        let textures = vec![texture_axolotl];

        // Small darker pixels in the sand
        let mut sand_highlights = vec![];
        for _ in 0..100 {
            sand_highlights.push((rng.gen_range(0..800), rng.gen_range(400..600)));
        }

        Self {
            objects: Vec::new(),
            new_object_at_fc: rng.gen_range(0..100),
            textures,
            sand_highlights
        }
    }
}

impl<'a> Renderable for Background<'a> {
    fn render(&self, state: &GameState, canvas: &mut WindowCanvas) -> Result<(), String> {
        let theme = state.theme.current();
        let (ww, wh) = canvas.window().size();

        canvas.set_draw_color(Color::RGB(theme.water.0, theme.water.1, theme.water.2));
        canvas.clear();

        // Print ground line
        let y = wh - (wh / 3);
        canvas.set_draw_color(Color::RGB(theme.sand.0, theme.sand.1, theme.sand.2));
        canvas.fill_rect(Rect::new(0, y as i32, ww, wh - y))?;

        // Print sand highlights
        for (x, y) in &self.sand_highlights {
            canvas.set_draw_color(Color::RGB(theme.sand_highlight.0, theme.sand_highlight.1, theme.sand_highlight.2));
            canvas.fill_rect(Rect::new(*x, *y, 2, 2))?;
        }

        // Render all objects
        for obj in &self.objects {
            let texture = &self.textures[obj.texture_idx];
            let q = texture.query();
            let rect = Rect::new(obj.x, obj.y, q.width, q.height);
            canvas.copy(texture, None, rect)?;
        }

        Ok(())
    }

    fn update(&mut self, state: &GameState) {
        // Update sand highlights and reset them if they go off-screen
        for (x, y) in &mut self.sand_highlights {
            *x -= state.x_speed;
            if *x < 0 {
                *x = 800;
                *y = rand::thread_rng().gen_range(400..600);
            }
        }

        // Maybe add some other background elements here, sand, rocks, shipwreck, axolotl, etc.
        if state.fc >= self.new_object_at_fc {
            let mut rng = rand::thread_rng();

            let obj = BackgroundObject::new(
                800,
                rng.gen_range(450..550),
                rng.gen_range(0..self.textures.len())
            );

            self.objects.push(obj);

            self.new_object_at_fc = state.fc + rng.gen_range(50..300);
        }

        // Any objects are moved here
        for obj in &mut self.objects {
            obj.update(state.x_speed);
        }

        // Remove objects that are off-screen
        self.objects.retain(|obj| !obj.is_finished());
    }

    fn switch_theme(&mut self, theme: &Theme) {
        for texture in self.textures.iter_mut() {
            texture.set_color_mod(theme.fauna_color_1.0, theme.fauna_color_1.1, theme.fauna_color_1.2);
        }
    }

    fn reset(&mut self) {
        // No need to reset
    }
}