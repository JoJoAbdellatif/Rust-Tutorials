// Iterators are lists containing elements that can be consumed.
// Iterators in RUST are lazy, meaning they need to be consumed to perform a certain task.
// In RUST, iterators use the "iterator" trait (to be discussed in the traits session).
// For loops in RUST understand that the objects inputted to them are iterators and it consumes it automatically.
// This means that the iterators do not need to be consumed manually in for loops
// In vectors, the ".iter()" function gives an iterator over the vector, which can be consumed.
// Note that the ".iter()" function works with references to the variables such as the "&i32" in the for loop.
pub fn iterating_over_vector () {
    let values = vec![1 , 2 , 3 , 4 , 5];
    
    for value in values.iter(){
        println!("{value}" , );
    }

    // Some functions can be performed on the iterator itself.
    // Say, for example, the sum of all values in the vector are needed.
    let iter = values.iter();
    let _sum: i32 = iter.sum();
    
    // Now that the pointer of "iter" is at the end of the vector, the iterator is now consumed.
    // If sum or any other function for this iterator is called, a compile_time error will be thrown.
    // Another iterator will need to be created for the "values" vector.
    /*
    let _sum2: i32 = iter.sum();
    */

    // Iterators can have their results mapped.
    let _values_map = values.iter().map(|v| v * 2);

    // Leaving the map like that will not return a vector. 
    // To return a vector, the "collect()" function should be used.
    let values_updated: Vec<i32>= values.iter().map(|v| v * 2).collect();
    println!("{:?}" , values_updated);


    // Using ".into_iter()" instead of ".iter()" will move the values themselves and will not work with references.
    // This moves the ownership from the vector entirely and the ".into_iter()" will own the collection.
    let values_updated: Vec<i32>= values.into_iter().map(|v| v * 2).collect();
    println!("{:?}" , values_updated);

}