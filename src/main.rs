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

const MAX_BUBBLES: i32 = 10;

struct Pipe {
    x: i32,
    top_offset: i32,
    bottom_offset: i32,
}

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
            rng.gen_range(0..w + 600) as f32,
            (h + rng.gen_range(10..100)) as f32,
            -3.0,
            rng.gen_range(-3.0..-0.5),
            Rc::clone(&self.textures[texture_idx]),
        )
    }
}


struct Bubble<'a> {
    x: f32,
    y: f32,
    velocity_x: f32,
    velocity_y: f32,
    texture: Rc<Texture<'a>>,
}

impl<'a> Bubble<'a> {
    fn new(x: f32, y: f32, velocity_x: f32, velocity_y: f32, texture: Rc<Texture<'a>>) -> Self {
        Self {
            x,
            y,
            velocity_x,
            velocity_y,
            texture,
        }
    }

    fn update(&mut self, x_speed: f32) {
        self.x += x_speed;
        self.y += self.velocity_y;
    }

    fn finished(&self) -> bool {
        self.y < 0.0 || self.x < 0.0
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


    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let mut font = ttf_context.load_font("images/Lato-Regular.ttf", 128)?;
    font.set_style(sdl2::ttf::FontStyle::BOLD);


    let texture_creator = canvas.texture_creator();
    // let sub_texture = texture_creator.load_texture("images/sub.png")?;
    let sub_texture = texture_creator.load_texture("images/sub-large.png")?;

    let pipe_texture = texture_creator.load_texture("images/pipe.png")?;


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

    let mut pipes = Vec::new();

    let mut game_started = false;
    let mut game_over = false;
    let mut sub = Sub::default();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut angle = 0.0;

    while !game_started {
        let (_, wh) = canvas.window().size();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    game_started = true;
                    game_over = true;
                }
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    game_started = true;
                }
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(192, 192, 192));
        canvas.clear();


        let y = (wh - (wh / 3)) as i32;
        canvas.set_draw_color(Color::RGB(64, 64, 64));
        canvas.draw_line((0, y), (ww as i32, y))?;
        // canvas.set_draw_color(Color::RGB(128, 128, 128));
        // canvas.draw_rect(Rect::new(0, 0, ww, wh - 100))?;


        let surface = font
            .render("Press <space> to begin")
            .blended(Color::RGBA(128, 128, 128, 255))
            .map_err(|e| e.to_string())?;
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        canvas.copy(&texture, None, Rect::new((ww/2) as i32 - 200,(wh/2) as i32 - 30, 400, 60))?;


        angle += 0.04;
        angle %= 2.0 * std::f64::consts::PI;

        let a = angle.sin() * 15.0;
        canvas.copy(&sub_texture, None, Rect::new(sub.x, sub.y + a as i32, 50, 45))?;

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    let mut fc = 0;
    while !game_over {
        fc += 1;

        if fc % 100 == 0 {
            pipes.push(Pipe{
                x: 800,
                top_offset: 0,
                bottom_offset: 0,
            })
        }

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


        // Print submarine
        let angle = match sub.velocity {
            v if v < 0.0 => -15.0,
            v if v > 0.0 => 15.0,
            _ => 0.0,
        };
        canvas.copy_ex(&sub_texture, None, Rect::new(sub.x, sub.y, 50, 45), angle, None, false, false)?;


        for i in 0..pipes.len() {
            pipes[i].x -= 3;

            let q = pipe_texture.query();
            let qh = (q.height / 5) as i32;
            let qw = (q.width / 10) as i32;

            canvas.copy_ex(&pipe_texture, None, Rect::new(pipes[i].x, 0, qw as u32, qh as u32), 0.0, None, false, true)?;
            canvas.copy(&pipe_texture, None, Rect::new(pipes[i].x, (wh - qh as u32) as i32, qw as u32, qh as u32))?;
        }

        let y = (wh - (wh / 3)) as i32;
        canvas.set_draw_color(Color::RGB(64, 64, 64));
        canvas.draw_line((0, y), (ww as i32, y))?;


        for i in (0..pipes.len()).rev() {
            let q = pipe_texture.query();
            let qw = (q.width / 10) as i32;

            if pipes[i].x < -qw  {
                pipes.remove(i);
            }
        }

        // Print bubbles
        for i in 0..bubbles.len() {
            bubbles[i].update(if game_started { -3.0 } else { 0.0 });
            bubbles[i].render(&mut canvas);

            if bubbles[i].finished() {
                bubbles[i] = generator.generate(ww as i32, wh as i32);
            }
        }

        let surface = font
            .render(format!("Score: {:08}", fc).as_str())
            .blended(Color::RGBA(64, 64, 64, 255))
            .map_err(|e| e.to_string())?;
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;
        canvas.copy(&texture, None, Rect::new(200, 10, 100, 20))?;

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
