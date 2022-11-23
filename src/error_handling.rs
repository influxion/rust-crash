#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]

use core::panic;
use std::{io, fs, error::Error, num::ParseIntError, collections::HashMap, fmt::{Display, Debug, write}};

pub fn unrecoverable_errors() {
    let v = vec!["one", "two", "three"];
    println!("{}", v[3]);

    panic!("Somethign went horribly wrong");
}

pub fn recoverable_errors() {
    let file = fs::File::open("example.txt").expect("Failed to open file!");

    // let file = match file {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("Failed to open file: {:?}", error)
    //     }
    // };

}

fn get_user_id(username: &str) -> Result<i32, String> {
    if username.is_empty() {
        Err("Username can not be empty".to_owned())
    } else {
        Ok(1)
    }
}
// -------------

pub fn propagating_errors() {
    let content = read_file("example.txt");
}

fn read_file(filename: &str) -> Result<String, io::Error> {
    let mut contents = fs::read_to_string(filename)?;
    Ok(contents)
}

//---------

pub fn multiple_error_types() {
    let i = parse_file("example.txt");
}

enum ParseFileError {
    File,
    Parse(ParseIntError)
}

fn parse_file(filename: &str) -> Result<i32, ParseFileError> {
    let s = fs::read_to_string(filename).map_err(|e| ParseFileError::File)?;
    let i = s.parse().map_err(|e| ParseFileError::Parse(e))?;
    Ok(i)
}

//----------------------
// ADVANCED ERROR HANDLING
//----------------------

struct ParsePaymentInfoError {
    source: Option<Box<dyn Error>>,
    msg: String
}

impl Debug for ParsePaymentInfoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self} \n\t{}", self.msg)?;

        if let Some(e) = self.source.as_ref() {
            write!(f, "\n\nCaused by:\n\t{e:?}")?;
        }

        Ok(())
    }
}

impl Display for ParsePaymentInfoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Parsing payment error: invalid payment info")
    }
}

impl Error for ParsePaymentInfoError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_deref()
    }
}

fn parse_card_numbers(card: &str) -> Result<Vec<u32>, ParsePaymentInfoError> {
    let numbers = card
        .split(" ")
        .into_iter()
        .map(|s| {
            s.parse().map_err(|_| ParsePaymentInfoError {
                source: None,
                msg: format!("{s:?} could not be parsed as u32")
            })
        })
        .collect::<Result<Vec<u32>, _>>()
        .map_err(|e| ParsePaymentInfoError {
            source: Some(Box::new(e)),
            msg: format!("Failed to parse input as numbers. Input: {card}")
        })?;

    Ok(numbers)
}

#[derive(Debug)]
struct Expiration {
    year: u32,
    month: u32
}

#[derive(Debug)]
struct Card {
    number: u32,
    exp: Expiration,
    cvv: u32,
}

fn parse_card(card: &str) -> Result<Card, ParsePaymentInfoError> {
    let mut numbers = parse_card_numbers(card)?;

    let len = numbers.len();
    let expected_len = 4;

    if len != expected_len {
        return Err(ParsePaymentInfoError { 
            source: None, 
            msg: format!("Incorrect number of elements parsed. Expected {expected_len} but got {len}. Elements: {numbers:?}")
        })
    }

    let cvv = numbers.pop().unwrap();
    let year = numbers.pop().unwrap();
    let month = numbers.pop().unwrap();
    let number = numbers.pop().unwrap(); 

    Ok(Card {
        number,
        exp: Expiration { year, month },
        cvv
    })
}

enum CreditCardError {
    InvalidInput(String),
    Other(Box<dyn Error>, String)
}

impl Debug for CreditCardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidInput(msg) => write!(f, "{self}\n{msg}"),
            Self::Other(e, msg) => write!(f, "{self}\n{msg}\n\nCaused by:\n\t{e:?}")
        }
    }
}

impl Display for CreditCardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Credit card error: Could not retrieve credit card.")
    }
}

impl Error for CreditCardError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            CreditCardError::InvalidInput(_) => None,
            CreditCardError::Other(e, _) => {
                Some(e.as_ref())
            }
        }
    }
}

fn get_credit_card_info(
    credit_cards: &HashMap<&str, &str>,
    name: &str,
) -> Result<Card, CreditCardError> {
    let card_string = credit_cards.get(name).ok_or(CreditCardError::InvalidInput(format!("No credit card was found for {name}.")))?;

    let card = parse_card(card_string).map_err(|e| {
        CreditCardError::Other(Box::new(e), format!("{name}'s card could not be parsed."))
    })?;

    Ok(card)
}

pub fn credit_card_cli() {
    env_logger::init();

    let credit_cards = HashMap::from([
        ("Amy", "1234567 12 16 123"),
        ("Tim", "1234567 0616 123"),
        ("Bob", "1234567 Dec 08 123"),
    ]);

    println!("Enter Name: ");

    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    
    let result = get_credit_card_info(&credit_cards, name.trim());

    match result {
        Ok(card) => println!("\nCredit Card Info: {card:?}"),
        Err(err) => {
            match &err {
                CreditCardError::InvalidInput(msg) => println!("{msg}"),
                CreditCardError::Other(_, _) => println!("\nSomething went wrong! Try again!")
            }

            log::error!("\n{err:?}");
        },
    }
}