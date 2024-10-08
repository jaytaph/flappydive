use std::rc::Rc;
use sdl2::image::LoadTexture;
use sdl2::render::{Canvas, Texture, WindowCanvas};
use crate::{GameState, Renderable};

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
}

pub struct PipeGenerator<'a> {
    pipe_texture: Rc<Texture<'a>>,
    pipe_end_texture: Rc<Texture<'a>>,
    pipes: Vec<Pipe>,
    next_pipe_at: usize,
}

impl<'a> PipeGenerator<'a> {
    pub fn new(canvas: &WindowCanvas) -> Self {
        let texture_creator = canvas.texture_creator();
        let pipe_texture = texture_creator.load_texture("images/pipe.png").unwrap();
        let pipe_end_texture = texture_creator.load_texture("images/pipe-end.png").unwrap();

        Self {
            pipe_texture: Rc::new(pipe_texture),
            pipe_end_texture: Rc::new(pipe_end_texture),
            pipes: Vec::new(),
            next_pipe_at: 0,
        }
    }

    fn generate(&mut self, x: i32, top_offset: i32, bottom_offset: i32) {
        self.pipes.push(Pipe::new(x, top_offset, bottom_offset));
    }
}

impl<'a, T> Renderable<T> for PipeGenerator<'a> {
    fn render(&self, _state: &GameState, _canvas: &mut Canvas<T>) -> Result<(), String> {
        todo!()
        // render each pipe
    }

    fn update(&mut self, _state: &GameState) {
        todo!()
        // update pipes by moving them to the left
        // if pipe is off-screen, remove it
        // if pipe is at the end of the screen, generate a new pipe
        // if self.next_pipe_at > state.fc {
            // New pipe
//        }
    }
}