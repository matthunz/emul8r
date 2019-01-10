enum Op {
    JP(u16),
    SE(u8, u8),
}

struct Cpu {
    pc: u16,
    registers: [u8; 16],
}

impl Cpu {
    fn new() -> Self {
        Self { pc: 0, registers: [0; 16] }
    }
    fn opcode(&mut self, op: Op) {
        match op {
            Op::JP(addr) => self.pc = addr,
            Op::SE(vx, kk) => {
                if self.registers[vx as usize] == kk {
                    self.pc += 2;
                } else {
                    self.pc += 1;
                }
            }
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
        cpu.opcode(Op::SE(0, 0));
        assert_eq!(cpu.pc, 6);
        cpu.opcode(Op::SE(0, 1));
        assert_eq!(cpu.pc, 7);
    }
}
