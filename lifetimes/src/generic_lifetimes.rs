// Lifetimes are essential to properly utilizing rust.
// Lifetimes specify the scope in which references live, such that when the code goes out of scope, the reference is destroyed along with the values.
// Rust uses lifetimes to prevent dangling references—that is, references to data that has already been dropped.
// Lifetimes are needed as they ensure that the data being referred to lives long enough and the data is not accidentally used after it's been freed.

// The following function gives a lifetime specifier error because no lifetime is specified.
// This will also fail because the returned reference will outlive the string.
// Remember that references are borrowed values.
// They do not own the string.
/*
pub fn lifetime_error_1() -> &str {
    &"Khannnn"
}
*/

// When the return type is changed to "String", it works because the return type is an actual instance and not a reference.
// This also doesn't return an error because the caller of the function will own the value returned from this function
pub fn lifetime() -> String {
    "Khannnn".to_string()
}

// When the return type is changed to "&String", it then fails because the return type is a reference and not an actual instance.
/*
pub fn lifetime_error_3() -> &String {
    &"Khannnn".to_string()
}
*/

// Starting off, the "'static" keyword (notice the apostrophe) indicates that the reference should be alive throughout the entirety of the application's lifetime.
// This means that "Khannnn" will be stored in a place in the heap, in such a way that will make the variable live until the application terminates.
pub fn static_lifetime() -> &'static str {
    "Khannnn"
}

// Take the following function.
// Generic Lifetimes describe how long references are valid, without tying them to specific values up front.
// The explanation is as follows (buckle up, this is a long one):
// "fn longest<'a>"
// This defines a function named longest, and it declares a generic lifetime parameter 'a.
// 'a is not a variable; it’s a label that tells Rust: “I’m going to describe how long some references must live, and I’ll use 'a as the name for that duration.”
// It could have been named anything, but convention is to use a single lowercase letter starting with 'a.
// "x: &'a str"
// This means x is a reference to a string slice (&str) that must live at least as long as lifetime 'a.
// In plain English: "x must remain valid for as long as 'a."
// "y: &'a str"
// Same goes for the "y" parameter.
// "-> &'a str"
// The function returns a reference to a string slice that also lives at least as long as 'a.
// Meaning: the returned value is one of the inputs, and it won’t outlive either one — it’s tied to the same lifetime 'a.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// There is a hidden rule in rust in which functions that take only one argument and that argument is "&str" and take only one output as "&str" do not need to have lifetimes specified.
// Even though the inputs and outputs are references, Rust basically just writes this lifetime code internally for this specific usecase.
fn _ret_y(y: &str) -> &str {
    &y
}

pub fn input_lifetime() {
    // The returned reference from the function "longest()" will be live only as long as the arguments live.
    // This is completely safe because rust assigns "&str" values by default to be "'static".
    // This means that the strings will actually outlive the references no matter what.
    let _longer_str = longest("Khannn", "Khalili");

    // Say that the same thing is done with a "&String".
    // This will also compile because the strings live until the end of the statement.
    let _longer_string = longest(&"Khannn".to_string(), &"Khalili".to_string());

    // Now try to print...
    // It'll immediately cause the line above to give a compilation error.
    // Why? This happens because the the strings, when declared within the parameters, will die right after the line finishes execution.
    /*
    println!("{}" , _longer_string);
    */

    // To get around this, the strings need to be declared from before, like the following.
    // The following works!
    let x = "long_string_1";
    let y = "longer_string_2";
    let longer = longest(x, y);
    println!("{}", longer);
}
