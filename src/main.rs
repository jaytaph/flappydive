mod bubble;
mod sub;
mod pipe;
mod background;

extern crate sdl2;

use std::time::Duration;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, RenderTarget, WindowCanvas};
use crate::background::Background;
use crate::bubble::Bubbles;
use crate::pipe::PipeGenerator;
use crate::sub::Sub;

const MAX_BUBBLES: usize = 10;

struct GameState {
    /// True if the game has started. False if not started, or has ended
    game_started: bool,
    /// True if the game is over
    game_over: bool,
    /// Current frame counter
    fc: i64,
    /// Highest frame counter score encountered
    high_score: i64,
    /// Canvas to draw on
    canvas: WindowCanvas,
    /// Speed of the current game
    x_speed: i32,
}

/// A renderable is something that can be rendered onto screen and has its own update functionality
trait Renderable<T: RenderTarget> {
    fn render(&self, state: &GameState, canvas: &mut Canvas<T>) -> Result<(), String>;
    fn update(&mut self, state: &GameState);
}


pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("FlappyDive", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();


    // let texture_creator = canvas.texture_creator();
    // let axolotl_texture = texture_creator.load_texture("images/axolotl.png")?;

    let mut state = GameState {
        game_started: false,
        game_over: false,
        fc: 0,
        highscore: 0,
        canvas,
        x_speed: 10,
    };

    let mut background = Background::new(&state.canvas);
    let mut sub = Sub::new(&state.canvas, 100, 100);
    let mut bubbles = Bubbles::new(&state.canvas, MAX_BUBBLES);
    let mut pipes = PipeGenerator::new(&state.canvas);

    let mut first_run = true;

    loop {
        let mut event_pump = sdl_context.event_pump()?;

        // Do pregame
        do_pregame(&mut state, first_run, &mut event_pump, &mut background, &mut sub, &mut bubbles)?;
        first_run = false;

        // run a game
        do_game(&mut state, &mut event_pump, &mut background, &mut sub, &mut bubbles, &mut pipes)?;

        // Update score and reinitialize game
        if state.fc > state.highscore {
            state.highscore = state.fc;
        }

        state.fc = 0;
        state.game_started = false;
        state.game_over = false;
    }
}

// Returns Ok(true) when the game can begin. Returns ok(false) when we want to quit
fn do_pregame(state: &mut GameState, first_run: bool, event_pump: &mut sdl2::EventPump, background: &mut Background, sub: &mut Sub, bubbles: &mut Bubbles) -> Result<bool, String> {

    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let mut font = ttf_context.load_font("images/Lato-Regular.ttf", 128)?;
    font.set_style(sdl2::ttf::FontStyle::BOLD);

    loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    return Ok(false);
                }
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    state.game_started = true;
                    return Ok(true);
                }
                _ => {}
            }
        }

        // Render stuff
        let _ = background.render(&state, &mut state.canvas);
        let _ = sub.render(&state, &mut state.canvas);
        let _ = bubbles.render(&state, &mut state.canvas);


        // Print message
        let s = if first_run { "Press <space> to begin" } else { "You sunk. Press <space> to try again" };

        let surface = font
            .render(s)
            .blended(Color::RGBA(128, 128, 128, 255))
            .map_err(|e| e.to_string())?;

        let texture = state.canvas.texture_creator()
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;
        state.canvas.copy(&texture, None, Rect::new(300, 300, 400, 60))?;

        state.canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn do_game(state: &mut GameState, event_pump: &mut sdl2::EventPump, background: &mut Background, sub: &mut Sub, bubbles: &mut Bubbles, pipes: &mut PipeGenerator) -> Result<bool, String> {
    while !state.game_over {
        state.fc += 1;

        pipes.update(state);
        background.update(state);
        sub.update(state);
        bubbles.update(state);

        if collision_detected(sub, pipes) {
            state.game_over = true;
            return Ok(true)
        }
    }

    // // Generate new pipe if it's time for a new one
    // if fc > next_pipe_fc {
    //     let mut rng = rand::thread_rng();
    //     next_pipe_fc = fc + rng.gen_range(75..200);
    //
    //     let hole_size = rng.gen_range(150..250);
    //     let hole_offset = rng.gen_range(50..600);
    //
    //     pipes.generate(800, hole_offset as i32, (hole_offset + hole_size) as i32);
    // }

    // // Update the submarine
    // let (_, wh) = state.canvas.output_size().unwrap();
    // if sub.y < wh as i32 {
    //     sub.velocity += sub.gravity;
    //     sub.y = sub.y + sub.velocity as i32;
    // }

    // // Moved the pipes
    // pipes.update(state);

    // // Check if the sub hit the top or bottom
    // if sub.y >= wh as i32 || sub.y < 0 {
    //     state.game_over = true;
    // }

    // Poll for events
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. } => {
                state.game_over = true;
            }
            Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                sub.velocity = sub.jump_strength;
            }
            _ => {}
        }
    }

    // Draw everything
    let _ = background.render(&state, &mut state.canvas);
    let _ = sub.render(&state, &mut state.canvas);
    let _ = bubbles.render(&state, &mut state.canvas);
    let _ = pipes.render(&state, &mut state.canvas);

    Ok(true)
}

