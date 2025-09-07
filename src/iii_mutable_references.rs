// In rust a reference is by default immutable (read-only).
// The keyword "mut" has to be used explicitly to allow the mutation of the reference.

// To overwrite on a pass by reference, make the reference mutable.
// You can compare this with the function "borrow_string" in file "ii" of this module.
pub fn mut_borrow_string(value: &mut String) {
    value.clear();
}

pub fn demo_mut_borrow() {
    let _s1: String = String::from("Immutable String");
    let mut s2: String = String::from("String That Is Mutable");

    // The following commented block gives an error because the function expects a "&mut String" but only gets "&String".
    // Even when you add in the argument the "&mut" below, then it will still give a compilation error.
    // This is because "_s1" itself is not mutable.

    /*
    mut_borrow_string(&_s1);
    */

    // Even though "s2" is mutable, the following code block gives an error because function expects a "&mut String" but only gets "&String".

    /*
    mut_borrow_string(&s2);
    */

    // This is the correct way to call the function.
    // Notice that the variable "s2" is declared above as "mut".
    mut_borrow_string(&mut s2);
}

// Rust has a restriction, such that it allows only one mutable reference to a single variable.
// This is done because allowing 2 or more mutable refereces means that both can overwrite the variable, causing a race condition.

// For example the following piece of code gives a compilation error.
pub fn multi_mut_ref_error() {

    /*
    let mut x = 5;
    let r1 = &mut x;
    let r2 = &mut x;

    // both r1 and r2 exist at the same time now
    *r1 += 1;
    *r2 += 1;
    */
}

pub fn demo_mut_reference() {
    multi_mut_ref_error();
}

// Rust also allows 1 of the following but **NOT** both:
// A) any number of Immutable references.
// B) exactly one mutable reference.
// This makes sure that:
// 1) while something is mutably changing the data, no one else is allowed to even look at it.
// 2) while others are reading it, no one is allowed to change it.
pub fn demo_read_with_write() {
    // The following piece of code gives an error

    /*
    let mut s = String::from("hello");

    let r1 = &s;            // Immutable borrow
    let r2 = &mut s;        // Error: mutable borrow while immutable borrow is active
    */

    // The correct way of writing it is as follows.
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
        r1.push_str(" world");
    } // r1 ends here

    // Now we can borrow mutably again, such as in this code, or the mut can be replaced with a read-only reference.
    let r2 = &mut s;
    r2.push_str("!");
    println!("{s}");
}
