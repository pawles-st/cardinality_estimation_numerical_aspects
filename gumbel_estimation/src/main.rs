use gumbel_estimation::{GHLL, GHLLPlus, GHLLReal};
use std::collections::hash_map::RandomState;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

pub fn load_data(card: usize, size: usize) -> Result<Vec<u64>, io::Error>
{
    let file = File::open(format!("../data/data_{}_{}.txt", card, size))?;
    let reader = BufReader::new(file);

    reader.lines().map(|l| {
        l.and_then(|l| l.trim().parse::<u64>()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
        )
    }).collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    const NO_REGISTERS: u8 = 8;

    let builder = RandomState::new();
    let data = load_data(100_000, 10_000_000)?;

    {
        let mut ghll = GHLL::<_>::with_precision(NO_REGISTERS, builder.clone()).unwrap();
        for d in data.iter() {
            ghll.add(&d);
        }
        println!("GHLL (geo): {}", ghll.count_geo());
        println!("GHLL (har): {}", ghll.count_har());
    }
    
    {
        let mut ghllr = GHLLReal::<_>::with_precision(NO_REGISTERS, builder.clone()).unwrap();
        for d in data.iter() {
            ghllr.add(&d);
        }
        println!("GHLL Real (geo): {}", ghllr.count_geo());
        println!("GHLL Real (har): {}", ghllr.count_har());
    }

    {
        let mut ghllp = GHLLPlus::<_>::with_precision(NO_REGISTERS, builder.clone()).unwrap();
        for d in data.iter() {
            ghllp.add(&d);
        }
        println!("GHLL Plus: {}", ghllp.count());
    }

    Ok(())
}
