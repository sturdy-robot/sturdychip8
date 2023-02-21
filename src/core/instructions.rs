use super::cpu::Cpu;
use rand::prelude::*;

pub fn decode(cpu: &mut Cpu) {
    let instruction = cpu.current_opcode >> 7;
    match instruction {
        0x0 => {
            decode_op0x(cpu);
        }
        0x1 => {
            let address = cpu.current_opcode & 0xFFF;
            cpu.reg.pc = address;
        }
        0x2 => { cpu.reg.pc = cpu.reg.pc.wrapping_add(2); }
        0x3 => {
            let reg_index = ((cpu.current_opcode & 0xF00) >> 8) as usize;
            let value = (cpu.current_opcode & 0xFF) as u8;
            if cpu.reg.v_r[reg_index] == value {
                cpu.reg.pc = cpu.reg.pc.wrapping_add(2);
            }
            cpu.reg.pc = cpu.reg.pc.wrapping_add(2);
        }
        0x4 => {
            let reg_index = ((cpu.current_opcode & 0xF00) >> 8) as usize;
            let value = (cpu.current_opcode & 0xFF) as u8;
            if cpu.reg.v_r[reg_index] != value {
                cpu.reg.pc = cpu.reg.pc.wrapping_add(2);
            }
            cpu.reg.pc = cpu.reg.pc.wrapping_add(2);
        }
        0x5 => {
            if cpu.current_opcode & 0xF == 0 {
                let reg_index = ((cpu.current_opcode & 0xF00) >> 8) as usize;
                let reg2_index = ((cpu.current_opcode & 0xF0) >> 4) as usize;
                if cpu.reg.v_r[reg_index] == cpu.reg.v_r[reg2_index] {
                    cpu.reg.pc = cpu.reg.pc.wrapping_add(2);
                }
            }
            cpu.reg.pc = cpu.reg.pc.wrapping_add(2);
        }
        0x6 => {
            let reg_index = ((cpu.current_opcode & 0xF00) >> 8) as usize;
            let value = (cpu.current_opcode & 0xFF) as u8;
            cpu.reg.v_r[reg_index] = value;
            cpu.reg.pc = cpu.reg.pc.wrapping_add(2);
        }
        0x7 => {
            let reg_index = ((cpu.current_opcode & 0xF00) >> 8) as usize;
            let value = (cpu.current_opcode & 0xFF) as u8;
            if reg_index != 0xF {
                cpu.reg.v_r[reg_index] = cpu.reg.v_r[reg_index].wrapping_add(value);
            }
            cpu.reg.pc = cpu.reg.pc.wrapping_add(2);
        }
        0x8 => {
            decode_op8x(cpu);
        }
        0x9 => {
            if cpu.current_opcode & 0xF == 0 {
                let x = ((cpu.current_opcode & 0xF00) >> 8) as usize;
                let y = ((cpu.current_opcode & 0xF0) >> 4) as usize;
                if cpu.reg.v_r[x] != cpu.reg.v_r[y] {
                    cpu.reg.pc = cpu.reg.pc.wrapping_add(2);
                }
            }
            cpu.reg.pc = cpu.reg.pc.wrapping_add(2);
        }
        0xA => {
            cpu.reg.i = cpu.current_opcode & 0xFFF;
            cpu.reg.pc = cpu.reg.pc.wrapping_add(2);
        }
        0xB => {
            let v0 = cpu.reg.v_r[0] as u16;
            let value = cpu.current_opcode & 0xFFF;
            let address = value.wrapping_add(v0);
            cpu.reg.pc = address;
        }
        0xC => {
            let x = ((cpu.current_opcode & 0xF00) >> 8) as usize;
            let mut rng = rand::thread_rng();
            let address = ((cpu.current_opcode & 0xF0) >> 4) as u8;
            let random_value: u8 = rng.gen();
            cpu.reg.v_r[x] = random_value & address;
            cpu.reg.pc = cpu.reg.pc.wrapping_add(2);
        }
        0xD => {}
        0xE => {}
        0xF => {}
        _ => unreachable!(),
    };
}

fn decode_op0x(cpu: &mut Cpu) {
    if cpu.current_opcode == 0xE0 {}
    if cpu.current_opcode == 0xEE {}
    cpu.reg.pc = cpu.reg.pc.wrapping_add(2);
}

fn decode_op8x(cpu: &mut Cpu) {
    let x = ((cpu.current_opcode & 0xF00) >> 8) as usize;
    let y = ((cpu.current_opcode & 0xF0) >> 4) as usize;
    match cpu.current_opcode & 0xF {
        0x0 => cpu.reg.v_r[x] = cpu.reg.v_r[y],
        0x1 => cpu.reg.v_r[x] |= cpu.reg.v_r[y],
        0x2 => cpu.reg.v_r[x] &= cpu.reg.v_r[y],
        0x3 => cpu.reg.v_r[x] ^= cpu.reg.v_r[y],
        0x4 => cpu.reg.v_r[x] = cpu.reg.v_r[x].wrapping_add(cpu.reg.v_r[y]),
        0x5 => cpu.reg.v_r[x] = cpu.reg.v_r[x].wrapping_sub(cpu.reg.v_r[y]),
        0x6 => cpu.reg.v_r[x] >>= 1,
        0x7 => cpu.reg.v_r[x] = cpu.reg.v_r[y].wrapping_sub(cpu.reg.v_r[x]),
        0xE => cpu.reg.v_r[x] <<= 1,
        _ => panic!("Invalid instruction {:04X}", cpu.current_opcode),
    };
    cpu.reg.pc = cpu.reg.pc.wrapping_add(2);
}
