extern crate rand;
use rand::Rng;
use std::collections::HashSet;
use std::collections::HashMap;


use crate::game::Colour;
use crate::game::Level;
use crate::game::Tile;
use crate::game::TileType;

const EMPTY: i32 = -2;
const WALL: i32 = -1;

struct Regions {
    width: usize,
    height: usize,
    grid: Vec<i32>
}

impl Regions {
    pub fn new(width: usize, height: usize) -> Regions {
        let v = vec!(EMPTY; width*height);
        Regions { width, height, grid: v }
    }
}

impl std::ops::Index<usize> for Regions {
    type Output = [i32];

    fn index(&self, row: usize) -> &[i32] {
        let start = self.width*row;
        &self.grid[start .. start+self.width]
    }
}

impl std::ops::IndexMut<usize> for Regions {
    fn index_mut(&mut self, row: usize) -> &mut [i32] {
        let start = self.width*row;
        &mut self.grid[start .. start+self.width]
    }
}

pub struct Maze {}

impl Maze {
    pub fn new(width: usize, height: usize) -> Level {
        let mut level = Level::new(width, height);
        let mut regions = Regions::new(width, height);
        let mut num_regions: i32 = 100;

        Maze::fill_perimeter(&mut regions);
        Maze::gen_random_rooms(&mut regions, num_regions);
        for j in (1..height-1).step_by(2) {
            for i in (1..width-1).step_by(2) {
                Maze::gen_perfect_maze(&mut regions, (i,j), num_regions);
                num_regions += 1;
            }
        }
        Maze::connect_regions(&mut regions);
        Maze::remove_deadends(&mut regions);
        Maze::regions_to_tiles(&regions, &mut level);
        return level;
    }

    // Mark all the edges as walls
    fn fill_perimeter(regions: &mut Regions) {
        let height = regions.height;
        let width = regions.width;
        for i in 0 .. width {
            regions[0][i] = WALL;
            regions[height-1][i] = WALL;
        }
        for j in 1 .. height-1 {
            regions[j][0] = WALL;
            regions[j][width-1] = WALL;
        }
    }

    fn can_place_room(regions: &Regions, (x,y): (usize, usize), (w, h): (usize, usize)) -> bool {
        for j in 0..h {
            for i in 0..w {
                if regions[y+j][x+i] != EMPTY {
                    return false;
                }
            }
        }
        true
    }

    fn place_chamfered_room(
        regions: &mut Regions, (x,y): (usize, usize), (w, h): (usize, usize), chamfer: usize, region: i32) {

        for j in 0..h {
            for i in 0..w {
                if (i as i32 + j as i32) < chamfer as i32 { continue; }
                if (i as i32 - j as i32) > w as i32 - chamfer as i32 -1 { continue; }
                if (j as i32 - i as i32) > h as i32 - chamfer as i32 -1 { continue; }
                if (i as i32 + j as i32) > w as i32 + h as i32 - chamfer as i32 - 2 { continue; }

                regions[y+j][x+i] = region;
            }
        }   
    }

    // Uses a fixed number of attempts to try and place rooms
    fn gen_random_rooms(regions: &mut Regions, attempts: i32) {
        for a in 0..attempts {
            let w = rand::thread_rng().gen_range(3, 6)*2+1;
            let h = rand::thread_rng().gen_range(3, 5)*2+1;
            let x = rand::thread_rng().gen_range(0, regions.width>>1)*2+1;
            let y = rand::thread_rng().gen_range(0, regions.height>>1)*2+1;
            let shape = rand::thread_rng().gen_range(0, 10);

            if Maze::can_place_room(regions, (x, y), (w, h)) {
                match shape {
                    0 => Maze::place_chamfered_room(regions, (x, y), (w, h), 2, a),
                    1 => Maze::place_chamfered_room(regions, (x, y), (w, h), 2, a),
                    _ => Maze::place_chamfered_room(regions, (x, y), (w, h), 0, a), 
                }
            }
        }
    }

    // Use a depth first walk to generate a perfect maze starting from (i,j)
    fn gen_perfect_maze(regions: &mut Regions, (i, j): (usize, usize), region: i32) {

        // Makes a list of neighbouring cells which are empty and the wall in between is empty
        fn test_neighbours(regions: &mut Regions, (i, j): (usize, usize)) -> Vec<(isize, isize)> {
            let mut v: Vec<(isize, isize)> = Vec::new();
            if regions[j][i-1] == EMPTY && regions[j][i-2] == EMPTY { v.push((-1, 0)) };
            if regions[j][i+1] == EMPTY && regions[j][i+2] == EMPTY { v.push(( 1, 0)) };
            if regions[j-1][i] == EMPTY && regions[j-2][i] == EMPTY { v.push((0, -1)) };
            if regions[j+1][i] == EMPTY && regions[j+2][i] == EMPTY { v.push((0,  1)) };
            return v;
        }

        // Initialise the stack
        let mut stack: Vec<(usize, usize)> = Vec::new();
        stack.push((i, j));

        // Mark the current cell as floor
        regions[j][i] = region;

        while !stack.is_empty() {
            if let Some((x, y)) = stack.pop() {

                // Create a vector with valid options of where to step next
                let next_cells = test_neighbours(regions, (x, y));

                if next_cells.len() != 0 {
                    // Push this cell back onto the stack
                    stack.push((x, y));

                    // Randomly choose one of the options to step to next
                    let next_id = rand::thread_rng().gen_range(0, next_cells.len());
                    let step = next_cells[next_id];
                    let next_wall = ((x as isize + step.0) as usize, (y as isize + step.1) as usize);
                    let next_cell = ((x as isize + step.0*2) as usize, (y as isize + step.1*2) as usize);

                    // Turn the wall and the next cell into path
                    regions[next_wall.1][next_wall.0] = region;
                    regions[next_cell.1][next_cell.0] = region;

                    // Push the next cell onto the stack
                    stack.push(next_cell);
                }
            }
        }
    }

