mod theme;
mod actors;

extern crate sdl2;

use std::process::exit;
use std::time::Duration;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::ttf::Sdl2TtfContext;
use crate::actors::background::Background;
use crate::actors::bubble::Bubbles;
use crate::actors::pipe::Pipes;
use crate::actors::score::Score;
use crate::actors::sub::Sub;
use crate::theme::{Theme, ThemeSwitcher};

// Number of bubbles on the screen at one time
pub const MAX_BUBBLES: usize = 15;


struct Actors<'a> {
    sub: Sub<'a>,
    bubbles: Bubbles<'a>,
    pipes: Pipes<'a>,
    background: Background<'a>,
    score: Score<'a>
}

impl<'a> Actors<'a> {
    pub(crate) fn switch_theme(&mut self, theme: &&Theme) {
        self.background.switch_theme(theme);
        self.sub.switch_theme(theme);
        self.bubbles.switch_theme(theme);
        self.pipes.switch_theme(theme);
        self.score.switch_theme(theme);
    }

    pub(crate) fn reset(&mut self) {
        self.background.reset();
        self.sub.reset();
        self.bubbles.reset();
        self.pipes.reset();
        self.score.reset();
    }

    pub(crate) fn update(&mut self, state: &GameState) {
        self.background.update(state);
        self.sub.update(state);
        self.bubbles.update(state);
        self.pipes.update(state);
        self.score.update(state);
    }
}

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
    // Runcount
    run_count: i32
}

/// A renderable is something that can be rendered onto screen and has its own update functionality
trait Renderable {
    /// Render the actual object(s) on a canvas
    fn render(&self, state: &GameState, canvas: &mut WindowCanvas) -> Result<(), String>;
    /// Update any internal state of the object (movement, etc.)
    fn update(&mut self, state: &GameState);
    /// Switch theme for this object (if applicable)
    fn switch_theme(&mut self, theme: &Theme);
    /// Reset the object to its initial state for a new game
    fn reset(&mut self);
}

/// It's hard to work with fonts due to the lifetime issues, so we wrap it in a struct
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
        run_count: 0
    };

    // Create all renderables for the game
    let mut actors = Actors {
        sub: Sub::new(100, 100, &texture_creator),
        bubbles: Bubbles::new(MAX_BUBBLES, &texture_creator, w, h),
        pipes: Pipes::new(&texture_creator),
        background: Background::new(&state, &texture_creator),
        score: Score::new(&ttf)
    };

    actors.switch_theme(&state.theme.current());


    loop {
        let mut event_pump = sdl_context.event_pump()?;

        // Do pregame
        do_pregame(&mut state, &mut canvas, &ttf, &mut event_pump, &mut actors)?;

        actors.reset();

        // run a game
        do_game(&mut state, &mut canvas, &mut event_pump, &mut actors)?;

        // Update high score and reinitialize game
        if state.fc > state.high_score {
            state.high_score = state.fc;
        }

        state.run_count += 1;
        state.fc = 0;
        state.game_started = false;
        state.game_over = false;
    }
}

// Returns Ok(true) when the game can begin. Returns ok(false) when we want to quit
fn do_pregame(state: &mut GameState, canvas: &mut WindowCanvas, ttf: &TTF, event_pump: &mut sdl2::EventPump, actors: &mut Actors) -> Result<bool, String> {
    let theme = state.theme.current();


    actors.sub.reset();

    // Create message texture
    let s = if state.run_count == 0 { "Press <space> to begin" } else { "You sunk. Press <space> to try again" };

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

                    actors.background.switch_theme(&state.theme.current());
                    actors.sub.switch_theme(&state.theme.current());
                    actors.bubbles.switch_theme(&state.theme.current());
                }
                _ => {}
            }
        }

        // Update stuff
        actors.update(&state);

        // Render stuff
        actors.background.render(&state, canvas)?;
        actors.sub.render(&state, canvas)?;
        actors.bubbles.render(&state, canvas)?;

        canvas.copy(&title_texture, None, Rect::new(250, 100, 300, 60))?;
        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn do_game(state: &mut GameState, canvas: &mut WindowCanvas, event_pump: &mut sdl2::EventPump, actors: &mut Actors) -> Result<bool, String> {
    while !state.game_over {
        state.fc += 1;

        // Poll for events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    exit(0);
                }
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    actors.sub.velocity = actors.sub.jump_strength;
                }
                Event::KeyDown { keycode: Some(Keycode::C), .. } => {
                    state.game_over = true;
                    state.game_started = false;
                }
                Event::KeyDown { keycode: Some(Keycode::T), .. } => {
                    state.theme.next();

                    actors.switch_theme(&state.theme.current());
                }
                _ => {}
            }
        }

        actors.update(state);

        if collision_detected(&actors.sub, &actors.pipes) {
            state.game_over = true;
            return Ok(true)
        }

        // Draw everything
        actors.background.render(&state, canvas)?;
        actors.pipes.render(&state, canvas)?;
        actors.sub.render(&state, canvas)?;
        actors.bubbles.render(&state, canvas)?;
        actors.score.render(&state, canvas)?;

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(true)
}

/// Returns true if the submarines collided against a pipe (or the ground / surface)
fn collision_detected(_sub: &Sub, _pipes: &Pipes) -> bool {
    false
}