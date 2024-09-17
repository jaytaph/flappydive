extern crate sdl2;

use std::rc::Rc;
use std::time::Duration;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::image::LoadTexture;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture};
use rand::Rng;

const MAX_BUBBLES: i32 = 25;


#[derive(Debug)]
struct Sub {
    x: i32,
    y: i32,
    velocity: f32,
    gravity: f32,
    jump_strength: f32,
}

impl Default for Sub {
    fn default() -> Self {
        Sub {
            x: 100,
            y: 100,
            velocity: 0.0,
            gravity: 0.1,
            jump_strength: -5.0,
        }
    }
}

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
            rng.gen_range(0..w) as f32,
            (h + rng.gen_range(10..100)) as f32,
            rng.gen_range(-3.0..-0.5),
            Rc::clone(&self.textures[texture_idx]),
        )
    }
}


struct Bubble<'a> {
    x: f32,
    y: f32,
    velocity: f32,
    texture: Rc<Texture<'a>>,
}

impl<'a> Bubble<'a> {
    fn new(x: f32, y: f32, velocity: f32, texture: Rc<Texture<'a>>) -> Self {
        Self {
            x,
            y,
            velocity,
            texture,
        }
    }

    fn update(&mut self) {
        self.y += self.velocity;
    }

    fn finished(&self) -> bool {
        self.y < 0.0
    }

    fn render(&self, canvas: &mut Canvas<sdl2::video::Window>) {
        let q = self.texture.query();

        let _ = canvas.copy(&self.texture, None, Rect::new(self.x as i32, self.y as i32, q.width, q.height));
    }
}


pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("FlappyDive", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();


    let texture_creator = canvas.texture_creator();
    let sub_texture = texture_creator.load_texture("images/sub.png")?;


    let b_sm_texture = Rc::new(texture_creator.load_texture("images/bubble-sm.png")?);
    let b_md_texture = Rc::new(texture_creator.load_texture("images/bubble-md.png")?);
    let b_lg_texture = Rc::new(texture_creator.load_texture("images/bubble-lg.png")?);

    let generator = BubbleGenerator::new(vec![
        Rc::clone(&b_sm_texture),
        Rc::clone(&b_md_texture),
        Rc::clone(&b_lg_texture)
    ]);

    let (ww, wh) = canvas.window().size();

    let mut bubbles = Vec::new();
    for _ in 0..MAX_BUBBLES {
        bubbles.push(generator.generate(ww as i32, wh as i32));
    }



    let mut game_over = false;
    let mut sub = Sub::default();

    let mut event_pump = sdl_context.event_pump().unwrap();

    while !game_over {
        let (_, wh) = canvas.window().size();

        if sub.y < wh as i32 {
            sub.velocity += sub.gravity;
            sub.y = sub.y + sub.velocity as i32;
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    game_over = true;
                }
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    sub.velocity = sub.jump_strength;
                }
                _ => {}
            }
        }

        if sub.y >= wh as i32 || sub.y < 0 {
            game_over = true;
        }

        canvas.set_draw_color(Color::RGB(192, 192, 192));
        canvas.clear();
        canvas.copy(&sub_texture, None, Rect::new(sub.x, sub.y, 50, 45))?;

        for i in 0..bubbles.len() {
            bubbles[i].update();
            bubbles[i].render(&mut canvas);

            if bubbles[i].finished() {
                bubbles[i] = generator.generate(ww as i32, wh as i32);
            }
        }
        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
