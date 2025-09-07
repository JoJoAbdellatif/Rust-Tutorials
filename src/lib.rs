// Rust is not like other languages in which exceptions are just thrown as a string.
// Rust has error types and an error thrown needs to be of that particular type.
// If a function can return either a value or an error, then the return type of the function should be of type "Result".
// There a lot of concepts to understand here.
pub fn errors() {
    // This is a simple "Result".
    // Saying "Result<&str , Box<dyn std::error::Error>>" means that the result can be a string slice, or an error that sits in the heap.
    // The keyword "Box" is used to reference a value in the heap (to be discussed later in the pointers chapter).
    // "Error" is a trait. (Traits will be discussed in the trait module)
    // The use of "Ok()" indicates that this is a correct value
    let value: Result<&str , Box<dyn std::error::Error>> = Ok("Hello World");

    // Results can be matched using the "match" keyword.
    // In case of "Ok()", the value will be printed.
    // In case of an "Err", the error will be printed.
    // note the "error" of type "Box dynamic".
    match value {
        Ok(value) => println!("{}" , value),
        Err(error) => println!("{error}")
    }

    // Exceptions can also be customized.
    // This is done by voiding the error segment such that it executes some piece of code when the value is not "Ok".
    let value: Result<&str , ()> = Err(());

    match value {
        Ok(value) => println!("{value}"),
        Err(_) => println!("Some Error Occured")
    }

    // Results can have values expected from them using the ".expect(msg)" function.
    // This piece of code creates an inline function that returns a "Result".
    let get_user_name = || -> Result<String , ()> {
        Ok("Khan".to_string())
    };

    // After that, the function is called then checked upon to see if it has a value or not.
    // If the function has an "Ok" value, the print statement will run.
    // If the function returns "Err" then the code will panic with the expected "msg".
    // The code will panic if the ".expect(msg)" returns an error
    let user_name = get_user_name().expect("Failed to get user_name.");
    println!("Hello, {user_name}");

    // The ".expect_err(msg)" is the opposite.
    // It expects the code to actually bring an error and will panic if the result is "Ok".
    let _error = get_user_name().expect_err("This should've been an error");
    
    // Results can be mapped to have some actions done on them.
    // The ".unwrap_or_default()" will unwrap to a default string if the unwrapping fails.
    let _upper_case = get_user_name().map(|name| name.to_uppercase()).unwrap_or_default();

    
    
    // Results can be checked whether they're an "Err" or "Ok" with ".is_err()" and ".is_ok" functions.
    let ok = get_user_name().is_ok();
    let err = get_user_name().is_err();
    println!("{ok} , {err}");
    
    // There is a cool feature in rust in which rust can trigger early exits in the code if an error is present.
    // As an example, consider the functions "get_first_name" , "get_last_name" , and "get_full_name".
    // The "get_full_name" function calls the "get_first_name" and "get_last_name" but fails early because the first name will be error.
    // This will make the error propagate until it is being matched using the "match" statement.
    let get_first_name = || -> Result<String , ()> {
        Err(())
    };
    
    let get_last_name = || -> Result<String , ()> {
        Ok("Khan".to_string())
    };
    
    // Note the "?" for the failed exits in the following code.
    let get_full_name = || -> Result<String , ()> {
        let f_name = get_first_name()?;
        let l_name = get_last_name()?;
        Ok(format!("{f_name} {l_name}"))
    };
    
    let full_name = get_full_name();
    match full_name {
        Ok(name) => println!("Hello , {name}"),
        Err(_) => println!("ERROR!")
    }
    
    // If the "?" is not present, then the code will continue even though the "get_first_name()" returns "Err".
    // The code will only give an error only if the value of "f_name" is unwrapped.
    let f_name = get_first_name();
    let l_name = get_last_name();
    
    println!("Debug f_name: {:?}", f_name);
    println!("Debug l_name: {:?}", l_name);
    
    println!("Still Works");  // still returns Ok!
    
    // Errors can be mapped as well.
    let get_something = || -> Result<String , String> {
        Err("Could Not Get That Thing!".to_string())
    };

    let name = get_something();
    let _error_length = name.map_err(|e| e.len());
}

 