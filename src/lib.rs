pub fn vars() {
    // immutable
    let _i: u32 = 6_000_000;
    let _x = 5;

    // Mutable
    let mut _y = 24;
    _y = 10;

    let _rgb: i32 = 0xFF0000;
    let _binary: i32 = 0b1100_1001;

    let _distance: f32 = 20.25;
    let _distance2 = 20000.21445;

    let distance3: f32 = 30.5;
    let distance4 = 1.5;
    let _sum = distance3 + distance4;

    // --------------------------------------------------------------------------------------------------------------------------- //
    // Strings

    // This is a pointer to the string.
    // The bytes of the string itself are stored in the read-only portion of the binary.
    // The pointer (reference and string length) is stored in the stack.
    // This string is static, having a lifetime throughout the entire duration of the execution.
    let _x: &'static str = "Hello, World";

    // This string, however, is not static. so its lifetime is until it goes out of scope.
    let _y: &str = "Hello";

    // If a string reference is assigned to be mutable, then the reference itself is mutable, not the literal string.
    // The string literal is baked in the read-only section of the binary.
    // The following example shows the variable x changing references from pointing at "hello" to pointing at "world".
    let mut _x: &str = "Hello";
    _x = "world";

    // Completely fine to do in rust
    let _name = "Joey";
    let _name = "JoJo";

    let _username = "Mo";
    let _username = 2;

    // If a string literal is mutated to twice or more, it's most likely that rust will only bake the literal into the read-only section only one time.
    let mut _y = "This is a string literal";
    _y = "Hello";
    _y = "This is a string literal";

    // The following is a dynamically allocated string. This string is made up of 4 parts.
    // The pointer to the string, length of the string, and capacity of the allocated data all reside in the stack.
    // The data itself (in this case, the bytes of the string) that the pointer points at is dynamically allocated in the heap.
    // A good illustration of dynamically allocated strings can be found at: https://doc.rust-lang.org/book/img/trpl04-01.svg
    let _dyn_alloc: String = String::from("This is a dynamically allocated string!!");

    // --------------------------------------------------------------------------------------------------------------------------- //
    // Constants

    // Cannot be changed throughout the entire execution of the program.
    const _THIS_IS_A_CONST: u8 = 22;

    // --------------------------------------------------------------------------------------------------------------------------- //
    // Tuples
    
    // Tuples are identified by the parenthesis. 
    let data: (&str, u8) = ("Joey", 22u8);
    let _xyz = data.1;

    // The following context unpacks the elements in "data" into "_name" and "_age".
    let (_name, _age) = data;

    // There is also the option to unpack specific elements. the following unpacks the second element of data into "_age_specific".
    // The first element will not be packed
    let (_ , _age_specific) = data;

    // --------------------------------------------------------------------------------------------------------------------------- //
    // Lists

    let list = [1, 2, 3, 4, 5];
    let _element = list[0];
}
