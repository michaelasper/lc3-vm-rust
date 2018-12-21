#[macro_use]
extern crate num_derive;
use num_traits::cast::FromPrimitive;
use std::u16;

const PC_START: u16 = 0x3000;
const MEMORY_LIMIT: usize = u16::MAX as usize;
const REG_COUNT: usize = 10;

#[derive(FromPrimitive, ToPrimitive)]
enum Registers {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    PC, /* program counter */
    COND,
    COUNT,
}

#[derive(FromPrimitive, ToPrimitive)]
enum Opcodes {
    BR,   /* branch */
    ADD,  /* add  */
    LD,   /* load */
    ST,   /* store */
    JSR,  /* jump register */
    AND,  /* bitwise and */
    LDR,  /* load register */
    STR,  /* store register */
    RTI,  /* unused */
    NOT,  /* bitwise not */
    LDI,  /* load indirect */
    STI,  /* store indirect */
    JMP,  /* jump */
    RES,  /* reserved (unused) */
    LEA,  /* load effective address */
    TRAP, /* execute trap */
}

enum Flags {
    POS = 1 << 0, /* P */
    ZRO = 1 << 1, /* Z */
    NEG = 1 << 2, /* N */
}

macro_rules! load {
    ($x:expr, $y:expr) => {
        $x[num_traits::ToPrimitive::to_usize(&$y).unwrap()]
    };
}

fn main() {
    let mut memory: [u16; MEMORY_LIMIT] = [0; MEMORY_LIMIT];
    let mut reg: [u16; REG_COUNT] = [0; REG_COUNT];

    load!(reg, Registers::PC) = PC_START;
    let mut running = true;
    while running {
        let instr = mem_read(load!(reg, Registers::PC), &memory);
        let op: u16 = instr >> 12;

        match Opcodes::from_u16(op) {
            Some(Opcodes::BR) => unimplemented!(),
            Some(Opcodes::ADD) => add(instr, &mut reg),
            Some(Opcodes::LD) => unimplemented!(),
            Some(Opcodes::ST) => unimplemented!(),
            Some(Opcodes::JSR) => unimplemented!(),
            Some(Opcodes::AND) => and(instr, &mut reg),
            Some(Opcodes::LDR) => unimplemented!(),
            Some(Opcodes::STR) => unimplemented!(),
            Some(Opcodes::RTI) => unimplemented!(),
            Some(Opcodes::NOT) => not(instr, &mut reg),
            Some(Opcodes::LDI) => unimplemented!(),
            Some(Opcodes::STI) => unimplemented!(),
            Some(Opcodes::JMP) => jmp(instr, &mut reg),
            Some(Opcodes::RES) => unimplemented!(),
            Some(Opcodes::LEA) => unimplemented!(),
            Some(Opcodes::TRAP) => unimplemented!(),
            _ => panic!("Bad Opcode"),
        }
    }
}

fn add(instr: u16, reg: &mut [u16]) {
    /* destination register (DR) */
    let r0: u16 = (instr >> 9) & 0x7;
    /* first operand (SR1) */
    let r1: u16 = (instr >> 6) & 0x7;

    let mode = ((instr >> 5) & 0x1) == 1;

    if mode {
        let imm5 = extend(instr & 0x1F, 5);
        reg[r0 as usize] = reg[r1 as usize] + imm5;
    } else {
        let r2: u16 = instr & 0x7;
        reg[r0 as usize] = reg[r1 as usize] + reg[r2 as usize];
    }
    update_flags(r0, reg);

}

fn and(instr: u16, reg: &mut [u16]) {
    /* destination register (DR) */
    let r0: u16 = (instr >> 9) & 0x7;
    /* first operand (SR1) */
    let r1: u16 = (instr >> 6) & 0x7;

    let mode = ((instr >> 5) & 0x1) == 1;

    if mode {
        let imm5 = extend(instr & 0x1F, 5);
        reg[r0 as usize] = reg[r1 as usize] & imm5;
    } else {
        let r2: u16 = instr & 0x7;
        reg[r0 as usize] = reg[r1 as usize] & reg[r2 as usize];
    }
    update_flags(r0, reg);
}

fn not(instr: u16, reg: &mut [u16]) {
    /* destination register (DR) */
    let r0: u16 = (instr >> 9) & 0x7;
    /* first operand (SR1) */
    let r1: u16 = (instr >> 6) & 0x7;

    reg[r0 as usize] = !reg[r1 as usize];    
    update_flags(r0, reg);
}

fn jmp(instr: u16, reg: &mut [u16]) {
    let r1: u16 = (instr >> 6) & 0x7;
    load!(reg, Registers::PC) = reg[r1 as usize];
}

fn update_flags(r: u16, reg: &mut [u16]) {
    if reg[r as usize] == 0 { 
        load!(reg, Registers::COND) = Flags::ZRO as u16;
    } else if reg[r as usize] >> 15 == 1 {
        load!(reg, Registers::COND) = Flags::NEG as u16;
    } else {
        load!(reg, Registers::COND) = Flags::POS as u16;
    }
}

fn extend(mut x: u16, bit_count: i32) -> u16{
    if ((x >> (bit_count - 1)) & 1) == 1 {
        x |= 0xFFFF << bit_count;
    }
    return x;
}

fn mem_read(index: u16, memory: &[u16]) -> u16 {
    return memory[index as usize];
}
