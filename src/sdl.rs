use crate::render::Renderer;
use sdl2::{
    pixels::Color,
    rect::{Point, Rect},
    render::Canvas,
    video,
};

use crate::{player::Player, render::Renderable, Direction};

pub(crate) struct SdlRenderer {}

impl Renderer<Canvas<video::Window>> for SdlRenderer {
    fn render(
        canvas: &mut Canvas<video::Window>,
        renderables: &Vec<&Box<dyn Renderable<Canvas<video::Window>> + '_>>,
    ) -> std::result::Result<(), std::string::String> {
        canvas.clear();

        for renderable in renderables {
            renderable.render(canvas)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub(crate) struct SdlBackground {
    pub(crate) color: Color,
}

impl Renderable<Canvas<video::Window>> for SdlBackground {
    fn render(&self, target: &mut Canvas<video::Window>) -> Result<(), String> {
        target.set_draw_color(self.color);
        Ok(())
    }
}

pub(crate) fn render(
    canvas: &mut Canvas<video::Window>,
    color: Color,
    player: &Player,
) -> Result<(), String> {
    canvas.set_draw_color(color);
    canvas.clear();

    player.render(canvas)?;

    canvas.present();
    Ok(())
}

impl<'a> Renderable<Canvas<video::Window>> for Player<'a> {
    fn render(&self, canvas: &mut Canvas<video::Window>) -> Result<(), String> {
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
