use std::env;

enum Stage {
    Initial = 1,
    Options = 2,
    Confirmation = 3
}

fn main() {
    let stage: Stage = Stage::Initial;
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}
