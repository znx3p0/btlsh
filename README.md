# Btl

Btl is a simple library that makes shell scripting with rust easier.
It was originally written with the purposes of being used for build.rs files,
but it can be used for more complex purposes.

It's main premise is about making shell scripting easier to work with rust.
This works both on windows and unix machines. Originally designed in linux,
not tested on Windows or Mac yet, but they should work since the library is platform-agnostic.

Btl is extremely simple. It's composed by three macros.

## btl!{}

This macro allows you to create custom commands a lá format!():

```rust
btl! {
    "echo {}", "Hello world!";
};

// Both of these examples are runnable on unix machines.
// Windows has not been tested yet, but it uses the cmd instead of sh.
btl! {
    "mkdir {} && sleep {} && rmdir {}", "xyz", 10, "xyz";
};

// This will cd the program backwards. Quotes are not needed.
btl! {
    cd ..;
};

// Changing directories within a btl will not change the program's directory.
btl! {
    "ls -la && cd .. && ls -la && pwd";
}

// As demonstrated by this.
btl! {
    "pwd";
}
// To change the directory you have to use the cd macro or use btl!{cd ..};
```

## shell!{}

This macro allows you to write complete shell scripts.

```rust

shell! {
    "mkdir xyz";
    "sleep 10";
    "rmdir xyz";
    "echo done!";
}
// It should be noted that you cannot use variables within this macro
```

## cd! {}

This macro is pretty straight forward. Changes the program's directory.

```rust
// Change the program's directory backwards
btl! {
    "pwd";
};

cd! [..];

btl! {
    "pwd";
};
```
