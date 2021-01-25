





#[cfg(test)]
mod tests {
    use btl::*;

    #[test]
    fn shell() {

        rsh! {
            "exa -la";
        };

        rsh! {
            cd ..

        };

        shell! {
            "echo 3";
            "echo 3";
            "echo 4";
        };

        cd![..];

    }

}
