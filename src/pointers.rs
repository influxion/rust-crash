#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]

trait UIComponent {
    fn render(&self) {
        println!("Rendering component...");
    }
}

struct Button {
    text: String
}

impl UIComponent for Button {}

struct Container {
    name: String,
    child: Box<Container>
}

impl UIComponent for Container {}

pub fn box_smart_pointer() {
    let button_a = Button { text: "button a ".to_owned() };
    let button_b = Box::new(Button { text: "button a ".to_owned() });

    let button_c = button_a;
    let button_d = button_b;

    let components: Vec<Box<dyn UIComponent>> = vec![
        Box::new(button_c),
        button_d
    ];
    
}

// ---------------

use std::rc::Rc;
use std::cell::RefCell;

struct Database {
    max_connections: u32
}

struct AuthService {
    db: Rc<RefCell<Database>>
}

struct ContentService {
    db: Rc<RefCell<Database>>
}

pub fn rc_smart_pointer() {
    let database = Rc::new(RefCell::new(Database {
        max_connections: 100
    }));
    let auth_service = AuthService { db: Rc::clone(&database) };
    let content_service = ContentService { db: Rc::clone(&database) };

    // let mut r1 = database.borrow_mut(); // need to be careful in this situation as you can cause program to panic
    let r2 = database.borrow_mut();

    database.borrow_mut().max_connections = 200;

}

// -----------
use std::ops::{Deref, DerefMut};

struct MySmartPointer<T> {
    value: T
}

impl <T> MySmartPointer<T> {
    fn new(value: T) -> MySmartPointer<T> {
        MySmartPointer { value }
    }
}

impl<T> Deref for MySmartPointer<T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for MySmartPointer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

pub fn deref_coercion() {
    let mut s = MySmartPointer::new(Box::new("Jordon".to_owned()));
    // let s = &(***s);
    // &MySmartPointer -> &Box -> &String -> &str
    print(&mut s);
}

fn print(s: &str) {
    println!("{s}")
}