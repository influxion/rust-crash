#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::collections::HashMap;
use std::collections::HashSet;

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

    match a.get(3) 
    {
        Some(x) => println!("a[3] = {}", x),
        None => println!("error, no such element")
    }

    for x in &a { println!("{}", x); }

    a.push(44);
    println!("{:?}", a);

    // Some(77)
    let last_elem = a.pop();
    println!("last elem is {:?}, a = {:?}", last_elem, a);

    // let Some(last_value) = a.pop(); // this does not work because it may not be Some

    while let Some(x) = a.pop() 
    {
        println!("{}", x)
    }
}

pub fn hashmap() {
    let mut shapes = HashMap::new();
    shapes.insert(String::from("triangle"), 3);
    shapes.insert(String::from("square"), 4);

    shapes.insert("square".into(), 5);
    println!("{:?}", shapes);

    shapes.entry("circle".into()).or_insert(1);
    {
        let actual = shapes
            .entry("circle".into())
            .or_insert(2);
        *actual = 0;
    }
    println!("{:?}", shapes)
}

pub fn hashset() {
    let mut greeks = HashSet::new();
    greeks.insert("gamma");
    greeks.insert("delta");
    println!("{:?}", greeks);
    
    greeks.insert("delta");
    println!("{:?}", greeks);
    
    let added_vega = greeks.insert("vega");
    if added_vega {
        println!("we added vega! hooray!")
    }
    
    if !greeks.contains("kappa") {
        println!("we dont have kappa")
    }
    
    let removed = greeks.remove("delta");
    if removed {
        println!("we removed delta")
    }
    println!("{:?}", greeks);

    let _1_5: HashSet<_> = (1..=5).collect();
    let _6_10: HashSet<_> = (6..=10).collect();
    let _1_10: HashSet<_> = (1..=10).collect();
    let _2_8: HashSet<_> = (2..=8).collect();

    // subset
    println!("is {:?} a subset of {:?}? {}", _2_8, _1_10, _2_8.is_subset(&_1_10));
    
    // disjoint = no common elements
    println!("is {:?} disjoint of {:?}? {}", _1_5, _6_10, _1_5.is_disjoint(&_6_10));
    
    // union, intersection (present in both)
    println!("items in either {:?} and {:?} are {:?}", _2_8, _6_10, _2_8.union(&_6_10));
    
    // difference = 
    println!("items in {:?} that are not in {:?} are {:?}", _2_8, _6_10, _2_8.difference(&_6_10));

    // symmetric_difference = union - intersection
    println!("items in {:?} and items in {:?} that are different are {:?}", _2_8, _6_10, _2_8.symmetric_difference(&_6_10));



}

pub fn iterators() {
    let mut vec = vec![3, 2, 1];
    let mut vec2 = vec![1, 2, 3];
    vec2.extend(vec);
    println!("{:?}", vec2);
    // println!("{:?}", vec)
    // iter_mut, iter, iter().rev()
}






