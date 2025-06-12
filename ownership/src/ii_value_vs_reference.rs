// In rust, there is passing an argument by value and passing the argument by references.
// If a value is passed, rust will have 2 options.
// If the value being copied is an int or bool or a primitive data type, then the value is eligible for the "copy" trait.
// If the value, however, is not a primitive data type, then the value is not eligible to being copied.
// What happens in this case is that rust "moves" the ownership from the variable holding the data to the function that this data is being passed into.
// Note: Traits will be discussed in a separate section..

pub fn takes_ownership(s: String) {
    // Remember that String and &str are different data types.
    println!("This is a string with moved ownership: {}", s);
}

// When the ownership of a variable is "moved" to a function, the variable cannot be used after the function.
pub fn demo_ownership() {
    let s = String::from("hello");
    takes_ownership(s);
    /*
    println!("{}", s);      // error: value borrowed after move
    */
}

pub fn copied(i: i32) {
    println!("This is the copied value: {}", i)
}

// If a reference is passed, then rust just borrows the string as read only.
// Note that this is an immutable reference.
pub fn borrow_string(s: &String) {
    println!("This is a borrowed string: {}", s);
}
