#![allow(dead_code)]
#![allow(unused_imports)]

pub fn if_statement() {
    let temp = 35;

    if temp > 30 {
        println!("really hot outside")
    } else if temp < -10 {
        println!("really cold!")
    } else {
        println!("nice outside today!")
    }

    let day = if temp > 20 {"sunny"} else {"cloudy"};
    println!("today is {}", day);

    println!("is it {}", if temp > 20 {"hot"} else if temp > 10 {"cold"} else {"OK"});

    println!("it is {}", if temp > 20 {if temp > 30 {"very hot"} else {"hot"}} else if temp < 10 {"cold"} else {"OK"});
}

pub fn while_and_loop() {
    let mut x = 1;

    while x < 1000 {
        x *= 2;

        if x == 64 { continue; }

        println!("x = {}", x)
    }

    let mut y = 1;

    loop { // while true
        y *= 2;
        println!("y = {}", y);

        if y == 1<<10 { break; }
    }
}

pub fn for_loop() {
    for x in 1..11 {
        if x == 3 { continue; }

        if x == 8 { break; }

        println!("x = {}", x);
    }

    for (pos,y) in (30..41).enumerate() {
        println!("{}: {}", pos, y)
    }
}

pub fn match_statement() {
    let country_code = 46;

    let country = match country_code {
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        1..=1000 => "unknown",
        _ => "invalid"
    };

    println!("the country with the code {} is {}", country_code, country);

    let x = false;

    let s = match x {
        true => "yes",
        false => "no"
    };

    println!("{}", s)
}