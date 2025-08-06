// "Cell" in rust is another smart pointer that allows internal mutability.
// Mutability is allowed even if accessed by immutable references.
// "Cell" is intended for objects and datatypes that have the "copy" trait.
// Because "Cell" is internally mutable, it is generally considered unsafe in rust as it can introduce the possibility of memory unsafety if misused.

// To use "Cell", it needs to be imported.
use std::cell::Cell;

struct Person {
    name: String,
    age: Cell<u8>
} 

impl Person {
    fn increment_age(&self) {
        // To utilize the internal mutibility, the "Cell" works with getters and setters.
        // The following function call gets the value in the "Cell", adds 1 to it, and then sets the "Cell" to the resulting value.
        self.age.set(self.age.get() + 1);
    }
}

pub fn cell_testing() {
    let p1 = Person{
        name: "Khan".to_string(),
        age: Cell::new(20)
    };

    println!("{}" , p1.age.get());
    p1.increment_age();
    println!("{}" , p1.age.get());
}


// "RefCell" in rust is another pointer that allows multiple immutable borrows or 1 mutable borrow.
// It is considered not safe because it bypasses the borrow checks made during compile-time.
// Rust will still panic if it sees a mutable borrow while 1 or more immutable borrows are still active, but it'll panic as a runtime-error.

// "RefCell" needs to be imported first
use std::cell::RefCell;

pub fn ref_cell_testing(){
    // Remember the borrow checking allows for either 1 mutable borrow or many immutable borrows, but NOT both.
    // The following code is able to bypass the borrow check in compile-time, and thus, the code will run.
    // But then the code will panic during the runtime borrow check.
    let ref_cell = RefCell::new(vec![10 , 20 , 30]);
    let mut mut_borrow = ref_cell.borrow_mut();
    let _str_length = ref_cell.borrow().len();
    mut_borrow.push(100);
}