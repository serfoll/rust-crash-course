fn greet(name: String) {
    println!("Hello, {name}!")
}

fn main() {
    // stack
    let x = 10;
    let y = x;
    println!("x = {x}, y = {y}");

    // Heap
    let name1 = "John".to_string();
    let name2 = name1; // moving name1 to name2. name2 now has ownership of name1

    // println!("Hello, {name1}"); // will throw a error (borrow of moved value: `name1` value borrowed here after move)
    println!("Hello, {name2}");

    // reference
    let s1 = "Hello".to_string();
    let s2 = &s1;

    println!("s1 = {s1}, s2 = {s2}");

    // mutable reference

    let a = String::from("Foo");
    greet(a);
}
