// Traits in Rust are similar to Interfaces or Protocols.
// Traits are a shared functionality, and similar to interfaces, Traits can be used to ensure that a functionality is implemented for a struct.
// Some Traits are built in and public such as the "Debug" trait.
// To use a Trait, the following syntax format is used:
    // "#[derive(<trait_name>)]"
// multiple traits can be put within the same "derive" function separated by a comma ",".

// Say that a person struct is created and instanciated and the instance needs to be printed as debug values.
// If the struct is created without deriving the debug trait, Rust will complain, such as in the following code-block
/*
struct Person {
    _first_name: String,
    _last_name: String,
    _age: u8,
}

// Try to create a "Person" and print its values as debug values without deriving the "Debug" Trait.
pub fn print_person(){
    let p1 = Person{
        _first_name: "Khann".to_string(),
        _last_name: "Khalili".to_string(),
        _age: 30 
    };
    
    println!("{:?}" , p1);
}
*/

// In order to fix it, the traits need to derived just before creating the struct.
#[derive(Debug)]
pub struct Person {
    _first_name: String,
    _last_name: String,
    _age: u8,
}

// Try to create a "Person" and print its values as debug values after deriving the "Debug" Trait.
pub fn print_person(){
    let p1 = Person{
        _first_name: "Khann".to_string(),
        _last_name: "Khalili".to_string(),
        _age: 30 
    };
    
    println!("{:?}" , p1);
    println!("{}" , p1.full_name())
}

// Rust also allows the user to create custom traits.
pub trait HasFullName {
    fn full_name(&self) -> String;
}

// Any struct implementing the Trait will need to conform to that trait.
// When the "HasFullName" trait is applied to struct "Person" for example, this means that any instance of type "Person" should conform to "HasFullName".
// The struct "Person" will need to implement the function "full_name()" to fulfill the 'contract'.
impl HasFullName for Person {
    fn full_name(&self) -> String {
        format!("{} {}" , self._first_name , self._last_name)
    }
}

// Rust has a very cool feature in which it allows a user to implement a trait that defines methods that return an instance of the type that implements the trait.
// This means that the instance can be created with the following format:
    // "let <variable> = <struct_name>::new_instance(<Parameter>);"
    // "new_instance" can be renamed to any other name.
pub trait InitWithFullName {
    fn new_instance(full_name: &str) -> Self;
}

impl InitWithFullName for Person {
    fn new_instance(full_name: &str) -> Self {
        let names: Vec<&str> = full_name.split(" ").collect();

        Person {
            _first_name: names[0].to_string(),
            _last_name: names[1].to_string(),
            _age: 0
        }
    }
}

pub fn new_person(){
    let p1 = Person::new_instance("Khannn Khalili");
    
    println!("{:?}" , p1);
}


// The "fmt::Display" trait defines how a type should be formatted for human readability.
// It's intended for end-user output, like logs or messages.
// It's like the ".toString()" methods in other languages like Java.
use std::fmt;

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f , 
            "My First Name Is: {} \n
            My Last Name Is: {} \n
            My Age Is: {}" , 
            self._first_name , self._last_name , self._age)
        }
    }
    
    pub fn print_person_with_display(){
        let p1 = Person{
            _first_name: "Khan".to_string(),
            _last_name: "Khalili".to_string(),
        _age: 20
    };
    
    println!("{}" , p1);
}

// Traits can implement another traits using the "where" Keyword.
pub trait HasName {
    fn first_name(&self) -> String;
    fn last_name(&self) -> String;
}
pub trait PrintFullName where Self: HasName, {
    fn print_full_name(&self) -> String;
}

// To implement a trait within a trait, it uses a bit of generics.
// Generics will be explained later.
// This implementation is a generic trait implementation for "PrintFullName" for any type "T", so long as "T" implements the Trait "HasName".
impl<T> PrintFullName for T where T: HasName, {
    fn print_full_name(&self) -> String {
        format!("{} {}", self.first_name() , self.last_name())
    }
}

impl HasName for Person {
    fn first_name(&self) -> String {
        self.first_name()
    }
    fn last_name(&self) -> String {
        self.last_name()
    }
}