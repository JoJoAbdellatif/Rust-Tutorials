// There are 2 main concepts to consider which form the basis of owning a variable and managing memory: Borrowing and Moving.
//  1) Borrowing: means that the value of the variable is just accessed without any transfer of ownership from one place to the other.
//  2) Moving: means that the value of the variable has had its ownership transferred from one place to the other.

// The following function does NOT work. It even gives a compilation error, not a runtime one.
// In rust, assigning "s1" to "s2" means that the ownership of the string in "s1" has been moved to "s2".
// This basically means that after assigning "s1" to "s2", "s2" now owns the string.
// So when it goes to the print statement, "s1" now has effectively become obsolete.
// This happens because when the dynamically allocated string is freed from the heap, it needs to be freed manually, and "s1" and "s2" cannot free the data at the same time.
// If rust allowed this, then a double deallocation would occur.
// Rust prevents this double deallocation from happening by removing ownership (in this specific case) from "s1" and making "s2" the owner of the string.

// A good illustration can be found at: https://doc.rust-lang.org/book/img/trpl04-02.svg

pub fn borrow_vs_moving_str() {
    /*
    let s1 = String::from("Khalili");
    let s2 = s1;

    println!("Hello, {s1}", );
    println!("Hello, {s2}", );
    */
}

// If the variables are changed to be integers, however, suddenly the code works...
// This is because integers and some other variables are not dynamically allocated in the heap and instead are stored on the stack.
// It allows rust to copy the value of the first variable into the second variable.
// This means that both variables are totally independent from each other.
pub fn borrow_vs_moving_int() {
    let age1 = 10;
    let age2 = age1;

    println!("You are {age1} years old");
    println!("You are {age2} years old");
}

// So how to make the "borrow_vs_moving_str()" function work?
// It can be corrected by making "s2" a reference of "s1", such that now "s2" points to the structure of the string in "s1".
// The reference "s2" actually sits in the stack.
// See a good reference image at: https://doc.rust-lang.org/book/img/trpl04-06.svg
pub fn borrow_vs_moving_str_correction() {
    let s1 = String::from("Khalili");
    let s2 = &s1;

    println!("Hello, {s1}");
    println!("Hello, {s2}");
}

pub fn code_block() {
    // Some code goes here for example

    // Sample Code Block:
    {
        let name = String::from("Khan");
        println!("hello, {name}, from code_block");
    }
    // "name" here, outside of the code block, gives a compile error because the code now has gone out of scope.
    // This means that "name" is no longer allocated.

    /*
    name;
    */
}
