fn say_hello_world() -> String {
    String::from("Hello, world!")
}

fn say_hello_to() -> String {
    let to = "world".to_string();
    format!("Hello, {to}!") // uses litterals {} to format strring 
}

fn greet(name: String) {
    println!("Hello, {name}!");
}

fn add_one(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    let gretting = say_hello_world();
    println!("{gretting}");

    let hello = say_hello_to();
    println!("{hello}");

    let x = 10;
    let y = 12;
    let sum = add_one(x, y);
    println!("{x} + {y} = {sum}");

    let name = "John".to_string();
    greet(name);

    // inline function
    let times_two = |value: i32| value * 2;
    let result = times_two(2);
    println! {"2 x 2 = {result}"};

    // function pointers
    let ptr = times_two;
    let res = ptr(4);
    println!("4 x 2 = {res}");
}
