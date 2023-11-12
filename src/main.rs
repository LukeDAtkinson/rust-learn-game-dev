use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, Texture};
use std::time::Duration;

// Hack to keep speed roughly the same, even when moving at an angle
// 14^2 = 196 ~= 200 = (10^2 + 10^2)
// As it stands, diagonal movement will be slightly faster than along
// ordinal directions, but it's not very noticeable
const PLAYER_MOVEMENT_SPEED: i32 = 14;
const PLAYER_MOVEMENT_SPEED_ANGLE: i32 = 10;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Velocity {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Player {
    position: Point,
    sprite: Rect,
    facing: Direction,
    velocity: Velocity,
    current_frame: i32,
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

fn update_player(player: &mut Player) {
    if player.velocity.x != 0 && player.velocity.y != 0 {
        player.position = player.position.offset(
            player.velocity.x * PLAYER_MOVEMENT_SPEED_ANGLE,
            player.velocity.y * PLAYER_MOVEMENT_SPEED_ANGLE,
        );
    } else {
        player.position = player.position.offset(
            player.velocity.x * PLAYER_MOVEMENT_SPEED,
            player.velocity.y * PLAYER_MOVEMENT_SPEED,
        );
        if let Some(f) = velocity_to_facing(&player.velocity) {
            player.facing = f;
        }
    }
    // Only animate if the player is moving
    if player.velocity.x != 0 || player.velocity.y != 0 {
        // Cheat: using the fact that all animations are 3 frames (NOT extensible)
        player.current_frame = (player.current_frame + 1) % 3;
    }
}

fn render(
    canvas: &mut Canvas<sdl2::video::Window>,
    color: Color,
    texture: &Texture,
    player: &Player,
) -> Result<(), String> {
    canvas.set_draw_color(color);
    canvas.clear();

    let (width, height) = canvas.output_size()?;

    let (frame_width, frame_height) = player.sprite.size();

    let current_frame = Rect::new(
        player.sprite.x() + frame_width as i32 * player.current_frame,
        player.sprite.y() + frame_height as i32 * facing_to_spritesheet_row(&player.facing),
        frame_width,
        frame_height,
    );

    let screen_position = player.position + Point::new(width as i32 / 2, height as i32 / 2);

    let screen_rect = Rect::from_center(screen_position, frame_width, frame_height);
    canvas.copy(texture, current_frame, screen_rect)?;

    canvas.present();

    Ok(())
}
fn find_sdl_gl_driver() -> Option<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            return Some(index as u32);
        }
    }
    None
}
fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
    let window = video_subsystem
        .window("game tutorial", 800, 600)
        .position_centered()
        .opengl() // this line DOES NOT enable opengl, but allows you to create/get an OpenGL context from your window.
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .index(find_sdl_gl_driver().unwrap())
        .build()
        .map_err(|e| e.to_string())?;

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture("assets/bardo.png")?;

    let mut player = Player {
        position: Point::new(0, 0),
        sprite: Rect::new(0, 0, 26, 36),
        velocity: Velocity { x: 0, y: 0 },
        current_frame: 0,
        facing: Direction::Down,
    };

    let mut event_pump = sdl_context.event_pump()?;
    let mut i = 0;
    'running: loop {
        let color = Color::RGB(i, 64, 255 - i);

        // Handle Events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                Event::KeyDown {
                    keycode: Some(keycode),
                    repeat: false,
                    ..
                } => match keycode {
                    Keycode::D => player.velocity.x += 1,
                    Keycode::A => player.velocity.x -= 1,
                    Keycode::S => player.velocity.y += 1,
                    Keycode::W => player.velocity.y -= 1,
                    _ => {}
                },
                Event::KeyUp {
                    keycode: Some(keycode),
                    repeat: false,
                    ..
                } => match keycode {
                    Keycode::D => player.velocity.x -= 1,
                    Keycode::A => player.velocity.x += 1,
                    Keycode::S => player.velocity.y -= 1,
                    Keycode::W => player.velocity.y += 1,
                    _ => {}
                },
                _ => {}
            }
        }

        // Update
        i = (i + 1) % 255;
        update_player(&mut player);

        // Render
        render(&mut canvas, color, &texture, &player)?;

        // Time Management
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 20));
    }

    Ok(())
}
