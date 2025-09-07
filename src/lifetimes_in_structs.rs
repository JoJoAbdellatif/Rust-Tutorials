// Lifetimes need to be specified in Rust "struct"s.
// Without these lifetimes, Rust will not know for how long should an attribute within the struct live.
// The following struct gives an error because it does not specify a lifetime.
/*
pub struct Person{
    name: &str,
}
*/

// In order to specify a lifetime for an attribute within the struct, the struct itself needs to have a lifetime specifier.
// The lifetime specifier in this case is "<'a>" but it can be anything else other than the "a".
// The attributes should at least live for as long as the lifetime of the struct itself

pub struct _Person <'a> {
    name: &'a str,
}

// Some people write the lifetime specifier as "'static" to make the attribute live for as long as the application is alive
pub struct _Person2 {
    name: &'static str,
}

// Lifetime specifiers also work for enums
pub enum _Animal<'a> {
    Dog { name: &'a str },
}