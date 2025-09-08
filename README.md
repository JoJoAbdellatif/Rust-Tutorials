# Rust Basics Repo

This is a Repository for learning the basics of rust. It contains as many pieces of code as possible to be explored. This markdown discusses rust installation and writing the helloworld program.

These tutorials are based mostly on [Vandad Nahavandipoor's playlist](https://www.youtube.com/playlist?list=PL6yRaaP0WPkWRsXJgdnw9lj1vchAaKwfS) on Youtube.

## Install Rust on VScode

1) ### prerequisites
    - VScode installed
    - Visual studio installer (installer only) installed
    - Visual studio build tools installed with "Desktop development with C++"
        - Download File: https://aka.ms/vs/17/release/vs_BuildTools.exe
        - This is used for linking the libraries to the binary in the building process

2) ### installation
    - Install the `rust-analyzer` VScode extension:
        - go to extensions
        - type "rust-analyzer"
        - choose the verified one by rust-lang.org
        - install
    - Install `rustup`:
        - download file: https://www.rust-lang.org/tools/install
        - run the installation to install the standard packages
        - If Rust analyzer does not work yet, then cargo needs to be added to system ENVs:
            1) Type in the start menu "Edit system environment variables".
            2) Go to system variables.
            3) Edit the system path.
            4) Add the following path: "%USERPROFILE%\\.cargo\\bin"
    - Download clippy (for verbose stuff):
        - in a CMD, write `rustup component add clippy`.

## Hello, World

1) ### Create New Workspace
    - Open a new file in the project root called `Cargo.toml` (with a Capitalized "C").
    - Note: DO NOT NAME THE ROOT FOLDER AS "Code". THIS DOES NOT INITIALIZE THE WORKSPACE PROPERLY.
    - Insert the following in it:
        ```
        # toml

        # Specifying that this is a workspace to add multiple projects/subprojects. 
        [workspace]

        # Adding the projects/subprojects in the workspace to be read by rust.
        # Each member name written in this members section should be the same as the folder name for this specific member.
        # In rust folders are named in snake_case.
        members = [
            "hello_world",
        ]

        # The resolver is the algorithm version used to read and use dependencies.
        resolver = "3"

        ```
2) ### Create New SubProject
    - Open a Command Prompt and cd into the root folder (the same folder that has the `Cargo.toml`).
    - Write the following to create the new `hello_world` subproject:
        ```
        cargo new hello_world

        # or to not create a git repo within the subproject:

        cargo new hello_world --vcs none
        ```
    - Go to the hello_world folder and go to the `Cargo.toml` file in it. It should look like the following:
        ```
        [package]
            name = "hello_world"
            version = "0.1.0"
            edition = "2024"
        ```
3) ### Writing The Program
    - Now writing the code should be pretty simple. The `main.rs` file looks like the following:
        ```
        // This line enables clippy so that you get more verbose and much needed warnings for clean code.
        #![deny(clippy::all)]

        fn main() {
            println!("Hello, World");
        }

        ```

    - The `clippy` line:
        ```
        // rust

        #![deny(clippy::all)]
        ```
        - `#![...]` is an inner attribute, applying to the entire crate.
        - `deny(...)` makes all specified lints into hard compiler errors (not just warnings).
        - `clippy::all` refers to a group of all standard Clippy lints.
        - This line tells the compiler:
            - If any Clippy lint in the `clippy::all` group is triggered, fail compilation with an error.

4) ### Handling Comments In This Repo
    - The way comments are handled will be as follows:
        ```
        // This is a literal rust comment

        /*
        This is a piece of code that is giving errors but intentionally left in the code files for learning purposes.

        You can remove these comments to see why they're giving compile-time/runtime errors.
        */ 
        ```
