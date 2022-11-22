#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::mem;

struct Point {
    x: f64,
    y: f64
}

fn origin() -> Point {
    Point{x: 0.0, y: 0.0}
}

pub fn stack_and_heap() {
    let p1 = origin();
    let p2 = Box::new(origin());

    println!("p1 takes up {} bytes", mem::size_of_val(&p1));
    println!("p2 takes up {} bytes", mem::size_of_val(&p2));

    let p3 = *p2;
    println!("{}", p3.x)
}

// --------------------------

fn generate_string() -> String {
    String::from("Ferris")
}

fn print_string(value: String) {
    println!("{}", value);
}

fn add_to_string(mut p1: String) -> String {
    p1.push_str(" is awesome!");
    p1
}

fn print_integer(i: i32) {
    println!("i is: {}", i);

}


pub fn ownership() {
    let s1 =  String::from("Rust");
    let s2 = s1.clone();
    print_string(s1.clone());
    let s3 = generate_string();
    let s4 = add_to_string(s2);
    
    println!("s1 is: {s1}");
    println!("s3 is: {s3}");
    println!("s4 is: {s4}");
    
    let x = 10;
    let y = x;
    print_integer(x);
    println!("x is: {x}");
}

// --------------------------

fn print_string_borrowing(p1: &String) {
    println!("{p1}")
}

fn add_to_string_borrowing(p1: &mut String) {
    p1.push_str(" is awesome!");
}

fn generate_string_borrowing() -> String {
    String::from("Ferris")
}

pub fn borrowing() {
    let mut s1 =  String::from("Rust"); 
    let r1 = &s1;
    print_string_borrowing(r1);
    let r2 = &mut s1;
    add_to_string_borrowing(r2);
    println!("s1 is: {s1}");
    let s2 = generate_string();
}