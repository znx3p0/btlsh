# Btl

[![docs](https://docs.rs/btl/badge.svg?version=0.2.4)](https://crates.io/crates/btl)

Btl is a simple library that makes shell scripting with rust easier.
It was originally written with the purposes of being used for build.rs files,
but it can be used for more complex purposes.

It's main premise is about integrating shell scripting with rust ergonomically.
This works both on windows and unix machines.

Btl is extremely simple, composed of six macros which have the exact same syntax.
These macros allows for embedding shell scripts to rust while allowing extreme
customization ergonomically.

Macros:

- shell!{}
- detach!{}
- execute!{}
- exec!{}
- cd!{}

NOTE:
btl uses sh as the unix backend, and powershell as the windows backend.

If you happen to find any bug, please raise an issue at the github repo.

Example:

```rust
fn shell() -> Result<(), std::io::Error> {
    // all macros use the try operator to avoid unwrapping
    // creates a detached process that may outlive the current process
    detach! {
        "touch m.txt";
        "sleep {}", 10;
        "rm m.txt";
    };

    // changes directory
    cd!("..").unwrap();
    let contents = execute! {
        "ls";
    };
    // returns a string containing the stdout of the process
    println!("contents {:?}", contents);

    // returns a bool indicating if the operation was successful
    if exec! {
        "ls";
    } {
        println!("success")
    } else {
        println!("not successful")
    }

    shell! {
        "ls {}", "-la";
    };

    Ok(())
}
```
