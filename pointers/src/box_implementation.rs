// This file is here to show how the box is implemented internally.
// It also shows how custom boxes can be implemented.

// This creates a new struct that can take any value "T" and then puts it in "value".
struct RegularStruct<T> {
    value: T,
}

impl<T> RegularStruct<T> {
    fn new(value: T) -> Self{
        RegularStruct { value }
    }
}

pub fn age_in_reg_struct() {
    let age: RegularStruct<u8> = RegularStruct::new(21);

    // The value can be accessed totally fine.
    // But this is done without "deref()", which actually dereferences a pointer.
    println!("Value is: {}" , age.value);
}


// So what if the custom struct should behave like a "Box" in terms of it being a pointer type?
// The struct needs to implement the "Deref" trait from "std::ops::Deref".
// Doing this from the beginning, a struct can be created with the deref implementation.
// The "Deref" trait cannot be derived because the compiler has no idea which field or data in the struct to return in "deref()".
// This means that the "Deref" trait needs to be implemented in the struct.
use std::ops::Deref;
struct BoxedValue<T> {
    value: T,
}

impl<T> Deref for BoxedValue<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl <T> BoxedValue<T> {
    fn new(value: T) -> Self {
        BoxedValue { value }
    }
}

pub fn age_in_boxed_value() {
    let age_box: BoxedValue<u8> = BoxedValue::new(21);

    // The value can be accessed through dereferencing.
    println!("Value is: {}" , *age_box);

    // Try to call the ".deref()" function.
    // This will return a pointer to the "value" field in "BoxedValue" because the ".deref()" returns "&Self::Target", not the actual value.
    let _age_value = age_box.deref();

    // Since "age_box.deref()" returns a reference, this entire expression can be dereferenced using an "*".
    // Basically "*age_box" is the short hand for writing "*(age_box.deref())"
    println!("{} is the same as {}. The left value is the short hand for the right value." , *age_box , *(age_box.deref()))
}

// Rust is smart enough to dereference values when needed automatially.
// It implicitly helps in that process in certain cases.
// It is called Implicit Deref Coercion
// One of these cases is in functions.
// Try to print a reference in the function.
pub fn print(reference: &u8) {
    println!("{}" , reference);
}

// This can also work with "BoxedValue".
pub fn implicit_deref() {
    let boxed = BoxedValue::new(22);
    
    // Now by calling "print(&boxed)", rust implicitly calls the ".deref()" function and then gets a reference to the dereferenced value.
    // Rust does this to automate the process instead of having to dereference manually everytime.
    print(&boxed);
}