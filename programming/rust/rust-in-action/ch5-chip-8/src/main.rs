/**
 * We’ll implement a subset of a system that was available to consumers in the 1970s called CHIP-8.
 * CHIP-8 was supported by a number of manufacturers, but was fairly primitive even by the standards of the day.
 * (It was created to write games, rather than for commercial or scientific applications.) One device was the COSMAC VIP.
 * It had a single-color display with a resolution of 64x32 (0.0002 megapixels), 2KB RAM, 1.76 Mhz CPU
 * and was sold for USD275. Oh, and you needed to assemble the computer yourself.
 *
 * It also contained games programmed by the world’s first female game developer: Joyce Weisbecker.
 */

struct CPU {
    current_operation: u16,
    registers: [u8; 2],
}

impl CPU {
    fn read_opcode(&self) -> u16 {
        self.current_operation
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        self.registers[x as usize] += self.registers[y as usize];
    }

    fn run(&mut self) {
        let opcode = self.read_opcode();
        let c = ((opcode & 0xF000) >> 12) as u8;
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let y = ((opcode & 0x00F0) >> 4) as u8;
        let d = (opcode & 0x000F) as u8;

        match (c, x, y, d) {
            (0x8, _, _, 0x4) => self.add_xy(x, y),
            _ => todo!("opcode: {:04x}", opcode),
        }
    }
}

fn main() {
    let mut cpu = CPU {
        current_operation: 0,
        registers: [0; 2],
    };
    cpu.current_operation = 0x8014;
    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
    cpu.run();
    println!("{}", cpu.registers[0]);
}
