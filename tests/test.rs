





#[cfg(test)]
mod tests {
    use btl::*;

    #[test]
    fn shell() {

        let command = "mkdir";
        let arg = "xyz";
        btl! {
            "{} {}", command, arg;
        };

        
        btl! {
            "sleep 10";
        }
            
        btl! {
            "rmdir {}", arg;
        };

        shell! {
            "mkdir xyz";
            "sleep 10";
            "rmdir xyz";
        }

        btl! {
            "echo {}", 2;
        };

        btl! {
            cd ..
        };

        shell! {
            "ls -la";
            "pwd";
        };

        cd!{..};

    }

}
