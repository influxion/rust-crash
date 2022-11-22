#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

trait Vehicle: Paint {
    fn park(&self);
    fn get_default_color() -> String {
        "black".to_owned()
    }
}

trait Paint {
    fn paint(&self, color: String) {
        println!("painting object: {}", color)
    }
}

struct VehicleInfo {
    make: String,
    model: String,
    year: u16
}

struct Car {
    info: VehicleInfo
}

impl Vehicle for Car {
    fn park(&self) {
        println!("parking car!")
    }
}

impl Paint for Car {}

struct Truck {
    info: VehicleInfo
}

impl Truck {
    fn unload(&self) {
        println!("unloading truck.")
    }
}

impl Vehicle for Truck {
    fn park(&self) {
        println!("parking truck!")
    }
}

impl Paint for Truck {}

struct House {}

impl Paint for House {
    fn paint(&self, color: String) {
        println!("painting house: {color}")
    }
}

pub fn traits() {
    let car = Car {
        info: VehicleInfo { make: "Honda".to_owned(), model: "Civic".to_owned(), year: 2019 }
    };
    let house = House {};
    let object = create_paintable_object(true);

    let paintable_objects: Vec<&dyn Paint> = vec![&car, &house];

    paint_red(&car);
    paint_red(&house);
    paint_red(object.as_ref());

    paint_vehicle_green(&car);
    // paint_vehicle_green(&house);
    // paint_vehicle_green(&object);

}

// Trait bound
// fn paint_red<T: Paint>(object: &T) {
//     object.paint("red".to_owned());
// }
fn paint_red(object: &dyn Paint) {
    object.paint("red".to_owned());
}

fn paint_blue(object: &impl Paint) {
    object.paint("blue".to_owned());
}

fn paint_vehicle_green<T>(object: &T) where T: Vehicle {
    object.paint("green".to_owned());
}

fn create_paintable_object(vehicle: bool) -> Box<dyn Paint> {
    if vehicle {
        Box::new(Car {
            info: VehicleInfo { make: "Honda".to_owned(), model: "Civic".to_owned(), year: 2019 }
        })
    } else {
        Box::new(House {})
    }
}

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32
}

pub fn derive() {
    let p1 = Point { x: 3, y: 1 };
    let p2 = Point { x: 3, y: 1 };
    let p3 = Point { x: 5, y: 5 };

    println!("{:?}", p1);
    println!("{}", p1 == p2);
    println!("{}", p1 == p3);
}