// Optional is a datatype, an enum to be exact in its source code, in rust that can either hold a value or "None".
// This is done because rust has no "Null value".
// So the variable can hold some value, or hold no value.
// Optionals can be represented for example as "Option<i32>" for ints.

pub fn optionals() {
    let _value = Some(10);
    let _maybe_string = Some("maybe a string?".to_string());
    let _maybe_string2: Option<String> = None;

    // The optional can be unwrapped safely when needed using a "match" statement.
    // A "match" statement is needed because without it, the variable may still be None which will make any unwrapping in this case very not safe.
    // The "match" is needed to handle handle the "None" case safely.
    // The ".clone()" in this specific example is used because the value of name will be moved in the unwrapping process, and it will be needed in the future. 
    let name = Some("Joey".to_string());
    match name.clone() {
        Some(name) => println!("Name is: {}." , name),
        None => println!("There is no name.")
    }

    // A name can be unwrapped in an unsafe way without a compile error.
    // This is done even though rust is expected to panic (give a runtime error or throw an exception) when a value is "None".
    // The ".expect(msg)" function is used to unwrap the value with a "msg" provided that will be displayed if rust panics.
    // Again, the ".clone()" function is used because "name" is needed in future code-blocks.
    let _unwrapped_value = name.clone().expect("name was not provided");

    // Unwrapping can also be forced without any options, but rust will panic again if the value is "None";
    let _unwrapped_value = name.clone().unwrap();

    // Optionals in rust can be mutable using the "mut" keyword.
    let mut maybe_mut_age: Option<i8> = Some(20);
    match maybe_mut_age.as_mut() {
        Some(maybe_mut_age) => *maybe_mut_age += 10,
        None => {}
    }

    println!("age = {}." , maybe_mut_age.unwrap());

    // Multiple optionals can be unwrapped with tuples.
    // All optionals must have a value.
    // This can be done using the "if let" keyword.
    // "if let" is a convenient shorthand for matching one specific pattern in an "enum", "Option", or "Result", without writing a full "match" statement.
    let age1 = Some(10);
    let age2 = Some(20);
    let age3 = Some(30);

    if let (Some(age_1) , Some(age_2) , Some(age_3)) = (age1 , age2 , age3) {
        println!("{}, {}, {}" , age_1 , age_2 , age_3);
        println!("{}" , age_1 + age_2 + age_3);
    }

    // Unwrapping optionals can have a set default option.
    // This option allows the default to be inserted in case the optional is still "None".
    let name: Option<&str> = None;
    let _unwrapped_name = name.unwrap_or("Joey");

    // If "name" is already set, then the unwrapped value in "name" will be used.
    let name: Option<&str> = Some("Joey");
    let _unwrapped_name = name.unwrap_or("JoJo");

    // If the optional value is "None", then a function can be used to do some work and return a value.
    // The function is inline and does not take any arguments.
    // Rust is smart enough to know the return datatype of the function to put in the variable.
    // Just because it is an inline function, it doesn't mean that the function is restricted to be 1 line.
    // It can be an entire code-block.
    let value: Option<i8> = None;
    let _unwrapped_value = value.unwrap_or_else(|| { 10 * 2 });

    // using ".is_some()" or ".is_none()" will return a boolean of whether an optional actually has a value or not.
    let _bl = value.is_some();
    let _bl = value.is_none();

    // Some variables have built-in default values such as ints and strings.
    // If the optional is "None", then the built-in default value will be used.
    let value: Option<i32> = None;
    let _unwrapped_value = value.unwrap_or_default();

    // An optional can be mapped as well while being safely unwrapped.
    // In this specific example, this is safe because the value of the optional is "None", meaning the closure (inline function) actually never runs and returns "None" immediately.
    // If the value had an actual value in it, then the closure will run.
    let value: Option<i32> = None;
    let mapped_value = value.map(|value| value * 2);
    println!("{}", mapped_value.unwrap_or_default());

}