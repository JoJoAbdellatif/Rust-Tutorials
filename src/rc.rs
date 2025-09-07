// The "Rc" is another pointer.
// It stands for "Reference Counted".
// The type "Rc<T>" provides shared ownership of a value of type "T", allocated in the heap.
// Any object that can have a shared ownership disallows mutation.
// This means that these objects are "Read-Only", and the "Rc" pointer is no different.
// Once the value is created and wrapped, it is "Read-Only".
// Invoking "clone" on "Rc" produces a new pointer to the same value in the heap.
// "Rc" is intended for single threading purposes and cannot be passed around to multiple threads.
// "Rc" can have weak references assigned to it, but this is considered unsafe.
// It is unsafe because weak references will be invalid once the original object is destroyed, and calling the weak references will cause the app to crash.

// "Rc" needs to be imported first.
use std::rc::Rc;

fn _rc_pointers() {
    // To create an "Rc" reference, create the object first, and then wrap it with "Rc" with "Rc::new()".
    let name1 = "Khan".to_string();

    let reference1 = Rc::new(name1);

    // To get a weak reference, the ".downgrade()" method is called on the "Rc" object that was created.
    let weak_reference1 = Rc::downgrade(&reference1);

    
    /*
    // If the original reference is dropped, then the weak references will fail.
    drop(reference1);
    
    // Say, for example, the weak reference is to be upgraded later on.
    // Rust will panic (Runtime Error)
    // The ".upgrade()" function returns an optional, so it needs to be unwrapped to extract the value inside.
    let reference2 = weak_reference1.upgrade().unwrap();
    */

    // since the ".upgrade()" method returns an optional, it can be used in "match" statements.
    match weak_reference1.upgrade() {
        Some(rc) => println!("{:?}" , rc),
        _ => println!("None"),
    }

    // The cloning is as written in the introduction of this file.
    // This creates a fully new reference-counted that points to the same value in the heap as "reference1".
    // Below are 2 ways to clone the "Rc" object. Both work totally fine.
    let reference3 = reference1.clone();
    let reference4 = Rc::clone(&reference1);

    // Dropping "reference1" and printing the value in "reference3" here will completely work fine.
    // This works because both references are not tied to each other.
    drop(reference1);
    println!("{:?}" , reference3);

    drop(reference3);
    drop(reference4);

}