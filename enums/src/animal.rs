// Enumerations are useful to group related types.
// They are created using the "enum" keyword.
// An enum name is written as PascalCase.
// The enumerations themselves are also written in PascalCase.
// Each enum value is separated by a comma.

use std::default;

enum AnimalType {
    Dog,
    Cat,
    Rabbit,
    Snake,
}

pub fn enum_testing() {
    // Calling a value from the enum is done using the double colons.
    let animal_1 = AnimalType::Dog;

    // When comparing enum instances, an if-statement can be used, even though it is not recommended.
    // This will only work if you use the "#[derive(PartialEq)]" trait. (Traits will be in a later section).
    /*
    if animal_1 == AnimalType::Dog {
        println!("Animal is a dog!");
    }
    */

    // What is generally recommended for comparison (of course this is not always the case) is actually using a switch statement.
    // In a switch statement, the default is written with an underscore "_".
    match animal_1 {
        AnimalType::Cat => println!("Animal_1 is a cat!"),
        AnimalType::Dog => println!("Animal_1 is a Dog!"),
        AnimalType::Rabbit => println!("Animal_1 is a Rabbit!"),
        _ => println!("This seems Complicated!"),
    }
}
