// Rust doesn't allow a reference to live past the lifetime of the variable itself.
// Rust detects it and gives a compilation error such as the following piece of code.

/*
pub fn dangle() -> &String {
    let s = String::from("hello");
    &s // returning reference to local variable!
}
*/

pub fn no_dangle() -> String {
    let s = String::from("hello");
    s // ✅ move ownership instead of borrowing
}

pub fn demo_dangle() {
    /*
    let h = dangle();
    */

    let _s: String = no_dangle();
}
