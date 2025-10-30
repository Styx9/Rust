use std::{fs, io};
use std::str::FromStr;
use serde_derive::Deserialize;
use anyhow::Result;

use crate::life::read_grid_from_file;
mod life;
//Problema 1
fn p1() -> Result<()>
{
    let input =fs::read_to_string("data/p1.txt")?;
    let mut maxage:i32 = 0;
    let mut minage:i32 = 200;
    let mut maxname = String::new();
    let mut minname = String::new();
    for line in input.lines()
    {
        let vec:Vec<_> = line.splitn(3,',').collect();
        let age:i32 = match FromStr::from_str(vec[2].trim())
        {
            Ok(age) => age,
            Err(_) => {
                println!("Varsta inexistenta sau invalida!");
                continue;
            },//daca lipseste varsta trecem la urmatoarea linie
        
        };
        if age > maxage
        {
            maxage = age;
            maxname = vec[0].to_string();
        }
        if age < minage
        {
            minage = age;
            minname = vec[0].to_string();
        }
    }
    println!("P1 Using a TXT file :  Max age: {} - {}, Min age: {} - {}", maxage, maxname, minage, minname);
Ok(())
}
//Problema 2
struct Canvas{
    pixels: Vec<Vec<char>>,
}
fn new_canvas() -> Canvas{
    let pixels = vec![vec![' '; 100]; 55]; 
    Canvas { pixels}   
}
fn set_pixels(canvas : &mut Canvas, pixels: &[(usize, usize, u8)]){
    for &(x, y, chr) in pixels{
    if x >= 100 || y >= 55
    {
    println!("Coordinates ({},{}) are out of bounds!", x,y);
    }
        else{
            canvas.pixels[y][x] = chr as char;
        }
    }
}
fn print(canvas: Canvas){
    for row in &canvas.pixels{
        let line: String = row.iter().collect();
        println!("{}", line);
    }
}
//Problema 3

#[derive(Debug, Deserialize)]
struct Student{
    name: String,
    #[allow(dead_code)]//nu folosesc phone la nimic dar il las in structura pentru a functioa pe json
    phone: String,
    age: u8,
}
fn p3() ->Result<()>{
    let input = fs::read_to_string("data/p3.json")?;
    let students: Vec<Student> = serde_json::from_str(&input)?;
    let mut maxage = 0;
    let mut minage = 200;
    let mut minname = String::new();
    let mut maxname = String::new();
    for student in students{
        if student.age > maxage{
        maxage = student.age;
        maxname = student.name.clone();
    }
        if student.age < minage{
        minage = student.age;
        minname = student.name.clone();}
    }
    println!("P3 Using a JSON file : Max age: {} - {}, Min age: {} - {}", maxage, maxname, minage, minname);
    Ok(())
}
//Bonus

