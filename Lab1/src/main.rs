fn prime(number: u32) -> bool {
    if number <= 1 {
        return false;
    }
    for i in 2..=((number as f64).sqrt() as u32) {
        if number % i == 0 {
            return false;
        }
    }
    true
}
fn coprime(mut a: u32, mut b: u32) -> bool {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a == 1
}

fn main() {
    for i in 1..=100 {
        if prime(i) {
            println!("{} is a prime number", i);
        } else {
            println!("{} is not a prime number", i);
        }
    }
    for i in 1..=100 {   //let mut i = 1; while i<=100 { let mut j = 1; while j<=100 {if coprime(i, j) { println!("{} and {} are coprime", i, j)j=j+1;}i=1+i;} <- vers cu while
         for j in 1..=100 {
             if coprime(i, j) {
                 println!("{} and {} are coprime", i, j);
             }
         }
     }
    for i in (0..=99).rev() {   //let mut i=99; while(i>0) ... i=i-1; <- vers cu while
        if i - 1 == 0 {
            println!(
                "{} bottle of beer on the wall,
            {} bottle of beer.
            Take one down, pass it around,
            no more bottles of beer on the wall.",
                i, i
            );
            break;
        }
        println!(
            "{} bottles of beer on the wall,
            {} bottles of beer.
            Take one down, pass it around,
            {} bottles of beer on the wall.",
            i,
            i,
            i - 1
        );
    }
}
