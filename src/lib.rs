use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::{parse_macro_input, Expr, Token};

struct Commands {
    fmt: Vec<String>,
    args: Vec<String>,
}

struct CD {
    fmt: Expr,
    args: Vec<Expr>,
}

impl Parse for Commands {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut fmt: Vec<String> = vec![];
        let mut args: Vec<String> = vec![];
        // Parse all commands loop

        'cmd: loop {
            let cmd = match input.parse::<Expr>() {
                Ok(cmd) => cmd,
                Err(_) => break,
            };

            let mut current_expr = vec![];

            loop {
                if input.parse::<Token![;]>().is_ok() {
                    fmt.push(quote! {#cmd}.to_string());
                    args.push(quote! {#(,#current_expr)*}.to_string());

                    continue 'cmd;
                }

                current_expr.push(input.parse::<Expr>().unwrap());
            }
        }

        Ok(Commands {
            fmt: fmt,
            args: args,
        })
    }
}

impl Parse for CD {
    fn parse(input: ParseStream) -> Result<Self> {
        let fmt: Expr = input.parse().unwrap();

        let mut args: Vec<Expr> = vec![];
        // Parse all commands loop

        loop {
            match input.parse::<Expr>() {
                Ok(cmd) => args.push(cmd),
                Err(_) => break,
            }
        }

        Ok(CD { fmt, args })
    }
}

/// The shell!{} macro allows for easy integration of the shell for rust.
/// It is designed to be ergonomic, easy to use and understand.

/// All other macros are based of this macro's syntax since it's easy to use,
/// and easy to customize.
///
///
/// Syntax:
/// You have to call the macro followed by a command.
/// This command can be used as a format! format string
///
/// Variables you use in the format string need to be after
/// the command and need to be separated by spaces. No commas.
/// Commands are separated by semicolons and they're obligatory.
///
/// ```
/// use btl::shell;
/// let foo = "test";
///
/// shell! {
///     "pwd";
///     "cd ..";
///     "pwd";
///     "echo {} > example.txt" foo;
/// };
/// ```
///
///
/// It's important to understand this syntax because all other macros this crate includes
/// are based off this syntax.
/// All other macros are slight variations of this macro,
/// but the use cases differ from relatively the same as this macro,
/// to completely different use cases, such as the detach!{} macro.
///
/// The main differences between these macros compared to the shell macro are:

/// - detach!{}: completely detaches the shell process to be run from the rust process.
/// - execute!{}: returns the stdout as a String.
/// - exec!{}: returns a bool if the command was successful.
/// - detailed_exec!{}: returns the output of the whole command as std::process::Output.
/// - cd!{}: changes the rust's process directory. This is important to note since all other macros live on their own shell.
///
#[proc_macro]
pub fn shell(input: TokenStream) -> TokenStream {
    if input.to_string().len() == 0 {
        return "".to_string().parse().unwrap();
    }

    let Commands { fmt, args } = parse_macro_input!(input as Commands);

    let key = std::time::SystemTime::elapsed(&std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let args = args.join("");
    let mut fmt = fmt.join(format!(" {}", key).as_str());

    if cfg!(target_os = "windows") {
        fmt = fmt.replace(format!("\" {}\"", key).as_str(), ";");
    } else {
        fmt = fmt.replace(format!("\" {}\"", key).as_str(), " && ");
    }

    let out;
    if cfg!(target_os = "windows") {
        out = format!(
            "
std::process::Command::new(\"powershell\")
.args(&[\"-C\", format!({}{}).as_str()])
.spawn()
.unwrap()
.wait()
.unwrap();
",
            fmt, args
        );
    } else {
        out = format!(
            "
std::process::Command::new(\"sh\")
.arg(\"-c\")
.arg(format!({}{}))
.spawn()
.unwrap()
.wait()
.unwrap();
",
            fmt, args
        );
    }

    out.to_string().parse().unwrap()
}

/// This is equivalent to shell in syntax and in execution.
/// The difference is that this shell process is completely separated
/// from the rust process and can outlive the rust process.     
/// This is exceptionally useful for creating programs which outlive the main process.
///
/// This macro returns an std::process::Child
///
/// Try running this example.
///
/// ```
/// use btl::detach;
///
/// detach! {
///     "touch example.txt";
///     "sleep {}" 10;
///     "rm example.txt";
/// };
///
/// ```
///
#[proc_macro]
pub fn detach(input: TokenStream) -> TokenStream {
    if input.to_string().len() == 0 {
        return "".to_string().parse().unwrap();
    }

    let Commands { fmt, args } = parse_macro_input!(input as Commands);

    let key = std::time::SystemTime::elapsed(&std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let args = args.join("");
    let mut fmt = fmt.join(format!(" {}", key).as_str());

    if cfg!(target_os = "windows") {
        fmt = fmt.replace(format!("\" {}\"", key).as_str(), ";");
    } else {
        fmt = fmt.replace(format!("\" {}\"", key).as_str(), " && ");
    }

    let out;
    if cfg!(target_os = "windows") {
        out = format!(
            "
{{
std::process::Command::new(\"powershell\")
.args(&[\"-C\", format!({}{}).as_str()])
.spawn()
.unwrap()
}}
",
            fmt, args
        );
    } else {
        out = format!(
            "
{{
std::process::Command::new(\"sh\")
.arg(\"-c\")
.arg(format!({}{}))
.spawn()
.unwrap()
}}
",
            fmt, args
        );
    }

    out.to_string().parse().unwrap()
}

// Returns stdout string
/// It's the same as all macros, but it returns the stdout as a String.

/// This is useful for getting information from cli tools back to rust ergonomically.

/// If you do not understand the syntax of this macro, please read the docs from shell!{}
/// as it provides the same interface but has different purposes.
///
/// ```
/// use btl::execute;
///
/// let contents = execute! {
///     "ls -la";
/// };
///
/// println!("Current directory's contents: {}", contents);
/// ```
#[proc_macro]
pub fn execute(input: TokenStream) -> TokenStream {
    if input.to_string().len() == 0 {
        return "".to_string().parse().unwrap();
    }

    let Commands { fmt, args } = parse_macro_input!(input as Commands);

    let key = std::time::SystemTime::elapsed(&std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let args = args.join("");
    let mut fmt = fmt.join(format!(" {}", key).as_str());

    if cfg!(target_os = "windows") {
        fmt = fmt.replace(format!("\" {}\"", key).as_str(), ";");
    } else {
        fmt = fmt.replace(format!("\" {}\"", key).as_str(), " && ");
    }

    let out;
    if cfg!(target_os = "windows") {
        out = format!(
            "
{{
    std::string::String::from_utf8_lossy(
        std::process::Command::new(\"powershell\")
        .args(&[\"-C\", format!({}{}).as_str()])
        .spawn()
        .unwrap().wait_with_output().unwrap().stdout.as_slice()
    ).to_string()
}}
",
            fmt, args
        );
    } else {
        out = format!(
            "
{{
    std::string::String::from_utf8_lossy(
        std::process::Command::new(\"sh\")
        .arg(\"-c\")
        .arg(format!({}{}))
        .spawn()
        .unwrap().wait_with_output().unwrap().stdout.as_slice()
    ).to_string()
}}
",
            fmt, args
        );
    }

    out.to_string().parse().unwrap()
}

// Returns status as bool
/// It's the same as all macros, but it returns a bool indicating if the command succeded.

/// If you do not understand the syntax of this macro, please read the docs from shell!{}
/// as it provides the same interface but has different purposes.
///
/// ```
/// use btl::exec;
///
/// if exec! {
///     "ls -la | grep Cargo";
/// } {
///     println!("Cargo found");
/// } else {
///     println!("Cargo not found");
/// }
/// ```
///
#[proc_macro]
pub fn exec(input: TokenStream) -> TokenStream {
    if input.to_string().len() == 0 {
        return "".to_string().parse().unwrap();
    }

    let Commands { fmt, args } = parse_macro_input!(input as Commands);

    let key = std::time::SystemTime::elapsed(&std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let args = args.join("");
    let mut fmt = fmt.join(format!(" {}", key).as_str());

    if cfg!(target_os = "windows") {
        fmt = fmt.replace(format!("\" {}\"", key).as_str(), ";");
    } else {
        fmt = fmt.replace(format!("\" {}\"", key).as_str(), " && ");
    }

    let out;
    if cfg!(target_os = "windows") {
        out = format!(
            "
{{
    std::process::Command::new(\"powershell\")
    .args(&[\"-C\", format!({}{}).as_str()])
    .output().unwrap().status.success()
}}
",
            fmt, args
        );
    } else {
        out = format!(
            "
{{
    std::process::Command::new(\"sh\")
    .arg(\"-c\")
    .arg(format!({}{}))
    .output().unwrap().status.success()
}}
",
            fmt, args
        );
    }

    out.to_string().parse().unwrap()
}

// Returns complete output
/// It's the same as all macros, but it returns a std::process::Output

/// If you do not understand the syntax of this macro, please read the docs from shell!{}
/// as it provides the same interface but has different purposes.
///
///```
/// use btl::detailed_exec;
///
/// let out: std::process::Output = detailed_exec! {
///     "pwd";
/// };
/// println!("{:#?}", out);
///
/// ```

#[proc_macro]
pub fn detailed_exec(input: TokenStream) -> TokenStream {
    if input.to_string().len() == 0 {
        return "".to_string().parse().unwrap();
    }

    let Commands { fmt, args } = parse_macro_input!(input as Commands);

    let key = std::time::SystemTime::elapsed(&std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let args = args.join("");
    let mut fmt = fmt.join(format!(" {}", key).as_str());

    if cfg!(target_os = "windows") {
        fmt = fmt.replace(format!("\" {}\"", key).as_str(), ";");
    } else {
        fmt = fmt.replace(format!("\" {}\"", key).as_str(), " && ");
    }

    let out;
    if cfg!(target_os = "windows") {
        out = format!(
            "
{{
    std::process::Command::new(\"powershell\")
    .args(&[\"-C\", format!({}{}).as_str()])
    .spawn()
    .unwrap().wait_with_output().unwrap()
}}
",
            fmt, args
        );
    } else {
        out = format!(
            "
{{
    std::process::Command::new(\"sh\")
    .arg(\"-c\")
    .arg(format!({}{}))
    .spawn()
    .unwrap().wait_with_output().unwrap()
}}
",
            fmt, args
        );
    }

    out.to_string().parse().unwrap()
}

/// The cd macro changes the directory of the current rust process.
/// This does not work for shell or any of the other macros.
/// Only works as a helper macro to make it easier to use cd inside rust.
/// You can still use all normal commands, including cd inside all macros.
/// They don't change the rust's process directory.
/// That's this macro's purpose.
///
///```
/// use btl::cd;
///
/// let dir = "..";
/// cd!{
///     "{}" dir
/// };
///
///
/// cd!{
///     ".."
/// };
///
///```
///
#[proc_macro]
pub fn cd(input: TokenStream) -> TokenStream {
    if input.to_string().len() == 0 {
        return "".to_string().parse().unwrap();
    }

    let CD { fmt, args } = parse_macro_input!(input as CD);

    let p = quote! {
        std::env::set_current_dir(format!(#fmt #(,#args)*).as_str()).unwrap();
    };

    TokenStream::from(p)
}

/// The pwd macro gives back the current working directory. Nothing fancy.
///
/// ```
/// use btl::pwd;
///
/// let curr = pwd!();
/// println!("Current working directory: {}", curr);
///
/// ```
///
#[proc_macro]
pub fn pwd(_: TokenStream) -> TokenStream {
    TokenStream::from(quote! {
        std::env::current_dir().unwrap().to_string_lossy().to_string()
    })
}
