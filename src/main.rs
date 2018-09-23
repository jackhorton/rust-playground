use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut remaining = Vec::new();
    for arg in args {
        if arg.starts_with("-trace=") {
            let trace = &arg["-trace=".len()..];
            println!("tracing: {}", trace);
        } else if arg.starts_with("-testtrace=") {

        } else if arg.starts_with("-dump=") {

        } else if arg.starts_with("-") {

        } else {
            remaining.push(arg);
        }
    }

    println!("remaining: {:?}", remaining);
}