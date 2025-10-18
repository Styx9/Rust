use std::char;
mod bonus;
#[derive(Debug)]
enum ErrorType{
    Overflow,
    NotAscii,
    NotDigit,
    NotBase16,
    NotLetter,
    NotPrintable,
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
fn print_error(e: ErrorType) -> String{
    match e {
        ErrorType::Overflow=>"Error: Overflow return value exceeds u32 type".to_string(),
        ErrorType::NotLetter=>"Error: Input type not a letter".to_string(),
        ErrorType::NotAscii=>"Error: Input type not an ASCII character".to_string(),
        ErrorType::NotPrintable=>"Error: Input is not printable".to_string(),
        ErrorType::NotDigit=>"Error: Input is not a digit".to_string(),
        ErrorType::NotBase16=>"Error: Input is not in base 16".to_string(),
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
fn propagate_err(a:u32,b:u32,c:u32)->Result<u32,ErrorType>
{
    let sum = check_add_enum(a, b)?;
    let rezultat = check_mul_enum(sum, c)?;
    Ok(rezultat)
}
//Problema 4
fn to_uppercase(a:char)->Result<char,ErrorType>
{
    if !a.is_ascii_alphabetic(){
        return Err(ErrorType::NotLetter);
    } 
    Ok(a.to_ascii_uppercase())
}
fn to_lowercase(a:char)->Result<char,ErrorType>
{
    if !a.is_ascii_alphabetic(){
        return Err(ErrorType::NotLetter);
    }
    Ok(a.to_ascii_lowercase())
}
fn print_char(a:char)->Result<(),ErrorType>
{
    if !a.is_ascii(){
        return Err(ErrorType::NotAscii);
    }
    if !a.is_ascii_graphic(){
        return Err(ErrorType::NotPrintable);
    }
    println!("Character: {a}");
    Ok(())
}
fn char_to_number(a:char)->Result<u32,ErrorType>{
    if !a.is_ascii(){
        return Err(ErrorType::NotAscii);
    }
    if !a.is_ascii_digit(){
        return Err(ErrorType::NotDigit);
    }
    //nu putem returna cu unwrap asa ca am folosit un match
    match a.to_digit(10) {
        Some(v) => Ok(v),
        None => Err(ErrorType::NotDigit),
    }
}
fn char_to_number_hex(a:char)->Result<u32,ErrorType>
{
    if !a.is_ascii(){
        return Err(ErrorType::NotAscii);
    }
    if !a.is_ascii_hexdigit(){
        return Err(ErrorType::NotBase16);
    }
    match a.to_digit(16) {
        Some(v) => Ok(v),
        None => Err(ErrorType::NotBase16),
    }
}
//Problema 5
//Verifica daca putem face o impartire
#[derive(Debug)]
enum DivError{
    DivByZero,
    MissingOp,
}
fn division_check(a: Option<f64>,b:Option<f64>) -> Result<f64, DivError>
{
    let x = a.ok_or(DivError::MissingOp)?;
    let y = b.ok_or(DivError::MissingOp)?;
    if y == 0.0{
        Err(DivError::DivByZero)
    }
    else {
        Ok(x/y)
    }
}
fn main() {
    //Problema 1
    println!("Problema 1");
    let mut a:u16 = 65000;//sa se afiseze doar cateva numere
    while let Some(p) = next_prime(a){
        println!("{a}");
        a=p;
    }

    //Problema 2
    println!("Problema 2 (cu eroare)");
    //cu eroare
    let a:u32 = 4_294_967_294;
    let b:u32 = 2;
    let r = check_add(a, b);
    println!("{r}");
    //cu eroare
    let r = check_mul(a, b);
    println!("{r}");
    
    //Problema 3
    println!("Problema 3");
    match propagate_err(u32::MAX, 2, 3){ //da eroare
        Ok(rezultat) => println!("(a+b)*c={rezultat}"),
        Err(e) => println!("{}",print_error(e)),
    }

    //Problema 4
    println!("Problema 4");
    match to_uppercase('c') { //merge
        Ok(ch) => println!("c to uppercase = {ch}"),
        Err(e) =>println!("{}", print_error(e)),
    }
    match to_lowercase('1'){ //da fail
        Ok(ch) => println!("1 to lowercase -> {ch}"),
        Err(e)=>println!("{}",print_error(e),)
    }
    match print_char('\n'){ //da fail
        Ok(_) => println!("Printed successfully"),
        Err(e) => println!("{}",print_error(e)),
    }
    match char_to_number('7'){ //merge
        Ok(ch)=> println!("'7' = {ch}"),
        Err(e) => println!("{}",print_error(e),)
    }
    match char_to_number_hex('K'){ //da fail
        Ok(ch) => println!("'K' hex value = {ch}"),
        Err(e) => println!("{}",print_error(e),)
    }

    //Problema 5
    let a = Some(8.0);
    let b = Some(2.0);
    let c = Some(0.0);
    let d:Option<f64> = None;

    println!("8 / 2 = {:?}",division_check(a,b)); //va merge
    println!("2 / 0 = {:?}", division_check(b,c)); //va da eroare pt div by zero
    println!("10 / None = {:?}", division_check(a, d)); //va da eroare pt missing operand
    
    //Problema Bonus //am modificat ex 3 si 4 codul este in bonus.rs
    println!("Problema Bonus");
    match bonus::propagate_err(u32::MAX, 2, 3){ //da eroare
        Ok(rezultat) => println!("(a+b)*c={rezultat}"),
        Err(e) => println!("Error:{}",e),
    }
    match bonus::to_uppercase('c') { //merge
        Ok(ch) => println!("c to uppercase = {ch}"),
        Err(e) =>println!("Error:{}", e),
    }
    match bonus::to_lowercase('1'){ //da fail
        Ok(ch) => println!("1 to lowercase -> {ch}"),
        Err(e)=>println!("{}",e)
    }
    match bonus::print_char('\n'){ //da fail
        Ok(_) => println!("Printed successfully"),
        Err(e) => println!("{}",e),
    }
    match bonus::char_to_number('7'){ //merge
        Ok(ch)=> println!("'7' = {ch}"),
        Err(e) => println!("{}",e,)
    }
    match bonus::char_to_number_hex('K'){ //da fail
        Ok(ch) => println!("'K' hex value = {ch}"),
        Err(e) => println!("{}",e,)
    }
}
