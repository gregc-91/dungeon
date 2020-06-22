extern crate rand;
use rand::Rng;

use crate::game::Colour;
use crate::game::Level;
use crate::game::Tile;
use crate::game::TileType;

pub struct Maze {}

impl Maze {
    pub fn new(width: usize, height: usize) -> Level {
        let mut level = Level::new(width, height);
        Maze::fill_perimeter(&mut level);
        Maze::gen_random_rooms(&mut level, 100);
        for j in (1..height-1).step_by(2) {
            for i in (1..width-1).step_by(2) {
                Maze::gen_perfect_maze(&mut level, (i,j));
            }
        }
        return level;
    }

    fn fill_perimeter(level: &mut Level) {
        // Mark all the edges as walls
        let height = level.height;
        let width = level.width;
        for i in 0 .. width {
            level[0][i] = Tile{tile_type: TileType::_Wall, colour: Colour{r:0,g:0,b:0,a:255}};
            level[height-1][i] = Tile{tile_type: TileType::_Wall, colour: Colour{r:0,g:0,b:0,a:255}};
        }
        for j in 1 .. height-1 {
            level[j][0] = Tile{tile_type: TileType::_Wall, colour: Colour{r:0,g:0,b:0,a:255}};
            level[j][width-1] = Tile{tile_type: TileType::_Wall, colour: Colour{r:0,g:0,b:0,a:255}};
        }
    }

    fn gen_perfect_maze(level: &mut Level, (i, j): (usize, usize)) {
        // Initialise the stack
        let mut stack: Vec<(usize, usize)> = Vec::new();
        stack.push((i, j));

        // Mark the current cell as floor
        level[j][i] = Tile{tile_type: TileType::_Floor, colour: Colour{r:128,g:128,b:128,a:255}};

        while !stack.is_empty() {
            if let Some((x, y)) = stack.pop() {

                // Create a vector with valid options of where to step next
                let next_cells = Maze::test_neighbours(level, (x, y));

                if next_cells.len() != 0 {
                    // Push this cell back onto the stack
                    stack.push((x, y));

                    // Randomly choose one of the options to step to next
                    let next_id = rand::thread_rng().gen_range(0, next_cells.len());
                    let step = next_cells[next_id];
                    let next_wall = ((x as isize + step.0) as usize, (y as isize + step.1) as usize);
                    let next_cell = ((x as isize + step.0*2) as usize, (y as isize + step.1*2) as usize);

                    // Turn the wall and the next cell into path
                    level[next_wall] = Tile{tile_type: TileType::_Floor, colour: Colour{r:128,g:128,b:128,a:255}};
                    level[next_cell] = Tile{tile_type: TileType::_Floor, colour: Colour{r:128,g:128,b:128,a:255}};

                    // Push the next cell onto the stack
                    stack.push(next_cell);
                }
            }
        }
    }

    fn test_neighbours(level: &mut Level, (i, j): (usize, usize)) -> Vec<(isize, isize)> {
        let mut v: Vec<(isize, isize)> = Vec::new();
        if level[j][i-1].tile_type == TileType::_Empty && level[j][i-2].tile_type == TileType::_Empty { v.push((-1, 0)) };
        if level[j][i+1].tile_type == TileType::_Empty && level[j][i+2].tile_type == TileType::_Empty { v.push(( 1, 0)) };
        if level[j-1][i].tile_type == TileType::_Empty && level[j-2][i].tile_type == TileType::_Empty { v.push((0, -1)) };
        if level[j+1][i].tile_type == TileType::_Empty && level[j+2][i].tile_type == TileType::_Empty { v.push((0,  1)) };
        return v;
    }

    fn can_place_room(level: &Level, (x,y): (usize, usize), (w, h): (usize, usize)) -> bool {
        for j in 0..h {
            for i in 0..w {
                if level[y+j][x+i].tile_type != TileType::_Empty {
                    return false;
                }
            }
        }
        true
    }

    fn place_room(level: &mut Level, (x,y): (usize, usize), (w, h): (usize, usize)) {
        for j in 0..h {
            for i in 0..w {
                level[y+j][x+i] = Tile{tile_type: TileType::_Floor, colour: Colour{r:128,g:128,b:128,a:255}};
            }
        }
    }

    fn gen_random_rooms(level: &mut Level, attempts: usize) {
        for _a in 0..attempts {
            let w = rand::thread_rng().gen_range(1, 5)*2+1;
            let h = rand::thread_rng().gen_range(1, 4)*2+1;
            let x = rand::thread_rng().gen_range(0, level.width>>1)*2+1;
            let y = rand::thread_rng().gen_range(0, level.height>>1)*2+1;

            if Maze::can_place_room(level, (x, y), (w, h)) {
                Maze::place_room(level, (x, y), (w, h));
            }
        }
    }
}