use std::env;
use std::error::Error;
use std::fs::File;

use gen_data::generate;

fn print_help() {
    println!("Usage: cargo run <output_file> <card> <size>\n");
    println!("Arguments:");
    println!("- output_file - the file data will be saved to");
    println!("- card - the cardinality of the underlying dataset");
    println!("- size - the total size of the dataset");
    println!("Example:");
    println!("cargo run data_1000_100000.txt 1000 100000");
}

fn parse_args() -> Result<(File, usize, usize), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect::<Vec<_>>();
    if args.len() != 4 {
        print_help();
        return Err("Incorrect number of arguments provided".into());
    }

    let out = File::create(&args[1])?;
    let card = args[2].parse::<usize>()?;
    let size = args[3].parse::<usize>()?;

    if card > size {
        return Err("dataset size has to be at least the size of its cardinality".into());
    }

    Ok((out, card, size))
}

fn main() -> Result<(), Box<dyn Error>> {
    let (mut out, card, size) = parse_args()?;
    generate(&mut out, card, size)?;

    Ok(())
}
