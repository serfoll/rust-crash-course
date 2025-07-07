#![deny(clippy::all)]

// simple stuct
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
    family_name: String,
}

impl Person {
    fn describe(&self) {
        println!("Person: {} {}, {}", self.name, self.family_name, self.age);
    }
}

// Debug trait
#[derive(Debug)]
// tuple
struct Point(f64, f64, f64);

// implementation block
impl Point {
    // does not mutate
    fn describe(&self) {
        println!("Point is at ({}, {}, {})", self.0, self.1, self.2);
    }

    fn zero() -> Point {
        Point(0.0, 0.0, 0.0)
    }
}

impl Point {
    // does mutate the instance
    fn make_twice(&mut self) {
        self.0 *= 2.0;
        self.1 *= 2.0;
        self.2 *= 2.0;
    }

    // does not mutate and returns a new Point
    fn twice(&self) -> Point {
        Point(self.0 * 2.0, self.1 * 2.0, self.2 * 2.0)
    }
}

fn main() {
    let person: Person = Person {
        name: "John".to_string(),
        age: 30,
        family_name: "Doe".to_string(),
    };

    person.describe();

    println!("person 1: {person:?}");

    let person2: Person = Person {
        name: "Jane".to_string(),
        ..person
    };

    println!(
        "{} {} is {} years old.",
        person2.name, person2.family_name, person2.age
    );

    // tuples
    let mut points = Point(1.0, 2.0, 3.0);
    points.describe();

    points.make_twice();
    points.describe();
    println!("points = {points:?}");

    let point1 = Point::zero();
    println!("point = {point1:?}");
}
