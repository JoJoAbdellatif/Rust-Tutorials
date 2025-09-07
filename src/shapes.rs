use std::f64::consts::PI;

// Enums in Rust are very flexible because enum values can have attributes that define each enum value.
// It is kind of like adding a structure in an enum.
// Here, the values in "Shapes" enum is defined using curly braces.
enum Shapes{
    Circle {center: (f64 , f64) , radius: f64},
    Rectangle {width: f64 , height: f64},
}

// Enums also have implemenetations similar to how structs have implementations.
impl Shapes {
    fn calc_area(&self) -> f64 {
        // This block shows off a really cool featur in rust enums.
        // The values that are returned from the switch statement, can be assigned to a variable.
        // Most programming languages can return the values from the switch statements using the return keyword.
        // It is very rare, however, to see that a switch statement that can actually be assigned to a variable.
        let area = match self {
            Shapes::Circle { center: _, radius } => PI * radius * radius,
            Shapes::Rectangle { width, height } => width * height,
        };

        area
    }
}

pub fn enum_testing() {
    // This codeblock create an enum with associated values "width" , "height".
    let rect = Shapes::Rectangle { 
        width: 2.0,
        height: 4.0
    };

    // To compare enums with associated values, the "if let" statement is used.
    // The following piece checks if "rect" is of type rectangle or not.
    if let Shapes::Rectangle { width, height } = rect{
        println!("This shape is a Rectangle with width = {width}, height = {height}, and area = {}!" , rect.calc_area());
    }

    // These types of enums can also be matched with a switch statement.
    let circ = Shapes::Circle { center: (0.0 , 0.0), radius: 5.0 };

    match circ {
        Shapes::Circle { center, radius } => {
            println!("This shape is a circle with center = ({} , {}), radius = {} and area = {}!" , center.0 , center.1 , radius , circ.calc_area());
        },
        _ => println!("This is complicated!"),
    }

}


// Values in enums can also be defined using parentheses, making them tuples and unnamed.
// The associated values can also be a struct.
pub struct Size {
    width: f32,
    height: f32,
}

enum ShapesPar {
    // In this example, the first 2 values could be considered the center, while the last value could be considered the radius.
    _Circle(f32 , f32 , f32),

    // In this example, the first 2 values can be considered the top left corner of the rectangle.
    // The "Size" is the stuct holding the "width" and "height".
    Rectangle(f32 , f32 , Size),
}


pub fn enum_testing_unnamed() {
    // This segment creates a rectangle and the size struct and associates the struct with the rectangle enum.
    // The struct can be also written in the enum creation itself such as "let _rect = ShapesPar::Rectangle(1.0, 1.0, Size {width: 10.0 , height: 20.0});".
    let rect_size = Size {
        width: 10.0,
        height: 20.0};

    let rect = ShapesPar::Rectangle(
        1.0,
        1.0,
        rect_size
    );

    // checking that this instance of the unnamed enum is of type rectangle
    if let ShapesPar::Rectangle(_x, _y , Size { width:_, height:_ }) = rect {
        println!("This is a rectangle!");
    }

    // Again, this can also be matched with a switch statement.
    match rect {
        ShapesPar::Rectangle(_x, _y , Size { width:_, height:_ }) => {
            println!("This is a rectangle!");
        },

        _ => println!("This is complicated!"),
    }
}