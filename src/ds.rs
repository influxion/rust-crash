#![allow(dead_code)]
#![allow(unused_imports)]


struct Point {
    x: f64,
    y: f64
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

enum Color {
    Red,
    Green, 
    Blue,
    RbgColor(u8,u8,u8), // tuple
    Cmyk {cyan:u8, magenta:u8, yellow:u8, black:u8}
}

pub fn enums() {
    let c: Color = Color::Cmyk { cyan:0, magenta:0, yellow:0, black:55 };

    match c {
        Color::Red => println!("r"),
        Color::Blue => println!("b"),
        Color::Green => println!("g"),
        Color::RbgColor(0, 0, 0) => println!("black"),
        Color::Cmyk { cyan: _, magenta: _, yellow: _, black: 255 } => println!("black"),
        Color::RbgColor(r, g, b) => println!("rgb({}, {}, {})", r, g, b),
        Color::Cmyk { cyan, magenta, yellow, black } => println!("cmyk({}, {}, {}, {})", cyan, magenta, yellow, black),
    }
}

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

pub fn optiont() {
    let x = 3.0;
    let y = 1.0;

    // Option
    let result = if y != 0.0 { Some(x/y) } else {None};

    match result {
        Some(z) => println!("{}/{}={}", x, y, z),
        None => println!("cannot divide by zero")
    }
}





