use crate::error::Error;
use std::io::Read;

#[derive(Debug)]
pub enum Op {
    JP(u16),
    SE(u8, u8),
    LD(u8, u8),
    LDI(u16),
}

impl Op {
    pub fn read_next<R: Read>(reader: &mut R) -> Result<Op, Error> {
        let mut bytes = [0; 2];
        // TODO handle EOF
        reader.read_exact(&mut bytes)?;
        println!("{:x} {:x}", bytes[0], bytes[1]);

        let op = match (bytes[0] >> 4, bytes[0] & 0xF, bytes[1] >> 4, bytes[1] & 0xF) {
            (6, vx, a, b) => Op::LD(vx, a << 4 | b),
            (0xA, a, b, c) => Op::LDI((a as u16) << 8 | (b as u16) << 4 | (c as u16)),
            _ => Err(Error::Unimplemented {
                op: (bytes[0] as u16) << 8 | bytes[1] as u16,
            })?,
        };

        Ok(op)
    }
}
