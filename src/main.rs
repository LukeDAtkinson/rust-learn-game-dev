mod maths;
mod player;
mod render;
mod render_sdl;

use maths::Vec2;
use player::Player;
use render::Renderer;
use render_sdl::background::Background;
use render_sdl::renderer::SdlRenderer;
use render_sdl::sdl_player::SdlPlayer;
use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use std::time::Duration;

// Hack to keep speed roughly the same, even when moving at an angle
// 14^2 = 196 ~= 200 = (10^2 + 10^2)
// As it stands, diagonal movement will be slightly faster than along
// ordinal directions, but it's not very noticeable

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
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

    let player = Player::new(Vec2::zero(), Direction::Down, Vec2::zero(), Vec2::zero(), 0);

    let mut sdl_player = SdlPlayer::new(player, texture, Rect::new(0, 0, 26, 36));

    let renderer = SdlRenderer {};
    let mut background = Background::new(Color::RGB(0, 64, 255));
    let mut event_pump = sdl_context.event_pump()?;
    let mut i = 0;
    'running: loop {
        let color = Color::RGB(i, 64, 255 - i);

        background = background.update(color);

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
                    Keycode::D => sdl_player.set_accelerating(&Direction::Right),
                    Keycode::A => sdl_player.set_accelerating(&Direction::Left),
                    Keycode::S => sdl_player.set_accelerating(&Direction::Down),
                    Keycode::W => sdl_player.set_accelerating(&Direction::Up),
                    _ => {}
                },
                Event::KeyUp {
                    keycode: Some(keycode),
                    repeat: false,
                    ..
                } => match keycode {
                    Keycode::D => sdl_player.stop_accelerating(&Direction::Right),
                    Keycode::A => sdl_player.stop_accelerating(&Direction::Left),
                    Keycode::S => sdl_player.stop_accelerating(&Direction::Down),
                    Keycode::W => sdl_player.stop_accelerating(&Direction::Up),
                    _ => {}
                },
                _ => {}
            }
        }

        // Update
        i = (i + 1) % 255;
        sdl_player = sdl_player.update();

        // Render
        renderer.render(&mut canvas, &[&background, &sdl_player])?;

        // Time Management
        // TODO: Learn how to handle time steps properly
        // https://gafferongames.com/post/fix_your_timestep/
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 20));
    }

    Ok(())
}
