use esaxx_rs::suffix_rs;
use std::env::args;
use std::fs;

fn main() {
    // Prints each argument on a separate line
    let args: Vec<_> = args().skip(1).collect();
    let filename = &args[0];

    let string = fs::read_to_string(filename).unwrap();
    for (c, u) in suffix_rs(&string).unwrap().iter() {
        println!("{c:?} : {u}");
    }
    let (count, _) = (suffix_rs(&string).unwrap().iter().count(), "Rust");
    println!("Found {count} nodes");
}