fn main() -> Result<()>{
    //Problema 1
    if let Err(e) = p1() {
    println!("Eroare la rularea Problemei 1: {}", e);
    }
    //Pronlema 2
    let mut canvas = new_canvas();
    let c = &mut canvas;

    set_pixels(c, &[(4, 25, 124), (3, 33, 124), (2, 24, 95), (4, 3, 95)]);
    set_pixels(c, &[(7, 2, 95), (4, 21, 124), (5, 16, 95)]);
    set_pixels(c, &[(4, 41, 124), (7, 1, 124), (5, 8, 92)]);
    set_pixels(c, &[(1, 31, 40), (2, 3, 95), (2, 41, 124)]);
    set_pixels(c, &[(2, 16, 95), (5, 35, 92), (6, 3, 95), (2, 11, 95), (5, 3, 95)]);
    set_pixels(c, &[(2, 38, 95), (4, 9, 40), (3, 41, 124), (2, 37, 95), (2, 25, 124)]);
    set_pixels(c, &[(5, 27, 124), (2, 27, 124), (4, 0, 124), (3, 35, 47), (2, 18, 95)]);
    set_pixels(c, &[(4, 13, 124), (4, 37, 95), (4, 16, 40), (3, 6, 124)]);
    set_pixels(c, &[(7, 32, 47), (4, 20, 124), (5, 11, 95), (5, 42, 95)]);
    set_pixels(c, &[(5, 15, 92), (4, 34, 124), (4, 45, 41), (5, 24, 95)]);
    set_pixels(c, &[(4, 2, 40), (7, 3, 95), (2, 44, 95)]);
    set_pixels(c, &[(6, 30, 95), (5, 45, 95), (4, 31, 124), (4, 7, 124), (3, 43, 39)]);
    set_pixels(c, &[(5, 17, 95), (1, 27, 124), (2, 5, 95)]);
    set_pixels(c, &[(3, 44, 95), (3, 19, 92), (5, 23, 95), (3, 8, 47), (2, 10, 95)]);
    set_pixels(c, &[(6, 6, 124), (5, 19, 47), (3, 24, 95), (3, 27, 124)]);
    set_pixels(c, &[(3, 10, 95), (4, 44, 95), (2, 9, 95), (0, 32, 95), (5, 2, 95)]);
    set_pixels(c, &[(6, 2, 95), (7, 31, 95), (1, 25, 124), (2, 36, 95)]);
    set_pixels(c, &[(3, 46, 92), (5, 25, 44), (1, 43, 124), (5, 46, 47), (3, 15, 47)]);
    set_pixels(c, &[(4, 17, 95), (2, 23, 95), (3, 39, 92)]);
    set_pixels(c, &[(4, 47, 124), (2, 45, 95), (3, 37, 95)]);
    set_pixels(c, &[(5, 44, 95), (2, 2, 95), (5, 10, 95), (5, 9, 95), (4, 43, 124)]);
    set_pixels(c, &[(4, 38, 41), (2, 17, 95), (0, 26, 95)]);
    set_pixels(c, &[(4, 18, 41), (7, 5, 47), (5, 41, 124), (5, 33, 124)]);
    set_pixels(c, &[(5, 12, 47), (5, 22, 92), (6, 33, 124), (5, 31, 124)]);
    set_pixels(c, &[(4, 40, 124), (3, 3, 95), (4, 4, 124), (6, 31, 47), (3, 4, 96)]);
    set_pixels(c, &[(0, 42, 95), (5, 18, 95), (4, 27, 124)]);
    set_pixels(c, &[(3, 12, 92), (2, 32, 95), (5, 37, 95), (5, 26, 95), (5, 39, 47)]);
    set_pixels(c, &[(3, 25, 96), (4, 14, 124), (4, 33, 124), (3, 1, 47)]);
    set_pixels(c, &[(5, 36, 95), (7, 30, 95), (6, 4, 47), (4, 24, 95), (1, 32, 95)]);
    set_pixels(c, &[(3, 22, 47), (4, 23, 40), (5, 6, 124)]);
    set_pixels(c, &[(1, 33, 41), (1, 41, 124), (7, 29, 124)]);
    set_pixels(c, &[(4, 6, 124), (5, 38, 95), (3, 31, 124), (7, 4, 95)]);
    set_pixels(c, &[(4, 11, 41), (4, 10, 95), (5, 1, 92)]);
    set_pixels(c, &[(2, 43, 124), (3, 17, 95), (5, 4, 44), (4, 36, 40)]);
    set_pixels(c, &[(5, 43, 46)]);
    print(canvas);
    //Problema 3
     if let Err(e) = p3() {
    println!("Eroare la rularea Problemei 3: {}", e);
    }
    //Problema Bonus
    println!("Choose which starting test file to use for the life game: 1 (blinker) 2 (pulsar)");
    let mut input = String::new();
    let mut grid;
    match io::stdin().read_line(&mut input){
        Ok(_) => {},
        Err(e) =>{
            println!("Error reading input! {}",e);
            return Ok(());
        }
    }
    if input.trim()== "1"
    {
       grid =  match read_grid_from_file("data/blinker.txt")
        {
            Ok(g) => g,
            Err(e) =>{
                println!("Error reading from file! {}",e);
                return Ok(());
            }
        };
    }
    else if input.trim() == "2"
    {
          grid = match read_grid_from_file("data/pulsar.txt")
        {
            Ok(g) => g,
            Err(e) =>{
                println!("Error reading from file! {}",e);
                return Ok(());
            }
        };
    }
    else {
    println!("Invalid choice, please type 1 or 2.");
    return Ok(());
    }
    //facem 5 generatii 
    grid = life::place_pattern(&grid);
    life::simulate(&grid, 5);
    Ok(())
}
