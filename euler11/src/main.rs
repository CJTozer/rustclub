use std::fs::File;
use std::io::Read;

fn main() {
    // ## unwrap() means panic on fail - but not a useful output...
    let mut file = File::open("data/grid").unwrap();

    let data = &mut String::new();
    file.read_to_string(data);
    println!("We read data: {}", data);
}
