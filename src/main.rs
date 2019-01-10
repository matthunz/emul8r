mod cpu;
pub mod error;
pub mod op;

use self::error::Error;
use self::op::Op;
use clap::{App, Arg};
use std::fs::File;

fn run() -> Result<(), Error> {
    let matches = App::new("Emul8r")
        .arg(Arg::with_name("disassemble").short("d"))
        .arg(
            Arg::with_name("PATH")
                .help("ROM file path")
                .required(true)
                .index(1),
        )
        .get_matches();

    let path = matches.value_of("PATH").ok_or(Error::Path)?;
    let mut file = File::open(&path)?;

    if matches.is_present("disassemble") {
        loop {
            println!("{:?}", Op::read_next(&mut file)?);
        }
    } else {
        unimplemented!()
    }
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
    }
}
