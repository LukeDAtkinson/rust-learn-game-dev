use sdl2::{pixels::Color, render::Canvas};

use crate::render::Renderable;

pub(crate) struct Background {
    color: Color,
}

impl Background {
    pub(crate) fn new(color: Color) -> Self {
        Self { color }
    }

    pub(crate) fn update(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}

impl Renderable<Canvas<sdl2::video::Window>> for Background {
    fn render(&self, target: &mut Canvas<sdl2::video::Window>) -> Result<(), String> {
        target.set_draw_color(self.color);
        target.clear();
        Ok(())
    }
}
