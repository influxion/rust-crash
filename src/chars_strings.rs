#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use rand::Rng;
use std::io::stdin;

pub fn strings() {
    // utf-8
    let s: &str = "hello there!"; // &str = string slice --  statically allocated
    // s = "abc"; this does not work
    // let h = s[0]; doesnt work

    for c in s.chars().rev() {
        println!("{}", c)
    }

    if let Some(first_char) = s.chars().nth(0) {
        println!("first letter is {}", first_char)
    }

    // heap
    // String
    let mut letters = String::new();
    let mut a = 'a' as u8;
    while a <= ('z' as u8) {
        letters.push(a as char);
        letters.push_str(",");
        a += 1;
    }

    println!("{}", letters);

    // &str <> String
    let u: &str = &letters;

    // concat
    // String + str
    // let z = letters + &letters;

    // let mut abc = String::from("hello world");
    let mut abc = "hello world".to_string();
    abc.remove(0);
    abc.push_str("!!!");
    println!("{}", abc.replace("ello", "goodbye"));
}

pub fn string_formatting() {
    let name = "Jordon";
    let greeting = format!("hi, I'm {}, nice to meet you", name);
    println!("{}", greeting);

    let hello = "hello";
    let rust = "rust";
    let hello_rust = format!("{}, {}!", hello, rust);
    println!("{}", hello_rust);
    
    let run = "run";
    let forest = "forest";
    let rfr = format!("{0}, {1}, {0}!", run, forest);
    println!("{}", rfr);
    
    let info = format!("the names {last}. {first} {last}.", first = "james", last = "bond");
    println!("{}", info);
    
    let mixed = format!("{1} {} {0} {} {data}", "alpha", "beta", data = "delta");
    println!("{}", mixed);
}

pub fn number_guessing_game() {
    let number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Enter your guess: ");

        let mut buffer = String::new();

        match stdin().read_line(&mut buffer) {
            Ok(_) => {
                let parsed = buffer.trim_end().parse::<i64>();
                match parsed {
                    Ok(guess) => {
                        if guess < 1 || guess > 100 {
                            println!("Your guess is out of range");
                        } else if guess < number {
                            println!("Your guess is too low");
                        } else if guess > number {
                            println!("Your guess is too high");
                        } else {
                            println!("Correct!");
                            break
                        }
                    },
                    Err(e) => {
                        println!("Could not read your input. {}. Try again!", e)
                    }
                }
            },
            Err(_) => continue,
        }
    }
}

fn trim_tweet(tweet: &str) -> &str {
    &tweet[..20]
}

pub fn slices() {
    // Slices are references to a contiguous sequence of elements in a collection.

    let s = "my string";

    let tweet = String::from("this is my tweet and its very long");
    let trimmed_tweet = trim_tweet(&tweet);

    let tweet2 = "this is my tweet and its very long";
    let trimmed_tweet2 = trim_tweet(tweet2);


    println!("{trimmed_tweet}");
    println!("{trimmed_tweet2}");

    let a = [1, 2, 3, 4, 5, 6];
    let a_slice = &a[..3];
    println!("{:?}", a_slice)
}







