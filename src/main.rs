use std::env;

fn main() {
    if let Err(e) = auto_gen::run(&mut env::args()) {
        eprintln!("Error: {e}");
    }
}
