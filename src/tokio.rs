// These imports need to be used.
// They will be discussed below.
use tokio::runtime::Runtime;
use tokio::time::{sleep , Duration};

// Now, "block_on()" is not the only syntax that can be used.
// The "await" syntax can be used using a runtime called "tokio".
// "tokio" needs to be installed as a dependency.
// "tokio" needs to run as what is known as "tokio main application", meaning that the main function of the application needs to be a "tokio main function".
// A "tokio main function" is implemented with:
    // "#[tokio::main]" written right above the main function.

// In order to bypass this and have the application just partially running on tokio, a struct needs to be created that will take a tokio runtime instance as a field.
// Functions that will need implementation using "tokio" can be implemented within the runtime struct.
pub struct TokioLib {
    rt: Runtime,
}

impl TokioLib {
    pub fn new() -> Self {
        Self {
            rt: Runtime::new().unwrap(),
        }
    }

    pub fn run_async_task(&self) {
        self.rt.block_on(async {
            // This async block runs inside Tokio, isolated from the rest of the app.
            // A ".await" can be called here to await the sleep till it's done.
            sleep(Duration::from_secs(1)).await;
            let call_1 = "Khan".to_string();
            println!("Processed: {call_1}");

            sleep(Duration::from_secs(1)).await;
            let call_2 = "Khalily".to_string();
            println!("Processed: {call_2}")
        })
    }
}