use std::ops::{Deref, DerefMut};

use sdl2::rect::Point;
use sdl2::{
    rect::Rect,
    render::{Canvas, Texture},
};

use crate::{player::Player, render::Renderable, Direction};

pub(crate) struct SdlPlayer {
    player: Player,
    texture: Texture,
    sprite: Rect,
}

impl SdlPlayer {
    pub(crate) fn new(player: Player, texture: Texture, sprite: Rect) -> Self {
        Self {
            player,
            texture,
            sprite,
        }
    }

    pub(crate) fn update(mut self) -> SdlPlayer {
        self.player = self.player.update();
        self
    }
}

impl Renderable<Canvas<sdl2::video::Window>> for SdlPlayer {
    fn render(&self, target: &mut Canvas<sdl2::video::Window>) -> Result<(), String> {
        let (width, height) = target.output_size()?;
        let (frame_width, frame_height) = self.sprite.size();
        let current_frame = Rect::new(
            self.sprite.x() + frame_width as i32 * self.current_frame(),
            self.sprite.y() + frame_height as i32 * facing_to_spritesheet_row(&self.facing()),
            frame_width,
            frame_height,
        );
        let screen_position = Point::new(
            width as i32 / 2 + self.position().x as i32,
            height as i32 / 2 + self.position().y as i32,
        );
        let screen_rect = Rect::from_center(screen_position, frame_width, frame_height);
        target.copy(&self.texture, current_frame, screen_rect)?;
        Ok(())
    }
}

impl Deref for SdlPlayer {
    type Target = Player;

    fn deref(&self) -> &Self::Target {
        &self.player
    }
}
impl DerefMut for SdlPlayer {
    fn deref_mut(&mut self) -> &mut Player {
        &mut self.player
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
