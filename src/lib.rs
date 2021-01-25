

#[macro_export]
macro_rules! btl {

    (cd $cmd: expr $(;)*) => {
        cd!($cmd);
    };

    ($cmd: expr, $($args:expr),*;) => {
        let command = format!($cmd $(,
            $args
        )*);

        if cfg!(target_os = "windows") {
            std::process::Command::new("cmd")
                    .args(&["/C", command.as_str()])
                    .spawn()
                    .unwrap()
                    .wait()
                    .unwrap();
        } else {
            std::process::Command::new("sh")
                    .arg("-c")
                    .arg(command)
                    .spawn()
                    .unwrap()
                    .wait()
                    .unwrap();
        };

    };

    ($cmd: expr $(;)*) => {
        if cfg!(target_os = "windows") {
            std::process::Command::new("cmd")
                    .args(&["/C", $cmd])
                    .spawn()
                    .unwrap()
                    .wait()
                    .unwrap();

        } else {
            std::process::Command::new("sh")
                    .arg("-c")
                    .arg($cmd)
                    .spawn()
                    .unwrap()
                    .wait()
                    .unwrap();
        };
    };
}

#[macro_export]
macro_rules! shell {

    ($first: expr; $($args: expr $(;)* )*) => {
        {

            let command = stringify!($first $(&& $args)*).to_string();

            let mut command = command.replace("\" && \"", " && ");
            command.pop();
            command.remove(0);

            if cfg!(target_os = "windows") {
                std::process::Command::new("cmd")
                        .args(&["/C", command.as_str()])
                        .spawn()
                        .unwrap()
                        .wait()
                        .unwrap();
            } else {
                std::process::Command::new("sh")
                        .arg("-c")
                        .arg(command)
                        .spawn()
                        .unwrap()
                        .wait()
                        .unwrap();
            };

        }
    };

}

#[macro_export]
macro_rules! cd {
    ($dir: expr) => {
        std::env::set_current_dir(stringify!($dir).to_string()).unwrap();
    }
}
