use sdl2::{
    rect::{Point, Rect},
    render::{Canvas, Texture},
};

use crate::{maths::Vec2, render::Renderable, Direction};

const MAX_PLAYER_MOVEMENT_SPEED: f64 = 7.0;

pub(crate) struct Player<'a> {
    position: Vec2,
    sprite: Rect,
    facing: Direction,
    velocity: Vec2,
    acceleration: Vec2,
    current_frame: i32,
    texture: Texture<'a>,
}

impl<'a> Renderable<Canvas<sdl2::video::Window>> for Player<'a> {
    fn render(&self, canvas: &mut Canvas<sdl2::video::Window>) -> Result<(), String> {
        let (width, height) = canvas.output_size()?;
        let (frame_width, frame_height) = self.sprite.size();
        let current_frame = Rect::new(
            self.sprite.x() + frame_width as i32 * self.current_frame,
            self.sprite.y() + frame_height as i32 * facing_to_spritesheet_row(&self.facing),
            frame_width,
            frame_height,
        );
        let screen_position = Point::new(width as i32 / 2, height as i32 / 2) + self.position;
        let screen_rect = Rect::from_center(screen_position, frame_width, frame_height);
        canvas.copy(&self.texture, current_frame, screen_rect)?;
        Ok(())
    }
}

impl<'a> Player<'a> {
    pub(crate) fn new(
        position: Vec2,
        sprite: Rect,
        facing: Direction,
        velocity: Vec2,
        acceleration: Vec2,
        current_frame: i32,
        texture: Texture<'a>,
    ) -> Self {
        Self {
            position,
            sprite,
            facing,
            velocity,
            acceleration,
            current_frame,
            texture,
        }
    }

    pub(crate) fn update(&mut self) {
        self.velocity = self.velocity + self.acceleration;
        if self.velocity.magnitude() > MAX_PLAYER_MOVEMENT_SPEED {
            self.velocity = MAX_PLAYER_MOVEMENT_SPEED * self.velocity.normalize()
        }

        if !self.velocity.near_zero() {
            self.position = self.position + self.velocity;
        }
        if let Some(f) = velocity_to_facing(&self.velocity) {
            self.facing = f;
        }

        // Only animate if the player is moving
        if !self.velocity.near_zero() {
            // Cheat: using the fact that all animations are 3 frames (NOT extensible)
            self.current_frame = (self.current_frame + 1) % 3;
        }
    }

    pub(crate) fn set_accelerating(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => self.acceleration.y = -1.0,
            Direction::Down => self.acceleration.y = 1.0,
            Direction::Left => self.acceleration.x = -1.0,
            Direction::Right => self.acceleration.x = 1.0,
        }
    }
    pub(crate) fn stop_accelerating(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => self.acceleration.y = 0.0,
            Direction::Down => self.acceleration.y = 0.0,
            Direction::Left => self.acceleration.x = 0.0,
            Direction::Right => self.acceleration.x = 0.0,
        }
    }
}

fn velocity_to_facing(v: &Vec2) -> Option<Direction> {
    // We only change facing if we are moving in one specific direction
    // Otherwise, we will keep the existing facing
    if v.near_zero() {
        return None;
    }
    let (x, y) = (v.x, v.y);
    if x.abs() > y.abs() {
        if x > 0.0 {
            return Some(Direction::Right);
        } else {
            return Some(Direction::Left);
        }
    }
    if y > 0.0 {
        Some(Direction::Down)
    } else {
        Some(Direction::Up)
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
