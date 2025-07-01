#![deny(clippy::all)]

fn main() {
    let mut name: &str = "Foo"; // string slice type
    println!("Initial name: {name}!");
    name = "John";
    println!("Updated name: {name}!");

    let age: i32 = 30; // non immutable 32 bit integer type
    println!("Age is: {age}!");
    let new_age: i32 = -30; // can be positive or negative
    println!("New age is: {new_age}!");

    let mut total_likes: i32 = 1_000; // _ can be used for better readability in integers
    println!("Total likes: {total_likes}");
    total_likes += 10;
    println!("Updated total likes: {total_likes}");

    let green: i32 = 0xFF0000; // can also be hex values

    println!("Hex code for the color green: ({green})");
    let distante_in_km: f32 = 10.5; // 32 bit floating point value
    println!("distance {distante_in_km} km");

    let personal_data: (&str, i32) = ("John", 20);
    let (name, age) = personal_data;
    println!("Personal data: {name} is {age} years old!");

    let new_name = personal_data.0;
    let new_age = personal_data.1;
    println!("new_age: {new_age}, new_name: {new_name}");
}
