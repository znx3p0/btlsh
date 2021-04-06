
mod sync;
#[cfg(feature = "tokio_shell")]
mod tokio;

#[macro_export(crate)]
macro_rules! __internal_command_builder {
    () => {};
    (
        $first_fmt: expr $(, $first_args: expr)*; $($fmt: expr $(, $args: expr)*;)*
    ) => {
        
        if cfg!(target_os = "windows") {
            format! (
                concat! (
                    $first_fmt,
                    $(";", $fmt,)*
                )
                $(, $first_args)*
                
                $(
                    $(, $args)*
                )*
            )
        } else {
            format! (
                concat! (
                    $first_fmt,
                    $(" && ", $fmt,)*
                )
                $(, $first_args)*
                
                $(
                    $(, $args)*
                )*
            )
        }

    }
}

#[macro_export(crate)]
macro_rules! __std_internal_builder {
    () => {
        std::process::Command::new(
            if cfg!(target_os = "windows") {
                "powershell"
            } else {
                "sh"
            }
        )
            .arg(
                if cfg!(target_os = "windows") {
                    "-C"
                } else {
                    "-c"
                }
            )
    }
}

#[cfg(feature = "tokio_shell")]
#[macro_export(crate)]
macro_rules! __tokio_internal_builder {
    () => {
        tokio::process::Command::new(
            if cfg!(target_os = "windows") {
                "powershell"
            } else {
                "sh"
            }
        )
            .arg(
                if cfg!(target_os = "windows") {
                    "-C"
                } else {
                    "-c"
                }
            )
    }
}

#[macro_export]
macro_rules! cd {
    () => {};
    ($first_fmt: expr $(, $first_args: expr)*) => {
        std::env::set_current_dir(format!($first_fmt $(, $first_args)*))
    }
}
