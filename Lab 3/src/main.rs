use std::char;
#[derive(Debug)]
enum ErrorType{
    Overflow,
    // NotAscii,
    // NotDigit,
    // NotBase16,
    NotLetter,
    // NotPrintable,
}
//Problema 1
fn is_prime(x: u16)->bool
{
    if x < 2
        {return false;
        }
    for i in 2..=((x as f64).sqrt()) as u16
        {
            if x % i ==0{
                return false;
            }

        }
    true
}
fn next_prime(mut x: u16) -> Option<u16>
{
    if x == u16::MAX
        {
            return None;
        }
    x+=1;
    while x < u16::MAX{
        if is_prime(x){
            return Some(x);
        }
        if x == u16::MAX{ //ne oprim inainte sa dea overflow pe u16
            break;
        }
        x+=1;
    }
    None
}
//Problema 2
fn check_add(a:u32,b:u32)->u32
{
    if b > (u32::MAX - a){
        panic!("Error: Addition overflows on the u32 type {a}+{b}...");
    }
    a+b
}
fn check_mul(a:u32,b:u32) -> u32{
    if a!=0 && b > u32::MAX/a{
        panic!("Error: Multiplication overflow on the u32 type {a}*{b}");
    }
    a*b
}
//Problema 3
fn what(e: ErrorType) -> String{
    match e {
        ErrorType::Overflow=>"Error: Overflow return value exceeds u32 type".to_string(),
        ErrorType::NotLetter=>"Error: Input type not a letter".to_string(),
    }
}
fn check_add_enum(a:u32,b:u32)->Result<u32,ErrorType>
{
    if b > (u32::MAX - a){
        Err(ErrorType::Overflow)
    }
    else {
        Ok(a+b)
    }
}
fn check_mul_enum(a:u32,b:u32) -> Result<u32, ErrorType>{
    if a!=0 && b > u32::MAX/a{
        Err(ErrorType::Overflow)
    }
    else{    
    Ok(a*b)
    }
}
fn to_uppercase(a:char)->Result<char,ErrorType>
{
    if !is_alphabetic(a){
        Err(ErrorType::NotLetter)
    } 
    else{
    Ok(a)
    }
}

fn main() {
    //Problema 1
    // let mut a:u16 = 65000;//sa se afiseze doar cateva numere
    // while let Some(p) = next_prime(a){
    //     println!("{a}");
    //     a=p;
    // }

    //Problema 2
    //cu eroare 
    let a:u32 = 4_294_967_294;
    let b:u32 = 2;
    // let r = check_add(a, b);
    // println!("{r}");
    //cu eroare
    // let r = check_mul(a, b);
    // println!("{r}");

    //Problema 3






    //pt problema 3-4 putem concatena enumurile si sa folosim o functie globala de print error cu match cu what function fn what(e:ErrorType)
    //Enum ErrorType : E1,E2
    //what(e:Errortype) -> string
    //match e
}
