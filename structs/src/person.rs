// Structs are written similar to C++.
// Rust used to have classes but now they are no longer available.
// Structs are written in PascalCase.
// Each field describing the struct is separated by a comma.
struct Person {
    name: String,
    gender: bool, // male is false , female is true (just for the sake of ease)
    age: u8,
}

pub fn instantiate() {
    // Creating an instance of the structure is as follows.
    // Instantiating while writing the "name" and "age" keywords (at least in this specific case) means that the order does not matter.
    let _man = Person {
        name: "Khann".to_string(),
        gender: false,
        age: 22,
    };
    // access the variables using the regular dot notation.
    println!("{} is {} years old.", _man.name, _man.age);

    // if the keywords and values have the same name such as:
    /*
        name: name,
        age: age,
    */
    // then the following "Field Init Shorthand" can be used. This following is an inline function but works as a regular function as well:
    let _woman = |name: String, gender: bool, age: u8| Person { name, gender, age };
    _woman("E".to_string(), true, 20);

    // Sometimes, a struct has 20 or 30 fields.
    // Say, for example, that 2 instances need to be created in which all 30 fields need to be initialized.
    // Instance A will be created regularly as shown in the previous steps.
    // Instance B is almost identical to Instance A, but Instance B has a few fields that have different values than Instance A.
    // This allows the use of something called "Struct Update Syntax".
    let instance_a = Person {
        name: "Khalili".to_string(),
        gender: false,
        age: 22,
    };

    // Instance B will then have someone who is also a male and 20 years old but he is someone else other than "Khalili".
    // The "Struct Update Syntax" is going to be used here.
    // It spreads the remaining fields and adds the values from fields in Instance A into Instance B.
    let _instance_b = Person {
        name: "John".to_string(),
        ..instance_a
    };

    // Of course Instance B can take the values of Instance A manually as well...
    let _instance_b = Person {
        name: "John".to_string(),
        gender: instance_a.gender,
        age: instance_a.age,
    };
}
