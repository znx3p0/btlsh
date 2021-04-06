/// runs shell commands
#[macro_export]
macro_rules! async_shell {
    () => {};
    ($($t:tt)*) => {
        {
            $crate::__tokio_internal_builder!()
                .arg(
                    $crate::__internal_command_builder!($($t)*)
                ).spawn()?.wait_with_output().await?
        }
    };
}

/// creates a detached process
#[macro_export]
macro_rules! async_detach {
    () => {};
    ($($t:tt)*) => {
        {
            $crate::__tokio_internal_builder!()
                .arg(
                    $crate::__internal_command_builder!($($t)*)
                )
                .spawn()?
        }
    };
}

/// returns a string of the stdout of the process
#[macro_export]
macro_rules! async_execute {
    () => {};
    ($($t:tt)*) => {
        {
            let p = $crate::__tokio_internal_builder!()
                .arg($crate::__internal_command_builder!($($t)*))
                .output().await?;

            std::string::String::from_utf8_lossy(
                p.stdout.as_slice()
            ).to_string()
        }
    };
}

/// returns a bool indicating if the process was successful
#[macro_export]
macro_rules! async_exec {
    () => {{}};
    ($($t:tt)*) => {
        $crate::__tokio_internal_builder!()
            .arg($crate::__internal_command_builder!($($t)*))
            .output()
            .await?
            .status
            .success()
    };
}
