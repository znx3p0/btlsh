


use proc_macro::TokenStream;
use quote::{quote};
use syn::parse::{Parse, ParseStream, Result};
use syn::{parse_macro_input, Expr, Token};

struct Commands {
    fmt: Vec<String>,
    args: Vec<String>,
}


impl Parse for Commands {
    fn parse(input: ParseStream) -> Result<Self> {



        let mut fmt: Vec<String> = vec![];
        let mut args: Vec<String> = vec![];
        // Parse all commands loop
        
        'cmd: loop {
            let cmd = match input.parse::<Expr>() {
                Ok(cmd) => cmd,
                Err(_) => break
            };
            
            let mut current_expr = vec![];
            
            loop {
                if input.parse::<Token![;]>().is_ok() {

                    fmt.push(quote!{#cmd}.to_string());
                    args.push(quote!{#(,#current_expr)*}.to_string());

                    continue 'cmd;
                }

                current_expr.push(input.parse::<Expr>().unwrap());
            }
            


            
        }

        Ok(Commands{fmt: fmt, args: args})


    }
}

#[proc_macro]
pub fn shell(input: TokenStream) -> TokenStream {

    if input.to_string().len() == 0 {
        return "".to_string().parse().unwrap()
    }

    let Commands {
        fmt, args
    } = parse_macro_input!(input as Commands);

    let key = std::time::SystemTime::elapsed(&std::time::UNIX_EPOCH).unwrap().as_nanos();
    let args = args.join("");
    let fmt = fmt.join(format!(" {}", key).as_str());
    let fmt = fmt.replace(format!("\" {}\"", key).as_str(), " && ");

    let out;
    if cfg!(target_os = "windows") {
        out = format!("
std::process::Command::new(\"cmd\")
.arg(\"-c\")
.arg(&[\"/C\", format!({}{}).as_str()])
.spawn()
.unwrap()
.wait()
.unwrap();
", fmt, args);
    } else {
        out = format!("
std::process::Command::new(\"sh\")
.arg(\"-c\")
.arg(format!({}{}))
.spawn()
.unwrap()
.wait()
.unwrap();
", fmt, args);
    }

    out.to_string().parse().unwrap()
}


#[proc_macro]
pub fn detach(input: TokenStream) -> TokenStream {

    if input.to_string().len() == 0 {
        return "".to_string().parse().unwrap()
    }

    let Commands {
        fmt, args
    } = parse_macro_input!(input as Commands);

    let key = std::time::SystemTime::elapsed(&std::time::UNIX_EPOCH).unwrap().as_nanos();
    let args = args.join("");
    let fmt = fmt.join(format!(" {}", key).as_str());
    let fmt = fmt.replace(format!("\" {}\"", key).as_str(), " && ");

    let out;
    if cfg!(target_os = "windows") {
        out = format!("
std::process::Command::new(\"cmd\")
.arg(\"-c\")
.arg(&[\"/C\", format!({}{}).as_str()])
.spawn()
.unwrap();
", fmt, args);
    } else {
        out = format!("
std::process::Command::new(\"sh\")
.arg(\"-c\")
.arg(format!({}{}))
.spawn()
.unwrap();
", fmt, args);
    }

    out.to_string().parse().unwrap()
}

// Returns stdout string
#[proc_macro]
pub fn execute(input: TokenStream) -> TokenStream {

    if input.to_string().len() == 0 {
        return "".to_string().parse().unwrap()
    }

    let Commands {
        fmt, args
    } = parse_macro_input!(input as Commands);

    let key = std::time::SystemTime::elapsed(&std::time::UNIX_EPOCH).unwrap().as_nanos();
    let args = args.join("");
    let fmt = fmt.join(format!(" {}", key).as_str());
    let fmt = fmt.replace(format!("\" {}\"", key).as_str(), " && ");

    let out;
    if cfg!(target_os = "windows") {
        out = format!("
{{
    std::string::String::from_utf8_lossy(
        std::process::Command::new(\"cmd\")
        .arg(\"-c\")
        .arg(&[\"/C\", format!({}{}).as_str()])
        .spawn()
        .unwrap().wait_with_output().unwrap().stdout.as_slice()
    ).to_string()
}}
", fmt, args);
    } else {
        out = format!("
{{
    std::string::String::from_utf8_lossy(
        std::process::Command::new(\"sh\")
        .arg(\"-c\")
        .arg(format!({}{}))
        .spawn()
        .unwrap().wait_with_output().unwrap().stdout.as_slice()
    ).to_string()
}}
", fmt, args);
    }

    out.to_string().parse().unwrap()
}

// Returns status as bool
#[proc_macro]
pub fn exec(input: TokenStream) -> TokenStream {

    if input.to_string().len() == 0 {
        return "".to_string().parse().unwrap()
    }

    let Commands {
        fmt, args
    } = parse_macro_input!(input as Commands);

    let key = std::time::SystemTime::elapsed(&std::time::UNIX_EPOCH).unwrap().as_nanos();
    let args = args.join("");
    let fmt = fmt.join(format!(" {}", key).as_str());
    let fmt = fmt.replace(format!("\" {}\"", key).as_str(), " && ");

    let out;
    if cfg!(target_os = "windows") {
        out = format!("
{{
    std::process::Command::new(\"cmd\")
    .arg(\"-c\")
    .arg(&[\"/C\", format!({}{}).as_str()])
    .output().unwrap().status.success()
}}
", fmt, args);
    } else {
        out = format!("
{{
    std::process::Command::new(\"sh\")
    .arg(\"-c\")
    .arg(format!({}{}))
    .output().unwrap().status.success()
}}
", fmt, args);
    }

    out.to_string().parse().unwrap()
}


// Returns complete output
#[proc_macro]
pub fn detailed_exec(input: TokenStream) -> TokenStream {

    if input.to_string().len() == 0 {
        return "".to_string().parse().unwrap()
    }

    let Commands {
        fmt, args
    } = parse_macro_input!(input as Commands);

    let key = std::time::SystemTime::elapsed(&std::time::UNIX_EPOCH).unwrap().as_nanos();
    let args = args.join("");
    let fmt = fmt.join(format!(" {}", key).as_str());
    let fmt = fmt.replace(format!("\" {}\"", key).as_str(), " && ");

    let out;
    if cfg!(target_os = "windows") {
        out = format!("
{{
    std::process::Command::new(\"cmd\")
    .arg(\"-c\")
    .arg(&[\"/C\", format!({}{}).as_str()])
    .spawn()
    .unwrap().wait_with_output().unwrap()
}}
", fmt, args);
    } else {
        out = format!("
{{
    std::process::Command::new(\"sh\")
    .arg(\"-c\")
    .arg(format!({}{}))
    .spawn()
    .unwrap().wait_with_output().unwrap()
}}
", fmt, args);
    }

    out.to_string().parse().unwrap()
}
