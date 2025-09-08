
// Traits can also be passed as arguments.
// Take for example the following 2 structs: "Cat" and "Dog".
pub struct Cat {
    name: String,
}

pub struct Dog {
    name: String,
}

// Now, any dog or cat should be able to speak (meow or bark).
pub trait Talk {
    fn speak(&self) -> String;
}

impl Talk for Cat {
    fn speak(&self) -> String {
        "Meow!".to_string()
    }
}

impl Talk for Dog {
    fn speak(&self) -> String {
        "Woof!".to_string()
    }
}

// Now easily, the "speak" function can be called twice on both animal instances.
pub fn create_animals() {
    let dog1 = Dog{
        name: "d1".to_string(),
    };

    let cat1 = Cat{
        name: "c1".to_string(),
    };

    dog1.speak();
    cat1.speak();

}

// However, what if the exact type of the input is not known?
// This can be addressed using a function that takes a trait as a parameter.
// Effectively, this means that:
    // "Any type that implements the trait ('Talk' in this example) will be accepted as an input, even if I don’t know its exact type."
pub fn make_speak(animal: &dyn Talk) -> String {
    animal.speak()
}

pub fn animals() {
    let dog1 = Dog{
        name: "d1".to_string(),
    };

    let cat1 = Cat{
        name: "c1".to_string(),
    };

    make_speak(&dog1);
    make_speak(&cat1);
}

// Multiple traits can be bound in a single function.
// This is done by using the keyword "where".
// It basically says:
    // "Accept any input which implements traits x + y + etc..."
pub trait CanRun {
    fn run(&self) -> String;
}

impl CanRun for Cat {
    fn run(&self) -> String {
        "runnnn!".to_string()
    }
}

impl CanRun for Dog {
    fn run(&self) -> String {
        "run!".to_string()
    }
}

pub fn make_run(animal: &dyn CanRun) -> String {
    animal.run()
}

pub fn print_behavior<T>(animal: T) where T: CanRun + Talk {
    println!("{} {}" , make_speak(&animal) , make_run(&animal));
}
