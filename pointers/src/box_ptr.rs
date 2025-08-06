// A "Box" is a datatype that stores values and data in the Heap.
// Take the following example.
pub fn boxx() {
    // Ints are always initialized in the stack.
    let _a: u8 = 20;

    // However if an int is intended to be initialized in the heap, a box can be used.
    // Box is a pointer type that can be dereferenced to get the value inside.
    let x: Box<u8> = Box::new(100);

    // To dereference a pointer, it's just like C++.
    let mult_by_2 = *x * 2;
    println!("x times 2 = {}" , mult_by_2)
}