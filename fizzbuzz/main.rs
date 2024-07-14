#![allow(unused)]
fn main() {
   for x in fizz_buzz(30) {
    println!("{x}")
   }
}

fn modulus() {
    print!("{}", 1 % 3);
    print!("{}", 3 % 1);
    print!("{}", 3 % 3);
}

fn forfunc() {
    for i in 0..6 {
        print!("{i}");
    }
}

pub fn fizz_buzz(n: i32) -> Vec<String> {
    let fizz = "Fizz";
    let buzz: &'static str = "Buzz";
    let mut vec: Vec<String> = Vec::new();
    for i in 1..=n {
        let mut val = String::new();

        if i % 3 == 0 {
            val += fizz;
        }
        if i % 5 == 0 {
            val += buzz;
        }
        if val.is_empty() {
            val += &i.to_string();
        }
        vec.push(val);
    }
    return vec;
}
