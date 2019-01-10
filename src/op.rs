use crate::error::Error;
use std::io::Read;

#[derive(Debug)]
pub enum Op {
    JP(u16),
    SE(u8, u8),
    LD(u8, u8),
    LDV(u8, u8),
    ADD(u8, u8),
    LDI(u16),
    RND(u8, u8),
    DRW(u8, u8, u8),
}

impl Op {
    pub fn read_next<R: Read>(reader: &mut R) -> Result<Op, Error> {
        let mut bytes = [0; 2];
        // TODO handle EOF
        reader.read_exact(&mut bytes)?;

        let op = match (bytes[0] >> 4, bytes[0] & 0xF, bytes[1] >> 4, bytes[1] & 0xF) {
            (1, a, b, c) => Op::JP(nnn(a, b, c)),
            (3, vx, a, b) => Op::SE(vx, kk(a, b)),
            (6, vx, a, b) => Op::LD(vx, kk(a, b)),
            (7, vx, a, b) => Op::ADD(vx, kk(a, b)),
            (8, vx, vy, 0) => Op::LDV(vx, vy),
            (0xA, a, b, c) => Op::LDI(nnn(a, b, c)),
            (0xC, vx, a, b) => Op::RND(vx, kk(a, b)),
            (0xD, vx, vy, n) => Op::DRW(vx, vy, n),
            _ => Err(Error::Unimplemented {
                op: (bytes[0] as u16) << 8 | bytes[1] as u16,
            })?,
        };

        Ok(op)
    }
}

fn kk(a: u8, b: u8) -> u8 {
    a << 4 | b
}

fn nnn(a: u8, b: u8, c: u8) -> u16 {
    (a as u16) << 8 | (b as u16) << 4 | (c as u16)
}
