use sdl2::{rect::Rect, render::Texture};

use crate::{maths::Vec2, Direction};

const MAX_PLAYER_MOVEMENT_SPEED: f64 = 7.0;

pub(crate) struct Player {
    position: Vec2,
    sprite: Rect,
    facing: Direction,
    velocity: Vec2,
    acceleration: Vec2,
    current_frame: i32,
    texture: Texture,
}

impl Player {
    pub(crate) fn new(
        position: Vec2,
        sprite: Rect,
        facing: Direction,
        velocity: Vec2,
        acceleration: Vec2,
        current_frame: i32,
        texture: Texture,
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

    pub(crate) fn update(mut self) -> Self {
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
        self
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
    pub(crate) fn facing(&self) -> Direction {
        self.facing
    }
    pub(crate) fn sprite(&self) -> Rect {
        self.sprite
    }

    pub(crate) fn current_frame(&self) -> i32 {
        self.current_frame
    }

    pub(crate) fn position(&self) -> Vec2 {
        self.position
    }

    pub(crate) fn texture(&self) -> &Texture {
        &self.texture
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
