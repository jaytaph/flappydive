use sdl2::image::LoadTexture;
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;
use crate::{Collidable, GameState, Renderable};
use rand::Rng;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use crate::theme::{Theme, THEME};

struct Pipe {
    x: i32,
    top_offset: i32,
    bottom_offset: i32,
}

impl Pipe {
    fn new(x: i32, top_offset: i32, bottom_offset: i32) -> Self {
        Self {
            x,
            top_offset,
            bottom_offset,
        }
    }

    fn finished(&self) -> bool {
        self.x < -50.0 as i32
    }

    fn update(&mut self, speed: i32) {
        self.x -= speed;
    }
}

pub struct Pipes<'a> {
    pipe_texture: Texture<'a>,
    pipe_end_texture: Texture<'a>,
    pipes: Vec<Pipe>,
    next_pipe_at: i64,
}

impl<'a> Pipes<'a> {
    pub fn new(texture_creator: &'a TextureCreator<WindowContext>) -> Self {
        let mut pipe_texture = texture_creator.load_texture("images/pipe.png").unwrap();
        let mut pipe_end_texture = texture_creator.load_texture("images/pipe-end.png").unwrap();

        pipe_texture.set_color_mod(THEME.pipes.0, THEME.pipes.1, THEME.pipes.2);
        pipe_end_texture.set_color_mod(THEME.pipes.0, THEME.pipes.1, THEME.pipes.2);


        Self {
            pipe_texture: pipe_texture,
            pipe_end_texture: pipe_end_texture,
            pipes: Vec::new(),
            next_pipe_at: 0,
        }
    }

    fn generate(&self, x: i32, height: u32) -> Pipe {
        let mut rng = rand::thread_rng();

        let hole_size = rng.gen_range(150..250);
        let hole_offset = rng.gen_range(50..height - hole_size - 50);

        Pipe::new(x, hole_offset as i32, (hole_offset + hole_size) as i32)
    }
}

impl<'a> Renderable for Pipes<'a> {
    fn render(&self, _state: &GameState, canvas: &mut WindowCanvas) -> Result<(), String> {
        let q = self.pipe_texture.query();
        let qe = self.pipe_end_texture.query();

        // render each pipe
        for pipe in &self.pipes {
            // Top pipe
            canvas.copy_ex(
                &self.pipe_texture,
                None,
                Rect::new(pipe.x, 0, 50, pipe.top_offset as u32),
                0.0,
                None,
                false,
                true
            )?;

            canvas.copy_ex(
                &self.pipe_end_texture,
                None,
                Rect::new(pipe.x - 7, pipe.top_offset - 10, qe.width - 20, qe.height - 10),
                0.0,
                None,
                false,
                true
            )?;

            // Bottom pipe
            canvas.copy(
                &self.pipe_texture,
                None,
                Rect::new(pipe.x, pipe.bottom_offset, 50, q.height + 50 - pipe.bottom_offset as u32),
            )?;

            canvas.copy(
                &self.pipe_end_texture,
                None,
                Rect::new(pipe.x - 7, pipe.bottom_offset, qe.width - 20, qe.height - 10),
            )?;
        }

        // // Draw bounding boxes
        // canvas.set_draw_color(Color::RED);
        // for bb in self.get_bounding_boxes() {
        //     canvas.draw_rect(bb)?;
        // }

        Ok(())
    }

    fn update(&mut self, state: &GameState) {
        let mut rng = rand::thread_rng();

        // Add a new pipe when it's time for one
        if state.fc > self.next_pipe_at {
            // New pipe
            let new_pipe = self.generate(state.window_width as i32, state.window_height);
            self.pipes.push(new_pipe);

            self.next_pipe_at = state.fc + rng.gen_range(75..200);
        }

        // Move all the pipes
        for pipe in self.pipes.iter_mut() {
            pipe.update(state.x_speed);
        }

        // Remove pipes that are off-screen
        self.pipes.retain(|pipe| !pipe.finished());
    }

    fn switch_theme(&mut self, theme: &Theme) {
        self.pipe_texture.set_color_mod(theme.pipes.0, theme.pipes.1, theme.pipes.2);
        self.pipe_end_texture.set_color_mod(theme.pipes.0, theme.pipes.1, theme.pipes.2);
    }

    fn reset(&mut self) {
        self.pipes.clear();
        self.next_pipe_at = 0;
    }
}

impl<'a> Collidable for Pipes<'a> {
    fn get_bounding_boxes(&self) -> Vec<Rect> {
        let mut boxes = vec![];

        for pipe in &self.pipes {
            boxes.push(Rect::new(pipe.x - 5, 0, 60, pipe.top_offset as u32 + 20));
            boxes.push(Rect::new(pipe.x - 5, pipe.bottom_offset,60, 600));

        }

        boxes
    }
}