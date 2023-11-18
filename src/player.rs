use sdl2::{
    rect::{Point, Rect},
    render::{Canvas, Texture},
};

use crate::{Direction, Velocity};

const PLAYER_MOVEMENT_SPEED: i32 = 7;
const PLAYER_MOVEMENT_SPEED_ANGLE: i32 = 5;

pub(crate) struct Player<'a> {
    position: Point,
    sprite: Rect,
    facing: Direction,
    pub(crate) velocity: Velocity,
    current_frame: i32,
    texture: Texture<'a>,
}

impl<'a> Player<'a> {
    pub(crate) fn new(
        position: Point,
        sprite: Rect,
        facing: Direction,
        velocity: Velocity,
        current_frame: i32,
        texture: Texture<'a>,
    ) -> Self {
        Self {
            position,
            sprite,
            facing,
            velocity,
            current_frame,
            texture,
        }
    }

    pub(crate) fn update(&mut self) {
        if self.velocity.x != 0 && self.velocity.y != 0 {
            self.position = self.position.offset(
                self.velocity.x * PLAYER_MOVEMENT_SPEED_ANGLE,
                self.velocity.y * PLAYER_MOVEMENT_SPEED_ANGLE,
            );
        } else {
            self.position = self.position.offset(
                self.velocity.x * PLAYER_MOVEMENT_SPEED,
                self.velocity.y * PLAYER_MOVEMENT_SPEED,
            );
            if let Some(f) = velocity_to_facing(&self.velocity) {
                self.facing = f;
            }
        }
        // Only animate if the player is moving
        if self.velocity.x != 0 || self.velocity.y != 0 {
            // Cheat: using the fact that all animations are 3 frames (NOT extensible)
            self.current_frame = (self.current_frame + 1) % 3;
        }
    }

    pub(crate) fn render(&self, canvas: &mut Canvas<sdl2::video::Window>) -> Result<(), String> {
        let (width, height) = canvas.output_size()?;
        let (frame_width, frame_height) = self.sprite.size();
        let current_frame = Rect::new(
            self.sprite.x() + frame_width as i32 * self.current_frame,
            self.sprite.y() + frame_height as i32 * facing_to_spritesheet_row(&self.facing),
            frame_width,
            frame_height,
        );
        let screen_position = self.position + Point::new(width as i32 / 2, height as i32 / 2);
        let screen_rect = Rect::from_center(screen_position, frame_width, frame_height);
        canvas.copy(&self.texture, current_frame, screen_rect)?;
        Ok(())
    }
}

    fn velocity_to_facing(velocity: &Velocity) -> Option<Direction> {
        // We only change facing if we are moving in one specific direction
        // Otherwise, we will keep the existing facing
        match velocity {
            Velocity { x: -1, y: 0 } => Some(Direction::Left),
            Velocity { x: 1, y: 0 } => Some(Direction::Right),
            Velocity { x: 0, y: 1 } => Some(Direction::Down),
            Velocity { x: 0, y: -1 } => Some(Direction::Up),
            _ => None,
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
