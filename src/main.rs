
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


#[derive(Debug)]
struct Memory {
    locations: [u16; MEMORY_MAX]
}

#[derive(Debug)]
struct Reg {
    locations: [u16; REGISTER_MAX]
}


fn main() {
   let data = R_COND::NEG as u16;
    println!("{data:?}");
}
