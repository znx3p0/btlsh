# Btl

Btl is a simple library that makes shell scripting with rust easier.
It was originally written with the purposes of being used for build.rs files,
but it can be used for more complex purposes.

It's main premise is making shell scripting easier to work with rust.
This works both on windows and unix machines. Originally designed in linux,
not tested on Windows or Mac yet, but it should work since the library is platform-agnostic.

Btl is extremely simple, composed of two macros.
This macro allows for embedding shell scripts to rust naturally while allowing extreme
customization of the shell script to be in an ergonomic manner.

Macros:

- shell!{}
- detach!{}

Example:

```rust
use btl::shell;
use btl::detach;

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

// This is exceptionally useful for creating autoupdating programs.
// Try running this example locally. You'll probably find it interesting.
detach! {
    "touch example.txt";
    "sleep {}" bar;
    "rm example.txt";
};

// It should be noted that changing directories through either of these macros
// will not change the rust's programs directory. It only changes the shell's directory.

```
