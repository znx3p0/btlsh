/// runs shell commands
#[macro_export]
macro_rules! shell {
    () => {};
    ($($t:tt)*) => {
        {
            $crate::__std_internal_builder!()
                .arg($crate::__internal_command_builder!($($t)*)).spawn()?.wait_with_output()?
        }
    };
}

/// creates a detached process
#[macro_export]
macro_rules! detach {
    () => {};
    ($($t:tt)*) => {
        $crate::__std_internal_builder!()
            .arg($crate::__internal_command_builder!($($t)*)).spawn()?
    };
}

/// returns a string of the stdout of the process
#[macro_export]
macro_rules! execute {
    () => {};
    ($($t:tt)*) => {
        {
            let p = $crate::__std_internal_builder!()
                        .arg($crate::__internal_command_builder!($($t)*))
                        .output()?;

            std::string::String::from_utf8_lossy(
                p.stdout.as_slice()
            ).to_string()
        }
    };
}

/// returns a bool indicating if the process was successful
#[macro_export]
macro_rules! exec {
    () => {{}};
    ($($t:tt)*) => {
        $crate::__std_internal_builder!()
            .arg($crate::__internal_command_builder!($($t)*))
            .output()?.status.success()
    };
}
