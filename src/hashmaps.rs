// Hashmaps need to be imported first.
use std::collections::HashMap;

// Hashmaps are basically like dictionaries in python.
// The following function creates a new hashmap.
// Note that "HashMap::new()" is a generic "HashMap".
// The return value in this case is what specifies datatypes of the Key:Value pair.
// The following function effectively translates to "HashMap::<String , String>::new()".
pub fn hashmap_string_string() -> HashMap<String , String> {
    HashMap::new()
}

pub fn functions_on_hashmaps() {
    let mut map = hashmap_string_string();
    map.insert("Key".to_string(), "Value".to_string());
    
    // Putting the value of a "HashMap" element into a variable can be done in the following way.
    // This way, just like python does it, is actually unsafe because if the string in the "[]" is not valid, it'll give an error.
    // Note that the value from the "HashMap" is actually borrowed, but no ownership is taken. 
    let _s1 = &map[&"Key".to_string()];

    // Theoretically, the following line shouldn't work, but it does!
    // Why? It's because rust allows a "String" to borrow a "&str".
    // Alternatively, this also works:
    let _s2 = &map["Key"];
    
    // To have full ownership of the value, the value of this key needs to be cloned.
    let _s3: String = map["Key"].clone();


    // A safer way to assign the value to a variable is as follows.
    // This method returns None if the key is invalid and is safe.
    // It will not cause any errors.
    // Note that optionals will be discussed in a later module.
    let _s4 = map.get("Key");

    // An alternative to not using optionals is using the "match" keyword.
    // Yes "match" is the keyword for switch statements, but it also has some predefined keywords to use in some situations.
    // The following basically means:
        // "Get the value of the key in the HashMap."
        // "If something is found in this value, print it."
        // "If nothing is found in this value, print 'Not Found'."
    match map.get("Key"){
        Some(value) => println!("Value is -> {value}"),
        None => println!("Not Found"),
    }


    // Keys and values can be iterated upon.
    // The following piece means the following:
        // "In every iteration of the for loop, get the references of the key and value from the HashMap reference '&map'."
        // "Then put the references as is in 'k' and 'v' respectively."
    // Note that the map needs to be referenced.
    for (k , v) in &map {
        println!("{k} : {v}");
    }

    // So why does the code above work but the following commented code doesn't?
    // The "&" in "&k" and "&V" below mean the following:
        // "In every iteration of the for loop, get the references of the key and value from the HashMap reference '&map'."
        // "Then get the contents of this referenced key and value."
        // "Then **COPY** the contents of key and value."
        // "Then finally store them inside 'k' and 'V' respectively."
    // The following code does not work because of the COPYING that happens.
    // Remember that for some value to be copied, it needs to be of type "copy", such as ints.
    // "String" is not of type copy.
    /*
    for (&k , &v) in &map {
        println!("{k} : {v}");
    }
    */

    // Now if the "HashMap" is actually of type "&str, &str", the code that failed above will work now.
    // This is because having the "&mapstr" will get a reference to "&str" which will be "&&str".
    // When the "&k" and "&v" enter to get their assigned values, they will say:
        // "De-reference the '&&str' for me (which will be &str, a pointer having 'copy')."
        // "Then 'copy' the value of &str (the pointer itself) and put it in 'k' and in 'v'."
    // so "k" and "v" will be of type &str.
    let mut mapstr: HashMap<&str, &str> = HashMap::new();
    mapstr.insert("Key", "Value");
    for (&k , &v) in &mapstr{
        println!("{k} : {v}");
    }
    
    // The following "entry" function gets the entire entry with the specified key.
    // The ".entry" function will own the string created from ""Key".to_string()".
    // The value of the ".entry" function will only be a reference to the entire entry/row in the map.
    // Thus, the entire entry's (in the "HashMap") ownership will not be moved into the variable "entry".
    // The variable "entry" (under the hood) will just have a reference to the entry in the "HashMap".  
    let entry = map.entry("Key".to_string());
    println!("{}" , entry.key());

    // entries can be matched to do some task if they are occupied.
    match entry {
        std::collections::hash_map::Entry::Occupied(value) => {
            println!("{}" , value.get());
        }
        _ => println!("Not found")
    }

    // Entries can be entered if the key is absent.
    // Even though this can be done by a line that checks and a line that inserts, this can be done in a single line.
    // If the key exists, "Value2" will be discarded.
    // If it doesn't exist, then the entry will be created and the "HashMap" will take ownership of the entire entry.
    // The key is stored in the map’s internal memory, not tied to the "entry" variable’s lifetime.
    map.entry("Key2".to_string()).or_insert("Value2".to_string());

    
    map.remove(&"Key".to_string());
}

// Hashmaps can contain Structs.
// Note that the structs created can only be used in HashMaps if they are hashable.
// To make sure that the "HashMap" hashes the struct, three traits need to be derived (will get into traits in a later module):
    // Hash
    // Eq
    // PartialEq
// Without deriving them, a compile error will be thrown.
// This wasn't required by "String" because RUST already implements these traits in the definition of the "String"
#[derive(Hash, Eq, PartialEq)]
pub struct Person {
    name: String,
    age: u8,
}

pub fn structs_in_hashmaps() {
    let mut h_map: HashMap<Person , &str> = HashMap::new();
    h_map.insert(Person { name: "Khan".to_string(), age: 20 }, "test");
}