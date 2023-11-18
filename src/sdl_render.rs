use sdl2::{
    pixels::Color,
    rect::{Point, Rect},
    render::Canvas,
};

use crate::{player::Player, Direction};

pub(crate) trait Renderable<T> {
    fn render(&self, target: &mut T) -> Result<(), String>;
}

pub(crate) fn render(
    canvas: &mut Canvas<sdl2::video::Window>,
    color: Color,
    player: &Player,
) -> Result<(), String> {
    canvas.set_draw_color(color);
    canvas.clear();

    player.render(canvas)?;

    canvas.present();
    Ok(())
}

impl<'a> Renderable<Canvas<sdl2::video::Window>> for Player<'a> {
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
