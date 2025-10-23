use std::{fs, io};
use std::char;
use Metadata::len();
//Problema 1
fn read_file_p1() -> Result<String, io::Error> {
    let s = fs::read_to_string("src/text.txt")?;
    Ok(s)
}
//Problema 2
fn read_file_p2() -> Result<String,io::Error> {
    let s = fs::read_to_string("src/rot13.txt")?;
    Ok(s)
}
fn rot13(input: &str) -> Result<String, &'static str> {
    let mut s = String::new();
    for c in input.chars()
    {
        if !c.is_ascii()
        {
            return Err("Not an ascii character");      
        }
        let rot = match c {
          'A'..='Z' => (((c as u8 - b'A' + 13 ) % 26) + b'A') as char,
          'a'..='z' => (((c as u8 - b'a' + 13 ) % 26) + b'a') as char,
          _ => c,
        };
        s.push(rot);
    }
    Ok(s)
}
//Problema 3
fn correct(word:&str) -> &str
{
    match word {
        "pt" | "ptr" => "pentru",
        "dl" => "domnul",
        "dna" => "doamna",
        "vb" => "vorbit",
        "cv" => "ceva",
        _ => word,
    }   
}
//Problema 4
fn prob4() -> Result<(),io::Error>
{
    let input = fs::read_to_string("src/hosts.txt").expect("Error: Couldn't read from the hosts file"); //nu merge sa citesc hosts direct din calculator asa ca am pus inf intrun fisier text
    for line in input.lines()
    {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#')
        {
            continue;
        }
        let division: Vec<&str> = line.split_whitespace().collect();
        if division.len() >= 2 
        {
            println!("{} => {}",division[1],division[0]);
        }
    }
    Ok(())
}
fn main() -> Result<(), io::Error> {
    //Problema 1
    let s = read_file_p1()?;
    let mut longest_bytes = "";
    let mut longest_char = "";
    let mut max_bytes = 0;
    let mut max_chars = 0;
    for line in s.lines() {
        let byteln = line.len();
        let charln = line.chars().count();
        if byteln > max_bytes {
            max_bytes = byteln;
            longest_bytes = line;
        }
        if charln > max_chars {
            max_chars = charln;
            longest_char = line;
        }
    }
    println!(
        "Longest line by bytes is : {} with the lenght {}",
        longest_bytes, max_bytes
    );
    println!(
        "Longest line by chars is: {} with the lenght {}",
        longest_char,
        max_chars,
    );
    //Problema 2
    let text = read_file_p2()?;
    match rot13(&text) {
        Ok(res) => println!("Rot13:{}",res),
        Err(e) => {
            println!("{}",e);
        },
    }
    //Problema 3
    let input = fs::read_to_string("src/p3.txt").expect("Coulnd't open the specified file");
    let result: Vec<_> = input.split_whitespace().map(correct).collect();
    let final_string = result.join(" ");
    println!("P3: {}",final_string);
    //Problema 4
    prob4()?;
    Ok(())
}
