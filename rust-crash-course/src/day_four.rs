#![deny(clippy::all)]

#[derive(PartialEq)]
enum AnimalType {
    Dog,
    Cat,
    Rabbit,
    Horse,
}

enum Pet {
    Dog { name: String },
    Cat { name: String },
}

// Assiociated values with names
enum Shapes {
    Rectangle { width: f64, height: f64 },
    Circle { radient: f64, center: (f64, f64) },
}

// unnamed associated values
struct Size {
    width: f32,
    height: f32,
}

enum UnnamedShapes {
    Rectangle(f32, f32, Size),
    Circle(f32, f32, f32),
}

fn main() {
    let fluffy: AnimalType = AnimalType::Horse;
    match fluffy {
        AnimalType::Dog => println!("Woof!"),
        _ => println!("Something else!"),
    }

    let rectangle: Shapes = Shapes::Rectangle {
        width: 4.0,
        height: 5.0,
    };

    if let Shapes::Rectangle { width, height } = rectangle {
        println!("width: {width}, height: {height}");
    }

    match rectangle {
        Shapes::Rectangle { width, height } => {
            println!("w: {width}, h: {height}");
        }
        _ => println!("Not a rectangle"),
    }

    let rec: UnnamedShapes = UnnamedShapes::Rectangle(
        3.0,
        4.0,
        Size {
            width: 3.0,
            height: 5.0,
        },
    );
    if let UnnamedShapes::Rectangle(x, y, Size { width, height }) = rec {
        println!("{x}, {y}, {width}, {height}");
    }

    match rec {
        UnnamedShapes::Rectangle(x, y, Size { width, height }) => {
            println!("x:{x}, y:{y} size: w:{width}, h:{height}");
        }
        _ => println!("rec is not a rectangle"),
    }
    let pet_name = Pet::Dog {
        name: "Foo".to_string(),
    };
    let name = match pet_name {
        Pet::Dog { name } => name,
        Pet::Cat { name } => name,
    };
    println!(" name of pet: {name}");
}
