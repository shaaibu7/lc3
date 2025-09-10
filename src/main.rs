
enum Register {
    R0,
    R1,
    R2,
    R3,
    R4, // General purpose registers
    R5,
    R7,
    PC, // Program counter
    COND,   // Conditional flags
    COUNT
}

enum Opcodes {
    BR,             // Branch
    ADD,            // Add
    LD,             // Load
    ST,             // Store
    JSR,            // Jump Register
    AND,            // Bitwise And
    LDR,            // Load Register
    STR,            // Store Register
    RTI,            // Unused
    NOT,            // Bitwise Not
    LDI,            // Load Indirect
    STI,            // Store Indirect
    JMP,            // Jump
    RES,            // Reserved(Unused)
    LEA,            // Load Effective Address
    TRAP            // Execute Trap
}

#[derive(Debug)]
enum R_COND {
    POS = 1 << 0,
    ZRO = 1 << 1,
    NEG = 1 << 2
}

const MEMORY_MAX: usize = 1 << 16 as usize;
const REGISTER_MAX: usize = 10;
const PC_START: u16 = 0x3000;



trait MemoryTraits {
    fn mem_read(&self, offset: u16) -> Option<u16>;
    fn new() -> Self;
}

impl MemoryTraits for Memory {
    fn mem_read(&self, offset: u16) -> Option<u16> {
        let instr: u16 = self.locations[offset as usize];
        Some(instr)
    }

    fn new() -> Self {
        Memory { locations: [0; MEMORY_MAX] }
    }
}
#[derive(Debug)]
struct Memory {
    locations: [u16; MEMORY_MAX]
}

#[derive(Debug)]
struct Reg {
    locations: [u16; REGISTER_MAX]
}


fn main() {
   let instr: u16 = 0b0010_000_001_0_00_001;
//    let opcode: u16 = instr as u16 >> 12;
//    println!("{:016b}", opcode);

    let mut memory = Memory::new();
    memory.locations[PC_START as usize] = instr;
    let opcode = memory.mem_read(PC_START).unwrap() >> 12;
    println!("{:016b}", opcode);

    let exec = match opcode {
        1 => println!("addition op"),
        2 => println!("load op"),
        _ => println!("default op...")
    };
}
