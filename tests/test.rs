#[cfg(test)]

mod tests {

    use btl::shell;
    use btl::detach;
    
    #[test]
    fn test(){


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
        detach! {
            "touch example.txt";
            "sleep {}" bar;
            "rm example.txt";
        };
    }
}
