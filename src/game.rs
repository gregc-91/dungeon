extern crate sdl2;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use crate::actor::Actor;
use crate::hero::Hero;
use crate::monster::Monster;
use crate::action::*;
use crate::maze::Maze;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TileType {
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
pub struct Tile {
    pub tile_type: TileType,
    pub colour: Colour
}

impl Tile {
    fn can_walk(&self) -> bool {
        match self.tile_type {
            TileType::_Wall => false,
            _ => true
        }
    }
}

pub struct Level {
    pub width: usize,
    pub height: usize,
    grid: Vec<Tile>
}

impl Level {
    pub fn new(width: usize, height: usize) -> Level {
        let c = Colour{r:64, g:64, b:64, a:255};
        let v = vec!(Tile{tile_type: TileType::_Empty, colour: c}; width*height);
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

impl std::ops::Index<(usize, usize)> for Level {
    type Output = Tile;

    fn index(&self, (i, j): (usize, usize)) -> &Tile {
        &self.grid[j*self.width+i]
    }   
}

impl std::ops::IndexMut<(usize, usize)> for Level {
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut Tile {
        &mut self.grid[j*self.width+i]
    }   
}

pub struct Game {
    canvas: Canvas<Window>,
    level: Level,
    monsters: Vec<Monster>,
    pub hero: Hero
}

impl Game {
    pub fn new(canvas: Canvas<Window>) -> Game {
        Game {
            canvas,
            level: Maze::new(49, 37),
            monsters: Vec::new(),
            hero: Hero::new()
        }
    }

    pub fn update(&mut self) {
        let action = self.hero.get_action();
        action.perform(&mut self.hero);
        self.hero.set_next_action(Box::new(NullAction{}));
    }

    pub fn draw(&mut self) {
        self.canvas.set_draw_color(Color::RGB(100, 100, 100));
        self.canvas.clear();

        for j in 0..self.level.height {
            for i in 0..self.level.width {
                self.canvas.set_draw_color(Color::RGB(
                    self.level[j][i].colour.r,
                    self.level[j][i].colour.g,
                    self.level[j][i].colour.b));
                let _result = self.canvas.fill_rect(Rect::new((i*16+1) as i32, (j*16+1) as i32, 14, 14));
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
            Rect::new(pos.x*16+1, pos.y*16+1, 14, 14));
    }
}
