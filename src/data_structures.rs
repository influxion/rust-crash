#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::mem;

// structures
struct Point {
    x: f32,
    y: f32
}
struct Line {
    start: Point,
    end: Point
}

pub fn structures() {
    let p = Point { x: 3.0, y: 4.0 };
    println!("point p is at ({}, {})", p.x, p.y);

    let p2 = Point { x: 5.0, y: 10.0 };
    let myline = Line { start: p, end: p2 };

    println!("my line starts at x:{}, y:{} and ends at x:{}, y:{}", myline.start.x, myline.start.y, myline.end.x, myline.end.y);
}

// enums
enum Color {
    Red,
    Green, 
    Blue,
    RbgColor(u8,u8,u8), // tuple
    Cmyk {cyan:u8, magenta:u8, yellow:u8, black:u8}
}

pub fn enums() {
    let c: Color = Color::Cmyk { cyan:0, magenta:0, yellow:0, black:255 };

    match c {
        Color::Red => println!("r"),
        Color::Blue => println!("b"),
        Color::Green => println!("g"),
        Color::RbgColor(0, 0, 0) => println!("black"),
        Color::Cmyk {black: 255, .. } => println!("black"),
        Color::RbgColor(r, g, b) => println!("rgb({}, {}, {})", r, g, b),
        Color::Cmyk { cyan, magenta, yellow, black } => println!("cmyk({}, {}, {}, {})", cyan, magenta, yellow, black),
    }
}

// union types
union IntOrFloat {
    i: i32,
    f: f32
}

fn process_value(iof: IntOrFloat) {
    unsafe {
        match iof {
            IntOrFloat { i: 42 } => {
                println!("meaning of life")
            }

            IntOrFloat { f } => println!("value = {}", f)
        }
    }
}

pub fn union() {
    let mut iof = IntOrFloat { i:123 };
    iof.i =  234;

    let value = unsafe { iof.i };
    println!("iof.i value = {}", value);

    process_value(IntOrFloat{ i: 41 })
}

// Option<T>
pub fn optiont() {
    let x = 3.0;
    let y = 0.0;

    // Option
    let result = if y != 0.0 { Some(x/y) } else {None};

    match result {
        Some(z) => println!("{}/{}={}", x, y, z),
        None => println!("cannot divide by zero")
    }

    // tries to check if right side can assign to left side and will either return a true or false
    if let Some(z) = result { 
        println!("result = {}", z)
    }

    // while let Some(z) = result {
    // } // can also do while statement ones
}

// arrays
pub fn arrays() {
    let mut a = [1, 2, 3, 4, 5];

    println!("a has {} els, first is {}", a.len(), a[0]);
    a[0] = 321;
    println!("a[0] = {}", a[0]);

    println!("{:?}", a);

    if a != [1,2,3,4,5] {
        println!("does not match")
    }

    let b = [1; 10]; // b.len() == 10
    for i in 0..b.len() {
        println!("{}", b[i])
    }

    println!("b took up {} bytes", mem::size_of_val(&b));

    let mtx: [[f32; 3]; 3] = [
        [1.0, 0.0, 0.0],
        [0.0, 2.0, 0.0],
        [0.0, 0.0, 4.0]

    ];

    println!("{:?}", mtx);

    for i in 0..mtx.len() {
        for j in 0..mtx[i].len() {
            if i == j {
                println!("mtx[{}][{}] = {}", i, j, mtx[i][j]);
            }
        }
    }

}

// slcies
fn use_slice(slice: &mut [i32]) {
    println!("first elem = {}, len = {}", slice[0], slice.len());
    slice[0] = 4321
}

pub fn slices() {
    let mut data = [1,2,3,4,5];

    use_slice(&mut data[1..4]);
    // use_slice(&mut data);
    println!("{:?}", data);
}

//tuples
fn sum_and_product(x: i32, y: i32) -> (i32, i32) {
    (x+y, x*y)
}

pub fn tuples() {
    let x = 3;
    let y = 4;
    let sp = sum_and_product(x, y);

    println!("sp = {:?}", sp);
    println!("{0} + {1} = {2}, {0} * {1} = {3}", x, y, sp.0, sp.1);

    // destructuring
    let (a, b) = sp;
    println!("a = {}, b = {}", a, b);

    let sp2 = sum_and_product(4, 7);
    let combined = (sp, sp2);
    println!("{:?}", combined);
    println!("last elem = {}", (combined.1).1);

    let ((c,d), (e, f)) = combined;

    let foo = (true, 42.0, -1i8);

    println!("{:?}", foo);

    let meaning = (42,); // single tuple

    println!("{:?}", meaning);
}

// generics
struct PointTwo<T> {
    x: T,
    y: T
}

struct LineTwo<T> {
    start: PointTwo<T>,
    end: PointTwo<T>
}

pub fn generics() {
    let a = PointTwo { x: 0.0, y: 4.0 };
    let b = PointTwo { x: 1.2, y: 3.4 };

    let myline = LineTwo { start: a, end: b };
}
