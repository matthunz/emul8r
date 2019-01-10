enum Op {
    JP(u16),
}

struct Cpu {
    pc: u16,
}

impl Cpu {
    fn new() -> Self {
        Self { pc: 0 }
    }
    fn opcode(&mut self, op: Op) {
        match op {
            Op::JP(addr) => self.pc = addr,
        }
    }
}

fn main() {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn cpu() {
        let mut cpu = Cpu::new();
        cpu.opcode(Op::JP(4));
        assert_eq!(cpu.pc, 4);
    }
}
