use std::env;

use ebcdict_converter::ebcdic_hex_to_ascii;

fn main() {
    let hex_string = parse_args(env::args().collect());
    println!("{}", ebcdic_hex_to_ascii(hex_string));
}

fn parse_args(args: Vec<String>) -> String {
    
    if args.len() <= 1 {
        panic!("Must enter a string for conversion")
    }
       
    
    let mut hex_string = "".to_string();

    for arg in &args[1..] {
        hex_string += arg;
    }

    hex_string
}