#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

fn how_many(x: i32) -> &'static str {
    match x {
        0 => "no",
        1 | 2 => "one or two",
        12 => "a dozen",
        9..=11 => "lots of",
        _ if (x % 2 == 0) => "some",
        _ => "a few"
    }
}

enum Color {
    Red,
    Green, 
    Blue,
    RbgColor(u8,u8,u8), // tuple
    Cmyk {cyan:u8, magenta:u8, yellow:u8, black:u8}
}

pub fn pattern_matching() {
    for x in 0..=12 {
        println!("{}: I have {} oranges", x, how_many(x));
    }

    let point = (0, 0);

    match point {
        (0, 0) => println!("origin"),
        (0, y) => println!("x axis, y = {}", y),
        (x, 0) => println!("y axis, x = {}", x),
        (_, y) => println!("(?,{})", y)
    }

    let c: Color = Color::Cmyk { cyan:0, magenta:0, yellow:0, black:255 };

    match c {
        Color::Cmyk {black: 255, .. } => println!("black"), // spread match
        _ => println!("not defined")
    }
}

