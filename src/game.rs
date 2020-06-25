extern crate sdl2;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use crate::actor::Actor;
use crate::hero::Hero;
use crate::math::Vec2i;
use crate::monster::Monster;
use crate::action::*;
use crate::maze::Maze;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Direction {
    North,
    South,
    East,
    West
}

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

impl Colour {
    pub fn new(r: u8, g:u8, b:u8, a:u8) -> Colour {
        Colour {r, g, b, a}
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Tile {
    pub tile_type: TileType,
    pub colour: Colour
}

impl Tile {
    pub fn can_walk(&self) -> bool {
        match self.tile_type {
            TileType::_Wall => false,
            TileType::_Empty => false,
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

    pub fn can_walk(&self, pos: Vec2i) -> bool {
        if pos.x < 0 || pos.x >= self.width as i32 { return false };
        if pos.y < 0 || pos.y >= self.height as i32 { return false };

        self[pos.y as usize][pos.x as usize].can_walk()
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

pub struct Game<'a> {
    canvas: Canvas<Window>,
    pub level: Level,
    pub monsters: Vec<Monster<'a>>,
    pub hero: Hero
}

impl <'a> Game<'a> {
    pub fn new(canvas: Canvas<Window>) -> Game<'a> {
        let l: Level = Maze::new(49, 37);
        let hero_pos = Maze::find_spawn(&l);
        let mut m = Vec::new();

        for i in 0..10 {
            let p = Maze::find_spawn(&l);
            let g = Monster::goblin(p);
            m.push(g);
        }

        let mut game = Game {
            canvas,
            level: l,
            monsters: m,
            hero: Hero::new(hero_pos)
        };

        return game;
    }

    pub fn update(&mut self) {
        let action = self.hero.get_action();
        action.perform(&self.level, &mut self.hero);
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

        for m in &self.monsters {
            Game::draw_object(&mut self.canvas, m);
        }
        
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
