#![allow(unused_variables)]

pub fn concrete_lifetimes() {
    let mut s1 = String::from("My name is Jordon");
    let r1 = &s1; // this lifetime carries to the next line
    println!("r1: {r1}");  // and it ends here and does not overlap with the lifetime below

    let r2 = &mut s1; // lifetime starts
    r2.push_str("!") // lifetime ends *no overlapping*
}
//------------
pub fn generic_lifetimes() {
    let player1 = String::from("Player 1");
    let result;
    {
        let player2 = String::from("Player 2");
    
        result = first_turn(player1.as_str(), player2.as_str());
    }
    println!("player going first is: {result}")
}

fn first_turn(p1: &str, p2: &str) -> &'static str {
    let s: &'static str = "Lets get rusty";
    s
}
//------------

struct Tweet<'a> {
    content: &'a str,
}

impl<'a> Tweet<'a> {
    fn replace_content(&mut self, content: &'a str) -> &str {
        let old_content = self.content;
        self.content = content;
        old_content
    }
}

pub fn struct_lifetime() {
    let mut tweet = Tweet {
        content: "example"
    };
    let old_content = tweet.replace_content("replace_example");
    println!("{old_content}");
    println!("{}", tweet.content);
}

// 1. Each param that is a reference gets its own lifetime param.
// 2. If there is exactly oen input lifetime param, that lifetime
//    is assigned to all output lifetime params.
// 3. If there are multiple input lifetime params, but one of them is
//    &self or &mut self, the lifetime of self is assigned to all output
//    lifetime params.

fn take_and_return_content<'a>(content: &'a str, content2: &'a str) -> &'a str {
    content
}