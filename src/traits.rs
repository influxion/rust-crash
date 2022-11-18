#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

trait Animal {
    fn create(name: &'static str) -> Self;

    fn name(&self) -> &'static str;

    fn talk(&self) {
        println!("{} cannot talk", self.name())
    }
}

struct Cat {
    name: &'static str
}

struct Human {
    name: &'static str
}

impl Animal for Cat {
    fn create(name: &'static str) -> Cat {
        Cat { name }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn talk(&self) {
        println!("{} says meow", self.name())
    }
}

impl Animal for Human {
    fn create(name: &'static str) -> Human {
        Human { name }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn talk(&self) {
        println!("{} says hello", self.name())
    }
}

trait Summable<T> {
    fn sum(&self) -> T;
}

impl Summable<i32> for Vec<i32> {
    fn sum(&self) -> i32 {
        let mut result: i32 = 0;
        for x in self { result += *x; }
        result
    }
}

pub fn traits() {
    let h: Human = Animal::create("John");
    h.talk();

    let c = Cat::create("Misty");
    c.talk();

    let a = vec![1,2,3];
    println!("sum = {}", a.sum());
}

//-----------------------------------------------------------------------
use std::{fmt::Debug, str::SplitWhitespace};


#[derive(Debug)]
struct Circle {
    radius: f64,
}
#[derive(Debug)]
struct Square {
    side: f64,
}

trait Shape {
    fn area(&self) -> f64;
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        self.radius * self.radius * std::f64::consts::PI
    }
}

// fn print_info(shape: impl Shape + Debug) {
fn print_info<T: Shape + Debug>(shape: T) {
    println!("{:?}", shape);
    println!("The area is {}", shape.area())
}

pub fn params() {
    let c = Circle { radius: 2.0 };
    print_info(c)
}

// -----------------------------------------------------------------------

pub fn test() {
}