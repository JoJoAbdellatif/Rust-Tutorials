// Rust supports asyncronous programming using asyn/await just like other pogramming languages, most notably being JavaScript and TypeScript.
// Rust has a twist in it, such that it provides built-in basics to write async programming, but it needs what is called a "Runtime" provided by 3rd-parties to utilize the full experience.
// There are many types of runtimes.

use std::time::Duration;

// These imports need to be used.
// They will be discussed below.
use futures::executor::block_on;
use futures::Future;
use tokio::time::sleep;

// Async functions return data of type "Future".
// The "Future" will have an "Output=<the return type specified in the function>".
async fn get_name() -> String {
    "Khan".to_string()
}


pub fn futures_testing() {
    // The following function call will not return an error; however, the printing will.
    // This is because any function called that is "async" needs to be awaited.
    let _name = get_name();
    /*
    println!("Hello, {}" , _name);
    */
    
    // To solve this, the "futures" crate needs to be installed. This is where the first dependency of the course will be installed.
    // To install the dependency, it just needs to be added to the Cargo.toml file.
    // It can also be installed using "cargo add futures" in a CMD.
    
    // Functions can be awaited on from the "futures" crate using the "block_on()" function.
    // The "block_on()" function will await, and unwrap the data coming to reveal the return type of the function called: "String" in this case.
    // To use the "block_on()", the import above at the beginning of the file needs to be done.
    let name_2 = block_on(get_name());
    println!("Hello, {}" , name_2);
}

// Since Async returns a "Future", then the function can return a "Future".
// async functions do not need to use "async fn".
// This also means that the "async" can be removed from the function.
fn get_name_future() -> impl Future<Output = String>{
    // Async code inside an async block does not run right away.
    // It is run when the function is awaited (the ".await" is called).
    // Any code before or after the async block is actually run immediately when the function is called, not when the function is awaited.
    println!("print: before async creation");

    // An async block can be used.
    let async_block = async {
        println!("print: before async execution");
        sleep(Duration::from_secs(1)).await;
        println!("print: after async execution");
        "1 second passed".to_string()
    };

    println!("print: After async creation");

    // The async block is called.
    async_block
}

pub async fn futures_future_testing(){
    // await needs to be called here and then the "String" will be unwrapped from the "Future".
    // The call changes from "block_on()" to ".await".
    let text_future = get_name_future().await;
    println!("{}" , text_future)

}