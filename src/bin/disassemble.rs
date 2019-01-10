extern crate emul8r;

use emul8r::op::Op;
use emul8r::error::Error;
use std::env;
use std::fs::File;

fn run() -> Result<(), Error> {
    let path = env::args().nth(1).ok_or(Error::Path)?;
    let mut file = File::open(&path)?;
    loop {
        println!("{:?}", Op::read_next(&mut file)?);
    }
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
    }
}
