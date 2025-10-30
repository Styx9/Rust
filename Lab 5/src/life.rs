use std::{fs,io};

const WIDTH: usize = 30;
const HEIGHT: usize = 20;
const LIVE_CHAR: char = 'x';
const EMPTY_CHAR: char = ' ';

type Grid = Vec<Vec<char>>;

pub fn read_grid_from_file(path: &'static str) -> Result<Grid,io::Error>{
    let mut grid = Vec::new();
    let input = fs::read_to_string(path)?;
    for line in input.lines(){
        let row:Vec<char> = line.chars().collect();
        grid.push(row);
    }
    //aici vedem daca totul e in regula cu gridul nostru 
    let needed_width = grid[0].len();
    for row in grid.iter(){
        if row.len()!= needed_width
        {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Lungimile liniilor nu sunt egale!"));
        }
        for ch in row.iter(){
            if *ch != LIVE_CHAR && *ch != EMPTY_CHAR
            {
                return Err(io::Error::new(io::ErrorKind::InvalidData, "Caracter invalid in grid!"));
            }
        }
    }
    Ok(grid) 
}
pub fn print_grid(grid: &Grid){
    for row in grid.iter(){
        let line: String = row.iter().collect();
        println!("{}", line);
    }
}
pub fn place_pattern(grid: &Grid) -> Grid {
    let pattern_h = grid.len();
    let pattern_w = grid[0].len();
    let x = (WIDTH - pattern_w)/2;
    let y  = (HEIGHT- pattern_h)/2;
    let mut bigger_grid = initialize_grid();
    for i in 0..pattern_h{
        for j in 0..pattern_w{
            bigger_grid[y+i][x+j] = grid[i][j];
        }
    }
    bigger_grid
}
pub fn initialize_grid() -> Grid{
    let mut grid: Vec<Vec<char>> = Vec::new();
    for _ in 0..HEIGHT{
        let row = vec![EMPTY_CHAR;WIDTH];
        grid.push(row);
    }
    grid
}
pub fn count_neighbours(grid:&Grid, x:usize,y:usize ) -> u8
{
    let mut count =0;
    let height = grid.len();
    let width = grid[0].len();
    let possible_directions:[(isize,isize);8] = [(-1,-1),(-1,0),(-1,1),(0,-1),(0,1),(1,-1),(1,0),(1,1)];
    for (dx,dy) in possible_directions.iter(){
        let nx: isize  = x as isize + dx;
        let ny: isize = y as isize + dy;
        if nx < 0 || nx  >= width as isize || ny < 0 || ny >= height as isize{
            continue;
        }
        else if grid[ny as usize][nx as usize] == LIVE_CHAR{
            count+=1;
        }
    }
    count
}
pub fn next_cell_state(mut curr:char,neigh: u8) -> char{
    if curr == EMPTY_CHAR && neigh == 3 {
        curr = LIVE_CHAR;
    }
    if curr == LIVE_CHAR && !(2..=3).contains(&neigh){
        curr = EMPTY_CHAR;
    }
    curr
}
pub fn step(grid: &Grid) -> Grid{
    let mut new_grid:Vec<Vec<char>> = initialize_grid();
    for y in 0..HEIGHT{
        for x in 0..WIDTH{
            let mut  cell = grid[y][x];
            let neighbours = count_neighbours(grid, x, y);
            cell = next_cell_state(cell, neighbours);
            new_grid[y][x] = cell;
        }
    }
    new_grid
}
pub fn simulate(grid:&Grid, generations: usize) -> Grid{
     let mut current_grid = grid.clone();
    for _ in 1..=generations{
       current_grid = step(&current_grid);
       print_grid(&current_grid);
       std::thread::sleep(std::time::Duration::from_millis(300))
    }
    current_grid
}