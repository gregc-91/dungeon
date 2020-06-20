extern crate sdl2;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use crate::actor::Actor;
use crate::hero::Hero;
use crate::monster::Monster;

#[derive(Debug, Copy, Clone)]
enum TileType {
    _Empty,
    _Floor,
    _Wall,
    _Door,
    _Ladder,
    _Pit,
    _Fire,
    _Ice
}

#[derive(Debug, Copy, Clone)]
pub struct Colour {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8
}

#[derive(Debug, Copy, Clone)]
struct Tile {
    tile_type: TileType,
    colour: Colour
}

impl Tile {
    fn can_walk(&self) -> bool {
        match self.tile_type {
            TileType::_Wall => false,
            _ => true
        }
    }
}

struct Level {
    pub width: usize,
    pub height: usize,
    grid: Vec<Tile>
}

impl Level {
    fn new(width: usize, height: usize) -> Level {
        let c = Colour{r:128, g:128, b:128, a:255};
        let v = vec!(Tile{tile_type: TileType::_Floor, colour: c}; width*height);
        Level {
            width,
            height,
            grid: v
        }
    }
}

impl std::ops::Index<usize> for Level {
    type Output = [Tile];

    fn index(&self, row: usize) -> &[Tile] {
        let start = self.width*row;
        &self.grid[start .. start+self.width]
    }
}

impl std::ops::IndexMut<usize> for Level {
    fn index_mut(&mut self, row: usize) -> &mut [Tile] {
        let start = self.width*row;
        &mut self.grid[start .. start+self.width]
    }
}

pub struct Game {
    canvas: Canvas<Window>,
    level: Level,
    actors: Vec<Monster>,
    hero: Hero
}

impl Game {
    pub fn new(canvas: Canvas<Window>) -> Game {
        Game {
            canvas,
            level: Level::new(80, 60),
            actors: Vec::new(),
            hero: Hero::new()
        }
    }

    pub fn update(&mut self) {

    }

    pub fn draw(&mut self) {
        self.canvas.set_draw_color(Color::RGB(100, 100, 100));
        self.canvas.clear();

        self.canvas.set_draw_color(Color::RGB(128, 128, 128));
        for j in 0..self.level.height {
            for i in 0..self.level.width {
                let _result = self.canvas.fill_rect(Rect::new((i*10+1) as i32, (j*10+1) as i32, 8, 8));
            }
        }

        Game::draw_object(&mut self.canvas, &self.hero);
        
        self.canvas.present();

    }

    fn draw_object(canvas: &mut Canvas<Window>, actor: &dyn Actor) {
        let drawable = actor.get_drawable();
        let colour = drawable.colour;
        let pos = drawable.pos;

        canvas.set_draw_color(
            Color::RGBA(colour.r, colour.g, colour.b, colour.a));

        let _result = canvas.fill_rect(
            Rect::new(pos.x*10+1, pos.y*10+1, 8, 8));
    }
}
