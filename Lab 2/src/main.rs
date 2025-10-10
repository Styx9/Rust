//---Problema 1---
fn add_chars_n(mut s: String, c: char, n: i32) -> String {
    for _ in 0..n {
        s.push(c);
    }
    s
}
//---Problema 2---
fn add_chars_n2(s: &mut String, c: char, n: i32) {
    for _ in 0..n {
        s.push(c);
    }
}
//---Problema 3---
fn add_space(s: &mut String, n: i32) {
    for _ in 0..n {
        s.push(' ');
    }
}
fn add_str(s1: &mut String, s2: &str) {
    s1.push_str(s2);
}
fn add_integer(s: &mut String, mut nr: i32) {
    if nr == 0{
        s.push('0');
        return;}
    let mut temp = String::new();
    let mut i = 0;

    while nr != 0 {
        if i == 3 {
            temp.push('_');
            i = 0;
        }
        let c = ((nr % 10) as u8 + b'0') as char;
        temp.push(c);
        i += 1;
        nr /= 10;
    }
    while let Some(ch) = temp.pop(){
        s.push(ch);
    }
}
fn add_float(s: &mut String, nr: f32) {
    //facut manual cum zice in cerinta
    let int_part = nr as i32;
    let mut frac_part = nr - int_part as f32;
    let mut pow = 1;
    if int_part == 0 {
        s.push('0');
    } else {
        while int_part / pow >= 10 {
            pow *= 10;
        }
    }
    while pow > 0 {
        let c = int_part / pow;
        s.push((c as u8 + b'0') as char);
        pow /= 10;
    }
    s.push('.');
    for _ in 0..3 {
        //3 cifre dupa virgula maxim
        frac_part *= 10.0;
        let c = frac_part as i32;
        s.push((c as u8 + b'0') as char);
        frac_part -= c as f32;
    }
}
fn main() {
    let mut s = String::from("");
    let mut i = 0;
    while i < 26 {
        let c = (i as u8 + b'a') as char;
        s = add_chars_n(s, c, 26 - i);
        i += 1;
    }
    println!("{}", s);
    //pentru al doilea exercitiu
    let mut s = String::from("");
    let mut i = 0;
    while i < 26 {
        let c = (i as u8 + b'a') as char;
        add_chars_n2(&mut s, c, 26 - i);
        i += 1;
    }
    println!("{}", s);
    //pentru al treilea exercitiu
    let mut s = String::new();
    add_space(&mut s, 40); add_str(&mut s, "I 💚\n");
    add_space(&mut s, 40); add_str(&mut s, "RUST.\n");
    add_space(&mut s, 4);  add_str(&mut s, "Most");
    add_space(&mut s, 12); add_str(&mut s, "crate");
    add_space(&mut s, 6);  add_integer(&mut s, 306437968);
    add_space(&mut s, 11); add_str(&mut s, "and");
    add_space(&mut s, 5);  add_str(&mut s, "lastest");
    add_space(&mut s, 9);  add_str(&mut s, "is\n");
    add_space(&mut s, 9);  add_str(&mut s, "downloaded");
    add_space(&mut s, 8);  add_str(&mut s, "has");
    add_space(&mut s, 13); add_str(&mut s, "downloads");
    add_space(&mut s, 5);  add_str(&mut s, "the");
    add_space(&mut s, 9);  add_str(&mut s, "version");
    add_space(&mut s, 4);  add_float(&mut s, 2.038);
    add_str(&mut s, ".\n");
    println!("{}", s);
}
