use bfrs::BF;
use std::env::args;
use std::fs::File;
use std::io::Read;

fn main() {
    let file_path = args().nth(1).unwrap();
    let mut file = File::open(file_path).unwrap();
    let mut buf = Vec::new();
    let _ = file.read_to_end(&mut buf);
    let mut bf = BF::new(buf);
    bf.interpret();
}
