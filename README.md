# Btl

![docs](https://docs.rs/btl/badge.svg?version=0.2.4)

Btl is a simple library that makes shell scripting with rust easier.
It was originally written with the purposes of being used for build.rs files,
but it can be used for more complex purposes.

It's main premise is making shell scripting easier to work with rust.
This works both on windows and unix machines. Originally designed in linux,
not tested on Windows or Mac yet, but it should work since the library is platform-agnostic.

Btl is extremely simple, composed of six macros which have the exact same syntax.
This macro allows for embedding shell scripts to rust naturally while allowing extreme
customization of the shell script to be in an ergonomic manner.

Macros:

- shell!{}
- detach!{}
- execute!{}
- exec!{}
- detailed_exec!{}
- cd!{}

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

// The pwd macro gives back the current working directory. Nothing fancy.
let curr = pwd!();
println!("Current dir through macro: {:?}", curr);

// The second macro is detach!{}
// This is equivalent to shell in syntax and in execution.
// The difference is that this shell process is completely separated
// from the rust process and can outlive the rust process.

// This is exceptionally useful for creating programs which outlive the main process.
detach! {
    "touch example.txt";
    "sleep {}" bar;
    "rm example.txt";
};

// The third macro is execute!{}
// It's got the same syntax as all macros, but it returns the stdout as a String
let contents = execute! {
    "ls -la";
};
println!("Current directory's contents: {}", contents);

// The fourth macro is exec!{}
// It's the same as all macros, but it returns a bool indicating if the command succeded.
if exec! {
    "ls -la | grep Cargo";
} {
    println!("Cargo found");
} else {
    println!("Cargo not found");
}

shell! {
    "pwd";
};

// The cd macro changes the directory of the current rust process.
// This does not work for shell or any of the other macros.
// Only works as a helper macro to make it easier to use cd inside rust.
// You can still use all normal commands, including cd inside all macros.
// They just don't change the rust's process directory.
// That's this macro's purpose.
let dir = "..";
cd! {
    "{}" dir
};

shell! {
    "echo CURRENT DIR $(pwd)";
};

cd! {
    "{}" curr
};
// println!("Now in the original directory");
shell! {
    "echo CURRENT DIR $(pwd)";
};

// The next macro is detailed_exec!{}
// It's the same as all macros, but it returns a std::process::Output
let out: std::process::Output = detailed_exec! {
    "pwd";
};
println!("{:#?}", out);
```

The shell!{} macro allows for easy integration of the shell for rust.
It is designed to be ergonomic, easy to use and understand.

All other macros are based of this macro's syntax since it's easy to use,
and easy to customize.
Syntax:
You have to call the macro followed by a command.
This command can be used as a format! format string
Variables you use in the format string need to be after
the command and need to be separated by spaces. No commas.
Commands are separated by semicolons and they're obligatory.

```rust
let foo = "test";
shell! {
    "pwd";
    "cd ..";
    "pwd";
    "echo {} > example.txt" foo;
};
```

It's important to understand this syntax because all other macros this crate includes
are based off this syntax.
All other macros are slight variations of this macro,
but the use cases differ from relatively the same as this macro,
to completely different use cases, such as the detach!{} macro.
The main differences between these macros compared to the shell macro are:

- detach!{}: completely detaches the shell process to be run from the rust process.
- execute!{}: returns the stdout as a String.
- exec!{}: returns a bool if the command was successful.
- detailed_exec!{}: returns the output of the whole command as std::process::Output.
- cd!{}: changes the rust's process directory. This is important to note since all other macros live on their own shell.

Contributions:

Feel free to contribute, add features or modify the macros.
This project is licensed under the MIT license.
