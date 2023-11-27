use sdl2::{
    pixels::Color,
    rect::{Point, Rect},
    render::Canvas,
};

use crate::{
    player::Player,
    render::{Renderable, Renderer},
    Direction,
};

pub(crate) struct SdlRenderer;

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

impl Renderer<Canvas<sdl2::video::Window>> for SdlRenderer {
    fn render(
        &self,
        target: &mut Canvas<sdl2::video::Window>,
        renderables: &[&dyn Renderable<Canvas<sdl2::video::Window>>],
    ) -> Result<(), String> {
        for r in renderables {
            r.render(target)?
        }
        target.present();
        Ok(())
    }
}

impl Renderable<Canvas<sdl2::video::Window>> for Player {
    fn render(&self, canvas: &mut Canvas<sdl2::video::Window>) -> Result<(), String> {
        let (width, height) = canvas.output_size()?;
        let (frame_width, frame_height) = self.sprite().size();
        let current_frame = Rect::new(
            self.sprite().x() + frame_width as i32 * self.current_frame(),
            self.sprite().y() + frame_height as i32 * facing_to_spritesheet_row(&self.facing()),
            frame_width,
            frame_height,
        );
        let screen_position = Point::new(width as i32 / 2, height as i32 / 2) + self.position();
        let screen_rect = Rect::from_center(screen_position, frame_width, frame_height);
        canvas.copy(self.texture(), current_frame, screen_rect)?;
        Ok(())
    }
}

fn facing_to_spritesheet_row(direction: &Direction) -> i32 {
    match direction {
        Direction::Up => 3,
        Direction::Down => 0,
        Direction::Left => 1,
        Direction::Right => 2,
    }
}
