


use btl::{cd, detach, exec, execute};

#[test]
fn t() -> Result<(), std::io::Error> {
    let _p = detach! {
        "touch m.txt";
        "sleep {}", 10;
        "rm m.txt";
    };

    cd!("{}", "..").unwrap();

    let p = execute! {
        "ls";
    };

    if exec! {
        "ls";
    } {
        println!("success")
    } else {
        println!("not successful")
    }

    println!("conts {:?}", p);
    Ok(())
}

