extern crate sdl2;
use sdl2::render::Canvas;
use sdl2::video::Window;

mod action;
mod actor;
mod drawable;
mod game;
mod hero;
mod math;
mod monster;

use crate::game::Game;

fn main() {
    println!("Hello, world!");

    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let window = video_subsystem
        .window("Game", 800, 600)
        .resizable()
        .build()
        .unwrap();

    let canvas : Canvas<Window> = window
        .into_canvas()
        .present_vsync()
        .build()
        .unwrap();

    let mut game: Game = Game::new(canvas);

    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => break 'main,
                _ => {},
            }
        }

        game.update();
        game.draw();
    }
}

