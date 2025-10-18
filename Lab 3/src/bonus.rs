use thiserror::Error;

#[derive(Error,Debug)]
pub enum MyError {
    #[error("Overflow: {0} + {1} exceeds u32")]
    Overflow(u32,u32),
    #[error("Character is not ascii")]
    NotAscii,
    #[error("Character is not digit")]
    NotDigit,
    #[error("Character is not letter")]
    NotLetter,
    #[error("Character is not printable")]
    NotPrintable,
    #[error("Character is not hex digit")]
    NotBase16,
}
pub fn check_add_enum(a:u32,b:u32)->Result<u32,MyError>
{
    if b > (u32::MAX - a){
        Err(MyError::Overflow(a,b))
    }
    else {
        Ok(a+b)
    }
}
pub fn check_mul_enum(a:u32,b:u32) -> Result<u32, MyError>{
    if a!=0 && b > u32::MAX/a{
        Err(MyError::Overflow(a,b))
    }
    else{    
    Ok(a*b)
    }
}
pub fn propagate_err(a:u32,b:u32,c:u32)->Result<u32,MyError>
{
    let sum = check_add_enum(a, b)?;
    let rezultat = check_mul_enum(sum, c)?;
    Ok(rezultat)
}
//Problema 4
pub fn to_uppercase(a:char)->Result<char,MyError>
{
    if !a.is_ascii_alphabetic(){
        return Err(MyError::NotLetter);
    } 
    Ok(a.to_ascii_uppercase())
}
pub fn to_lowercase(a:char)->Result<char,MyError>
{
    if !a.is_ascii_alphabetic(){
        return Err(MyError::NotLetter);
    }
    Ok(a.to_ascii_lowercase())
}
pub fn print_char(a:char)->Result<(),MyError>
{
    if !a.is_ascii(){
        return Err(MyError::NotAscii);
    }
    if !a.is_ascii_graphic(){
        return Err(MyError::NotPrintable);
    }
    println!("Character: {a}");
    Ok(())
}
pub fn char_to_number(a:char)->Result<u32,MyError>{
    if !a.is_ascii(){
        return Err(MyError::NotAscii);
    }
    if !a.is_ascii_digit(){
        return Err(MyError::NotDigit);
    }
    //nu putem returna cu unwrap asa ca am folosit un ok_or
    a.to_digit(10).ok_or(MyError::NotDigit)
}
pub fn char_to_number_hex(a:char)->Result<u32,MyError>
{
    if !a.is_ascii(){
        return Err(MyError::NotAscii);
    }
    if !a.is_ascii_hexdigit(){
        return Err(MyError::NotBase16);
    }
    a.to_digit(16).ok_or(MyError::NotBase16)
}