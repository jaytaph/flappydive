mod bubble;
mod sub;
mod pipe;
mod background;
mod theme;

extern crate sdl2;

use std::process::exit;
use std::time::Duration;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::ttf::Sdl2TtfContext;
use crate::background::Background;
use crate::bubble::Bubbles;
use crate::pipe::Pipes;
use crate::sub::Sub;
use crate::theme::{Theme, ThemeSwitcher};

// Number of bubbles on the screen at one time
pub const MAX_BUBBLES: usize = 15;


/// Game state
struct GameState {
    /// True if the game has started. False if not started, or has ended
    game_started: bool,
    /// True if the current game is over
    game_over: bool,
    /// Current frame counter
    fc: i64,
    /// Highest frame counter score encountered
    high_score: i64,
    /// Speed of the current game
    x_speed: i32,
    // Current window height
    window_height: u32,
    // Current window width
    window_width: u32,
    // Theme switcher
    theme: ThemeSwitcher,
}

/// A renderable is something that can be rendered onto screen and has its own update functionality
trait Renderable {
    fn render(&self, state: &GameState, canvas: &mut WindowCanvas) -> Result<(), String>;
    fn update(&mut self, state: &GameState);
    fn switch_theme(&mut self, theme: &Theme);
}

pub struct TTF<'a> {
    font: sdl2::ttf::Font<'a, 'static>,
}

impl<'a> TTF<'a> {
    pub fn new(context: &'a Sdl2TtfContext) -> Self {
        let mut font = context.load_font("images/Lato-Regular.ttf", 128).unwrap();
        font.set_style(sdl2::ttf::FontStyle::BOLD);

        Self { font }
    }
}

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("FlappyDive", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let context = sdl2::ttf::init().unwrap();
    let ttf = TTF::new(&context);

    // let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let texture_creator = canvas.texture_creator();
    let (w, h) = canvas.output_size().unwrap();

    let mut state = GameState {
        game_started: false,
        game_over: false,
        fc: 0,
        high_score: 0,
        x_speed: 3,
        window_height: h,
        window_width: w,
        theme: ThemeSwitcher::new(),
    };

    // Create all renderables for the game
    let mut background = Background::new(&state, &ttf, &texture_creator);
    let mut sub = Sub::new(100, 100, &texture_creator);
    let mut bubbles = Bubbles::new(MAX_BUBBLES, &texture_creator, w, h);
    let mut pipes = Pipes::new(&texture_creator);

    // Initial theme switch to correct all colors
    background.switch_theme(&state.theme.current());
    sub.switch_theme(&state.theme.current());
    bubbles.switch_theme(&state.theme.current());
    pipes.switch_theme(&state.theme.current());


    let mut first_run = true;
    loop {
        let mut event_pump = sdl_context.event_pump()?;

        // Do pregame
        do_pregame(&mut state, &mut canvas, &ttf, first_run, &mut event_pump, &mut background, &mut sub, &mut bubbles)?;
        first_run = false;

        // run a game
        do_game(&mut state, &mut canvas, &mut event_pump, &mut background, &mut sub, &mut bubbles, &mut pipes)?;

        // Update high score and reinitialize game
        if state.fc > state.high_score {
            state.high_score = state.fc;
        }

        state.fc = 0;
        state.game_started = false;
        state.game_over = false;
    }
}

// Returns Ok(true) when the game can begin. Returns ok(false) when we want to quit
fn do_pregame(state: &mut GameState, canvas: &mut WindowCanvas, ttf: &TTF, first_run: bool, event_pump: &mut sdl2::EventPump, background: &mut Background, sub: &mut Sub, bubbles: &mut Bubbles) -> Result<bool, String> {
    let theme = state.theme.current();

    // Create message texture
    let s = if first_run { "Press <space> to begin" } else { "You sunk. Press <space> to try again" };

    let surface = ttf.font
        .render(s)
        .blended(Color::RGBA(theme.text.0, theme.text.1, theme.text.2, 255))
        .map_err(|e| e.to_string())?;

    let texture_creator = canvas.texture_creator();
    let title_texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;

    loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    exit(0);
                }
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    state.game_started = true;
                    return Ok(true);
                }
                Event::KeyDown { keycode: Some(Keycode::T), .. } => {
                    state.theme.next();

                    background.switch_theme(&state.theme.current());
                    sub.switch_theme(&state.theme.current());
                    bubbles.switch_theme(&state.theme.current());
                }
                _ => {}
            }
        }

        // Render stuff
        let _ = background.render(&state, canvas);
        let _ = sub.render(&state, canvas);
        let _ = bubbles.render(&state, canvas);

        canvas.copy(&title_texture, None, Rect::new(300, 300, 400, 60))?;
        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn do_game(state: &mut GameState, canvas: &mut WindowCanvas, event_pump: &mut sdl2::EventPump, background: &mut Background, sub: &mut Sub, bubbles: &mut Bubbles, pipes: &mut Pipes) -> Result<bool, String> {
    while !state.game_over {
        state.fc += 1;

        // Poll for events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    exit(0);
                }
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    sub.velocity = sub.jump_strength;
                }
                Event::KeyDown { keycode: Some(Keycode::T), .. } => {
                    state.theme.next();

                    background.switch_theme(&state.theme.current());
                    sub.switch_theme(&state.theme.current());
                    bubbles.switch_theme(&state.theme.current());
                    pipes.switch_theme(&state.theme.current());
                }
                _ => {}
            }
        }

        pipes.update(state);
        background.update(state);
        sub.update(state);
        bubbles.update(state);

        if collision_detected(sub, pipes) {
            state.game_over = true;
            return Ok(true)
        }

        // Draw everything
        let _ = background.render(&state, canvas);
        let _ = pipes.render(&state, canvas);
        let _ = sub.render(&state, canvas);
        let _ = bubbles.render(&state, canvas);

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(true)
}

/// Returns true if the submarines collided against a pipe (or the ground / surface)
fn collision_detected(_sub: &Sub, _pipes: &Pipes) -> bool {
    false
}