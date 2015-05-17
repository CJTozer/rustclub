use std::fs::File;
use std::io::Read;

fn main() {
    let mut data = String::new();
    read_data(&mut data);
}

fn read_data(data: &mut String) {
    // ## unwrap() means panic on fail - but not a useful output...
    let mut file = File::open("data/grid").unwrap();
    file.read_to_string(data).unwrap();
    println!("We read data: {}", data);
}
