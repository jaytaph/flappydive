use crate::actors::background::Background;
use crate::actors::bubble::Bubbles;
use crate::actors::pipe::Pipes;
use crate::actors::score::Score;
use crate::actors::sub::Sub;
use crate::{GameState, Renderable};
use crate::theme::Theme;

pub mod background;
pub mod bubble;
pub mod pipe;
pub mod score;
pub mod sub;

pub struct Actors<'a> {
    pub sub: Sub<'a>,
    pub bubbles: Bubbles<'a>,
    pub pipes: Pipes<'a>,
    pub background: Background<'a>,
    pub score: Score<'a>
}

impl<'a> Actors<'a> {
    pub(crate) fn switch_theme(&mut self, theme: &Theme) {
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