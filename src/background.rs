use std::rc::Rc;
use rand::Rng;
use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};
use sdl2::ttf::Font;
use crate::{GameState, Renderable};

pub struct BackgroundObject<'a> {
    x: i32,
    y: i32,
    texture: Rc<Texture<'a>>,
}

pub struct Background<'a> {
    objects: Vec<BackgroundObject<'a>>,
    new_object_at_fc: i64,
    font: Font<'a, 'static>,
    textures: Vec<Rc<Texture<'a>>>,
}

impl<'a> Background<'a> {
    pub fn new(canvas: &WindowCanvas) -> Self {
        let mut rng = rand::thread_rng();

        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();
        let mut font = ttf_context.load_font("images/Lato-Regular.ttf", 128).unwrap();
        font.set_style(sdl2::ttf::FontStyle::BOLD);

        let texture_creator = canvas.texture_creator();
        let texture = Rc::new(texture_creator.load_texture("images/axolotl.png").unwrap());

        Self {
            objects: Vec::new(),
            new_object_at_fc: rng.gen_range(0..100),
            font,
            textures: vec![texture],
        }
    }
}

impl<'a> Renderable for Background<'a> {
    fn render(&self, state: &GameState, canvas: &mut WindowCanvas) -> Result<(), String> {
        let (ww, wh) = canvas.window().size();

        canvas.set_draw_color(Color::RGB(192, 192, 192));
        canvas.clear();

        // Print ground line
        let y = wh - (wh / 3);
        canvas.set_draw_color(Color::RGB(164, 164, 164));
        canvas.fill_rect(Rect::new(0, y as i32, ww, wh - y))?;

        // Print score
        let surface = self.font
            .render(format!("Score: {:06}   Hi-Score: {:06}", state.fc, state.high_score).as_str())
            .blended(Color::RGBA(64, 64, 64, 255))
            .map_err(|e| e.to_string())?;

        let texture = canvas.texture_creator()
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        canvas.copy(&texture, None, Rect::new(20, 10, 300, 30))?;

        Ok(())
    }

    fn update(&mut self, state: &GameState) {
        // Maybe add some other background elements here, sand, rocks, shipwreck, axolotl, etc.
        if state.fc >= self.new_object_at_fc {
            let mut rng = rand::thread_rng();
            let x = 800;
            let y = rng.gen_range(0..400);

            self.objects.push(BackgroundObject { x, y, texture: self.textures[0].clone() });

            self.new_object_at_fc = state.fc + rng.gen_range(50..300);
        }

        // Any objects are moved here
        for obj in &mut self.objects {
            obj.x -= state.x_speed;
        }

        // Remove objects that are off-screen
        self.objects.retain(|obj| obj.x + obj.texture.query().width as i32 > 0);
    }
}