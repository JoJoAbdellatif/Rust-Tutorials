// Functions are written in snake_case.
// They are written in the form of:
    // <encapsulation> fn <function_name>(<arg1> , <arg2> , <etc...>) {
    //      code goes here...
    //      This function does not return anything
    // }

// They can also be written in the following form:
    // <encapsulation> fn <function_name>(<arg1> , <arg2> , <etc...>) -> () {
    //      code goes here...
    //      This function also does not return anything.
    //      The "-> ()"" is called a units type.
    // }

// They can also be written in the following form:
    // <encapsulation> fn <function_name>(<arg_1> , <arg_2> , <etc...>) -> String {
    //      code goes here...
    //      This function EXPECTS the return of a string.
    // }

// If the last statement in the function is written without a semicolon and without being assinged to a variable, it is returned automatically.
pub fn say_hello_world() -> String {
    // code goes here

    // This function returns "Hello World"
    String::from("Hello World!")
}

// Arguments are written in snake_case too.
pub fn say_hello_to(to_person: String) -> String {
    // code goes here

    // This function returns "Hello World"
    format!("Hello, {}!" , to_person)
}

// Functions can be passed as arguments in rust.
pub fn func_in_arg(function: fn(i32 , i32) -> i32){
    function(100 , 200);
}

pub fn func_testing() {
    let _hello = say_hello_world();

    say_hello_to(String::from("Joeyyy"));

    // Functions can be done inline instead of writing them on top.
    let _greeting = |name: &str| format!("Welcome to Inline, {}" , name);

    // Inline Functions can have multiple arguments.
    // Since the following function is a one line code, the curly braces that wrap the inline function can be removed. 
    let multiply_2_numbers = |x: i32 , y: i32| {
        x * y
    };
    
    multiply_2_numbers(10 ,20);

    // calling a function with an argument that is a function
    func_in_arg(multiply_2_numbers);

    // A pointer to a function can be created just like how pointers point to variables.
    let ptr = multiply_2_numbers;
    
    // This "ptr" can then be used as a function call now;
    ptr(20 , 40);


}

