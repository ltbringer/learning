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
    registers: [u8; 16],
    memory: [u8; 4096],
    position_in_memory: usize,
    stack: [u16; 16],
    stack_pointer: usize,
}

impl CPU {
    fn read_opcode(&self) -> u16 {
        let p = self.position_in_memory as usize;
        let op_high_byte = self.memory[p] as u16;
        let op_low_byte = self.memory[p + 1] as u16;
        op_high_byte << 8 | op_low_byte
    }

    fn call(&mut self, addr: u16) {
        let sp = self.stack_pointer;
        let stack = &mut self.stack;
        if sp > stack.len() {
            panic!("stack overflow")
        }

        stack[sp] = self.position_in_memory as u16;
        self.stack_pointer += 1;
        self.position_in_memory = addr as usize;
    }

    fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("stack overflow");
        }

        self.stack_pointer -= 1;
        self.position_in_memory = self.stack[self.stack_pointer] as usize;
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];
        let (val, overflow_detected) = arg1.overflowing_add(arg2);
        self.registers[x as usize] = val;
        if overflow_detected {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }

    fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();
            self.position_in_memory += 2;
            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let d = (opcode & 0x000F) as u8;
            let nnn = opcode & 0x0FFF;
            match (c, x, y, d) {
                (0, 0, 0, 0) => {
                    return;
                }
                (0, 0, 0xE, 0xE) => self.ret(),
                (0x2, _, _, _) => self.call(nnn),
                (0x8, _, _, 0x4) => self.add_xy(x, y),
                _ => todo!("opcode: {:04x}", opcode),
            }
            println!("{:?}", self.registers);
        }
    }
}

fn main() {
    let mut cpu = CPU {
        registers: [0; 16],
        memory: [0; 4096],
        position_in_memory: 0,
        stack: [0; 16],
        stack_pointer: 0,
    };

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
    cpu.registers[2] = 17;
    cpu.registers[3] = 21;

    cpu.memory[0x000] = 0x21;
    cpu.memory[0x001] = 0x00;
    cpu.memory[0x002] = 0x21;
    cpu.memory[0x003] = 0x00;

    cpu.memory[0x100] = 0x80;
    cpu.memory[0x101] = 0x14;
    cpu.memory[0x102] = 0x80;
    cpu.memory[0x103] = 0x14;
    cpu.memory[0x104] = 0x00;
    cpu.memory[0x105] = 0xEE;

    cpu.run();
    println!("{}", cpu.registers[0]);
}
