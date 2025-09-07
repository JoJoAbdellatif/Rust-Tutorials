// Vectors in rust are the same as arrays in other programming languages.

// A vector of a fixed size is written in the form of [datatype ; length].
// Note that the datatype and length are separated with a semi-colon ";".
pub fn vector_fixed_size() -> [String ; 2] {
    let values: [String ; 2] = ["Hello".to_string() , "world".to_string()];
    
    // getting the length of the vector is very simple.
    let _length = values.len();

    values
}

// Using loops to iterate over a vector is done by calling the "iter()" function in vectors:
pub fn iter_over_vector_loops() {
    for value in vector_fixed_size().iter(){
        println!("{value}");
    }
}

// calling specific values is done by calling the reference of the element being called.
pub fn specific_vector_element() {
    let values = vector_fixed_size();
    let hello = &values[0];
    println!("{hello}, {}" , &values[1]);
}

// Values in a vector can be mapped.
// For example, when iterating over a vector, each value in the vector can be mapped to be multiplied by 2:
// "map()" is conceptually similar to a for loop.
// However, as "map()" is lazy, it is best used when already working with other iterators.
// Note that "map()" returns a new map: it doesn't overwrite the old values.
pub fn mapping_iteration_vector() {
    let values: [i32; 2] = [10 , 20];


    let _iterated_values = values.iter().map(|x| x * 2);
}

// Vectors can be created with a shorthand typing.
// This is done using macros.
// Using this method creates a vector that does not have a fixed size.
// vectors are immutable by default, but this one will be mutable by using the "mut" keyword.
pub fn vector_shorthand() {
    let mut values = vec![1 , 2 , 3 , 4];
    // Because this does not have a fixed size, values can be added and removed from the vector.

    values.push(100);
    values.remove(1);

    println!("Values are {:?}" , values);
    values.clear();
    println!("Values are {:?}" , values);

}

// 2 vectors can be concatinated together:
pub fn concat_vectors() {
    let mut values = vec![1 , 2 , 3 , 4];
    println!("Values are {:?}" , values);
    values.extend_from_slice(&[10 , 11 , 12 , 13]);
    println!("Values are {:?}" , values);


    // it can also be done in the following way:
    let mut values1 = vec! [5 , 6 , 7 , 8];
    let mut values2 = vec! [15 , 16 , 17 , 18];

    println!("Values1 are {:?}" , values1);
    println!("Values2 are {:?}" , values2);
    values1.append(&mut values2);
    println!("Values1 are {:?}" , values1);
    println!("Values2 are {:?}" , values2);
}