    fn connect_regions(regions: &mut Regions) {

        fn get_neighbouring_regions(regions: &mut Regions, (x,y):(usize, usize)) -> HashSet<i32> {
            let mut neighbours = HashSet::new();
            if x > 0 && regions[y][x-1] >= 0 { neighbours.insert(regions[y][x-1]); }
            if y > 0 && regions[y-1][x] >= 0 { neighbours.insert(regions[y-1][x]); }
            if x < regions.width -1 && regions[y][x+1] >= 0 { neighbours.insert(regions[y][x+1]); }
            if y < regions.height-1 && regions[y+1][x] >= 0 { neighbours.insert(regions[y+1][x]); }
            neighbours
        }

        // Build a map of a list of connections for each room
        let mut connectors: HashMap<i32, Vec<(usize, usize)>> = HashMap::new();
        for j in 0..regions.height {
            for i in 0..regions.width {
                let neighbours = get_neighbouring_regions(regions, (i, j));
                if regions[j][i] == EMPTY && neighbours.len() > 1 {
                    // Push the cell into the map for each room it touches
                    for n in neighbours {
                        if n < 100 { // Maximum 100 rooms, TODO: Can we make this generic?
                            connectors.entry(n).or_insert_with(Vec::new).push((i,j));
                        }
                    }
                }
            }
        }

        let mut mappings: HashMap<i32, Vec<i32>> = HashMap::new();
        for room in connectors {
            // Build a list of wall cells that touch a region not yet merged
            let mut candidates: Vec<(usize, usize)> = Vec::new();
            for (i,j) in room.1 {
                for n in get_neighbouring_regions(regions, (i, j)) {
                    if !mappings.contains_key(&room.0) || 
                        !mappings[&room.0].contains(&n) { candidates.push((i,j)); break; }
                }
            }

            if candidates.len() > 0 {
                // Delete a random candidate wall and update the mappings
                let idx = rand::thread_rng().gen_range(0, candidates.len());
                let (x,y) = (candidates[idx].0, candidates[idx].1);
                regions[y][x] = 0;
                let mergees = get_neighbouring_regions(regions, (x, y));
                for m in &mergees {
                    for n in &mergees {
                        mappings.entry(*m).or_insert_with(Vec::new).push(*n);
                    }
                }
            }
        }
    }

    fn remove_deadends(regions: &mut Regions) {

        fn walk_deadend(regions: &mut Regions, (i,j): (usize, usize)) {
            if regions[j][i] >= 0 { // If the cell will be flooring
                let mut count: usize = 0;
                let (mut x, mut y) = (i, j);
                if regions[j][i+1] < 0 { count += 1; } else { x = i+1 }
                if regions[j][i-1] < 0 { count += 1; } else { x = i-1 }
                if regions[j+1][i] < 0 { count += 1; } else { y = j+1 }
                if regions[j-1][i] < 0 { count += 1; } else { y = j-1 }
                if count >= 3 { regions[j][i] = EMPTY; }
                if count == 3 {
                    walk_deadend(regions, (x,y));
                }
            }
        }

        for j in 0..regions.height {
            for i in 0..regions.width {
                walk_deadend(regions, (i,j));
            }
        }
    }

    fn regions_to_tiles(regions: &Regions, level: &mut Level) {
        for j in 0..regions.height {
            for i in 0..regions.width {
                match regions[j][i] {
                    EMPTY => level[j][i] = Tile{tile_type: TileType::_Empty, colour: Colour{r:64,g:64,b:64,a:255}},
                    WALL => level[j][i] = Tile{tile_type: TileType::_Wall, colour: Colour{r:0,g:0,b:0,a:255}},
                    _ => level[j][i] = Tile{tile_type: TileType::_Floor, colour: Colour{r:128,g:128,b:128,a:255}},
                }
            }
        }
    }

    pub fn find_spawn(level: &Level) -> (i32, i32) {

        fn valid_spawn(level: &Level, x: usize, y: usize) -> bool {
            level[y][x].tile_type == TileType::_Floor && 
            level[y][x+1].tile_type == TileType::_Floor && 
            level[y][x-1].tile_type == TileType::_Floor && 
            level[y+1][x].tile_type == TileType::_Floor && 
            level[y-1][x].tile_type == TileType::_Floor
        }

        let mut x = rand::thread_rng().gen_range(1, level.width-1);
        let mut y = rand::thread_rng().gen_range(1, level.height-1);
        while !valid_spawn(level, x, y) {
            x = rand::thread_rng().gen_range(1, level.width-1);
            y = rand::thread_rng().gen_range(1, level.height-1);           
        }
        (x as i32,y as i32)
    }
}