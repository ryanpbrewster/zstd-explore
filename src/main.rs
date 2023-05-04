use std::{
    io::{self, BufRead},
    path::PathBuf,
};

use clap::Parser;
use zstd::bulk::Compressor;

#[derive(Parser)]
struct Args {
    #[arg(long)]
    input: PathBuf,

    #[arg(long, default_value_t = 0)]
    level: i32,

    #[arg(long, default_value_t = 1 << 20)]
    block_size: usize,

    #[arg(long, default_value_t = 1_024)]
    dict_size: usize,
}
fn main() -> anyhow::Result<()> {
    let args = Args::try_parse()?;

    let samples: Vec<String> = {
        let fin = std::fs::File::open(args.input)?;
        io::BufReader::new(fin).lines().collect::<Result<_, _>>()?
    };

    let mut uncompressed = Summary::default();
    for s in &samples {
        uncompressed.record(s.len());
    }
    println!("uncompressed: {:?}", uncompressed);

    let mut c = zstd::bulk::Compressor::new(args.level)?;
    let mut naive = Summary::default();
    for s in &samples {
        naive.record(c.compress(s.as_bytes())?.len());
    }
    println!(
        "naive: {:.2} {:?}",
        uncompressed.total as f64 / naive.total as f64,
        naive
    );

    let mut c = zstd::bulk::Compressor::new(args.level)?;
    let mut buf = Vec::with_capacity(args.block_size);
    let mut block = Summary::default();
    for s in &samples {
        if !buf.is_empty() && buf.len() + s.len() > args.block_size {
            block.record(c.compress(&buf)?.len());
            buf.clear();
        }
        buf.extend_from_slice(s.as_bytes());
    }
    if !buf.is_empty() {
        block.record(c.compress(&buf)?.len());
    }
    println!(
        "block: {:.2} {:?}",
        uncompressed.total as f64 / block.total as f64,
        block
    );

    let mut c = Compressor::with_dictionary(
        args.level,
        &zstd::dict::from_samples(&samples, args.dict_size)?,
    )?;
    let mut dict = Summary::default();
    for s in &samples {
        dict.record(c.compress(s.as_bytes())?.len());
    }
    println!(
        "dict: {:.2} {:?}",
        uncompressed.total as f64 / dict.total as f64,
        dict
    );

    Ok(())
}

#[derive(Default, Debug)]
struct Summary {
    count: usize,
    total: usize,
}
impl Summary {
    fn record(&mut self, v: usize) {
        self.count += 1;
        self.total += v;
    }
}
