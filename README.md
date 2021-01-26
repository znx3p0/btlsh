# Btl

Btl is a simple library that makes shell scripting with rust easier.
It was originally written with the purposes of being used for build.rs files,
but it can be used for more complex purposes.

It's main premise is making shell scripting easier to work with rust.
This works both on windows and unix machines. Originally designed in linux,
not tested on Windows or Mac yet, but it should work since the library is platform-agnostic.

Btl is extremely simple, composed of five macros which have the exact same syntax.
This macro allows for embedding shell scripts to rust naturally while allowing extreme
customization of the shell script to be in an ergonomic manner.

Macros:

- shell!{}
- detach!{}

Example:

```rust

let foo = 2;
let bar = 5;

// Syntax:
// You have to call the macro followed by a command.
// This command can be used as a format! format string

// Variables you use in the format string need to be after
// the command and need to be separated by spaces. No commas.
// Commands are separated by semicolons and they're obligatory.

shell! {
    "pwd";

    "cd ..";

    "pwd";

    "echo {} > example.txt" foo;
};

// The second macro is detach!{}
// This is equivalent to shell in syntax and in execution.
// The difference is that this shell process is completely separated
// from the rust process and can outlive the rust process.

// This is exceptionally useful for creating programs which outlive the main process.
detach! {
    "touch example.txt";
    "sleep {}" bar;
    "rm example.txt";
}


// The third macro is execute!{}

// It's got the same syntax as all macros, but it returns the stdout as a String

let dir = execute! {
    "ls -la";
};
println!("Current directory: {}", dir)


// The fourth macro is exec!{}
// It's the same as all macros, but it returns a bool indicating if the command succeded.
// This is useful for creating shell-based rust logic.
if exec! {
    "ls -la | grep Cargo";
} {
    println!("Cargo found");
} else {
    println!("Cargo not found");
}

// The fifth macro is detailed_exec!{}
// It's the same as all macros, but it returns a std::process::Output
let out: std::process::Output = detailed_exec! {
    "pwd";
};

println!("{:#?}", out);

```
