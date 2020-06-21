extern crate sdl2;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

mod action;
mod actor;
mod drawable;
mod game;
mod hero;
mod input;
mod math;
mod maze;
mod monster;

use crate::action::WalkAction;
use crate::game::Game;
use crate::input::Input;
use crate::math::Vec2i;


fn main() {
    println!("Hello, world!");

    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let window = video_subsystem
        .window("Game", 784, 592)
        .resizable()
        .build()
        .unwrap();

    let canvas : Canvas<Window> = window
        .into_canvas()
        .present_vsync()
        .build()
        .unwrap();

    let mut game: Game = Game::new(canvas);

    fn handle_input(game: &mut Game, input: Input) {
        match input {
            Input::North => game.hero.set_next_action(Box::new(WalkAction{offset: Vec2i{x: 0, y: -1}})),
            Input::South => game.hero.set_next_action(Box::new(WalkAction{offset: Vec2i{x: 0, y: 1}})),
            Input::East => game.hero.set_next_action(Box::new(WalkAction{offset: Vec2i{x: 1, y: 0}})),
            Input::West => game.hero.set_next_action(Box::new(WalkAction{offset: Vec2i{x: -1, y: 0}})),
        }
    }

    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => break 'main,
                Event::KeyDown { keycode: Some(Keycode::Up), ..} => handle_input(&mut game, Input::North),
                Event::KeyDown { keycode: Some(Keycode::Down), ..} => handle_input(&mut game, Input::South),
                Event::KeyDown { keycode: Some(Keycode::Left), ..} => handle_input(&mut game, Input::West),
                Event::KeyDown { keycode: Some(Keycode::Right), ..} => handle_input(&mut game, Input::East),
                _ => {}
            }
        }

        game.update();
        game.draw();

        std::thread::sleep(Duration::new(0, 1000000000u32 / 60));
    }
}