/// Returns true if the submarines collided against a pipe (or the ground / surface)
fn collision_detected(_sub: &Sub, _pipes: &PipeGenerator) -> bool {
    false
}

/*
    let mut next_pipe_fc = 100;
    let mut fc = 0;
    let mut high_score = 0;
    while !game_over {
        fc += 1;

        // Generate new pipe if it's time for a new one
        if fc > next_pipe_fc {
            let mut rng = rand::thread_rng();
            // next_pipe_fc = fc + rng.gen_range(20..40);
            next_pipe_fc = fc + rng.gen_range(75..200);

            // let hole_size = 200;
            let hole_size = rng.gen_range(150..250);
            let hole_offset = rng.gen_range(50..wh - hole_size - 50);

            pipes.push(Pipe{
                x: 800,
                top_offset: hole_offset as i32,
                bottom_offset: (hole_offset + hole_size) as i32,
            })
        }

        // Update the submarine
        let (_, wh) = canvas.window().size();
        if sub.y < wh as i32 {
            sub.velocity += sub.gravity;
            sub.y = sub.y + sub.velocity as i32;
        }

        // Moved the pipes
        for i in 0..pipes.len() {
            pipes[i].x -= 3;
        }

        // Remove pipes that are off-screen
        for i in (0..pipes.len()).rev() {
            let q = pipe_texture.query();
            if pipes[i].x < 0 - q.width as i32  {
                pipes.remove(i);
            }
        }

        // Check if the sub hit the top or bottom
        if sub.y >= wh as i32 || sub.y < 0 {
            game_over = true;
        }

        // Poll for events
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


        draw_background(&mut canvas, ww, wh);

        // Print submarine
        canvas.copy_ex(&sub_texture, None, Rect::new(sub.x, sub.y, 50, 45), sub.velocity as f64, None, false, false)?;

        // Print pipes
        for i in 0..pipes.len() {
            let q = pipe_texture.query();
            let qe = pipe_end_texture.query();

            // Top pipe
            canvas.copy_ex(
                &pipe_texture,
                None,
                Rect::new(pipes[i].x, 0, q.width, pipes[i].top_offset as u32),
                0.0,
                None,
                false,
                true
            )?;

            canvas.copy_ex(
                &pipe_end_texture,
                None,
                Rect::new(pipes[i].x, pipes[i].top_offset - 10, qe.width, qe.height),
                0.0,
                None,
                false,
                true
            )?;

            // Bottom pipe
            canvas.copy(
                &pipe_texture,
                None,
                Rect::new(pipes[i].x, pipes[i].bottom_offset, q.width, q.height + 50 - pipes[i].bottom_offset as u32),
            )?;

            canvas.copy(
                &pipe_end_texture,
                None,
                Rect::new(pipes[i].x, pipes[i].bottom_offset, qe.width, qe.height),
            )?;
        }

        draw_bubbles(&mut canvas, &mut bubbles, ww as i32, wh as i32, &generator, -2.0);

        let surface = font
            .render(format!("Score: {:06}   Hi-Score: {:06}", fc, high_score).as_str())
            .blended(Color::RGBA(64, 64, 64, 255))
            .map_err(|e| e.to_string())?;
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;
        canvas.copy(&texture, None, Rect::new(20, 10, 300, 30))?;

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

 */
// fn draw_background(canvas: &mut WindowCanvas, ww: u32, wh: u32) {
//     canvas.set_draw_color(Color::RGB(192, 192, 192));
//     canvas.clear();
//
//     // Print ground line
//     let y = (wh - (wh / 3)) as i32;
//     canvas.set_draw_color(Color::RGB(164, 164, 164));
//     canvas.fill_rect(Rect::new(0, y, ww, wh - y as u32));
// }

// fn draw_bubbles<'a>(canvas: &mut WindowCanvas, bubbles: &mut Vec<Bubble<'a>>, ww: i32, wh: i32, bubble_generator: &'a BubbleGenerator<'a>, speed: f32) {
//     for i in 0..bubbles.len() {
//         bubbles[i].update(speed);
//         bubbles[i].render(canvas);
//
//         if bubbles[i].finished() {
//             bubbles[i] = bubble_generator.generate(ww, wh);
//         }
//     }
// }
