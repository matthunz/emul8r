extern crate emul8r;

use emul8r::Op;
use failure::Fail;
use std::env;
use std::fs::File;
use std::io::{Error as IOError, ErrorKind, Read};

#[derive(Debug, Fail)]
enum Error {
    #[fail(display = "Please provide a file path.")]
    Path,
    #[fail(display = "IO Error: {:?}", inner)]
    IO {
        inner: IOError
    },
    #[fail(display = "Unimplemented: {:x}", op)]
    Unimplemented {
        op: u16
    },
}

impl From<IOError> for Error {
    fn from(inner: IOError) -> Error {
        Error::IO { inner }
    }
}

fn next_op(file: &mut File) -> Result<Op, Error> {
    let mut bytes = [0; 2];
    // TODO handle EOF
    file.read_exact(&mut bytes)?;
    let op = match (bytes[0] >> 4, bytes[0] & 0xF, bytes[1] >> 4, bytes[1] & 0xF) {
        (6, vx, a, b) => Op::LD(vx, a << 4 | b),
        (a, b, c, d) => {
            let op = (a as u16) << 12 | (b as u16) << 8 | (c as u16) << 4 | d as u16;
            Err(Error::Unimplemented { op })?
        }
    };
    Ok(op)
}

fn run() -> Result<(), Error> {
    let path = env::args().nth(1).ok_or(Error::Path)?;
    let mut file = File::open(&path)?;
    loop {
        let op = next_op(&mut file)?;
        println!("{:?}", op);
    }
}

fn main() {
    run().map_err(|e| eprintln!("{}", e));
}
