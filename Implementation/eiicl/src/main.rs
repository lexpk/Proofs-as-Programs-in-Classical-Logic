mod encoding;

use std::fs::File;
use std::io::Read;
use std::path::Path;

fn example(bytes: &[u8]) {
    let encoding = encoding::Encoding::from_tptp(bytes);
    for annotated in encoding.to_tptp() {
        println!("{}", annotated);
    }
}

fn main() {
    let path = Path::new("example.p");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => example(s.as_bytes()),
    }
}
