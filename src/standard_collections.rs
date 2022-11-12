#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

pub fn vector() {
    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);
    
    println!("a = {:?}", a);
    
    a.push(44);
    
    println!("a = {:?}", a);

    // usize isize

    // let idx: i32 = 0; // this will not work must be usize
    let idx: usize = 0;

    a[idx] = 312;
    println!("a[0] = {}", a[idx]);

    match a.get(3) {
        Some(x) => println!("a[3] = {}", x),
        None => println!("error, no such element")
    }
}