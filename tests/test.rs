#[cfg(test)]

mod tests {

    use btl::*;

    #[cfg(target_family = "windows")]
    #[test]
    fn windows() {
        shell! {
            "ls";
        };
        
        let curr = pwd!();
        println!("Current directory {}", curr);
        
        let foo = 5;
        detach! {
            "echo a > example.txt";
            "sleep {}" foo;
            "rm example.txt";
        };
        cd! {
            "./{}" "tests"
        };

        // Exec doesn't print to stdout since it directly uses the status. That can be changed.
        if exec! {
            "ls -la";
        } {
            println!("No errors executing command");
        } else {
            println!("Command not successful");
        }

        let out: std::process::Output = detailed_exec! {
            "pwd";
        };
        println!("{:#?}", out);

    }

    // #[cfg(target_family = "unix")]
    // #[test]
    fn linux() {
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
    }
}
