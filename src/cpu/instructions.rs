enum AddressingMode {
    A,    // Use accumulator.
    Abs,  // Use data at address.
    AbsX, // Use data at (address + X).
    AbsY, // Use data at (address + Y).
    Imm,  // Use byte as data.
    Impl, // No operand.
    Ind,  // Use data at address as new address.
    XInd, // Use data at (address + x) as new address.
    IndY, // Use (data + y) at address as new address.
    Rel,  // Use data at (program counter + byte).
    Zpg,  // Operand is low byte of zero-page address
    ZpgX, // Operand is low byte of zero-page address incremented by X
    ZpgY, // Operand is low byte of zero-page address incremented by Y
}

enum Instruction {
    ADC, //
    AND, //
    ASL, //
    BCC, //
    BCS, //
    BEQ, //
    BIT, //
    BMI, //
    BNE, //
    BPL, //
    BRK, //
    BVC, //
    BVS, //
    CLC, //
    CLD, //
    CLI, //
    CLV, //
    CMP, //
    CPX, //
    CPY, //
    DEC, //
    DEX, //
    DEY, //
    EOR, //
    INC, //
    INX, //
    INY, //
    JMP, //
    JSR, //
    LDA, //
    LDX, //
    LDY, //
    LSR, //
    NOP, //
    ORA, //
    PHA, //
    PHP, //
    PLA, //
    PLP, //
    ROL, //
    ROR, //
    RTI, //
    RTS, //
    SBC, //
    SEC, //
    SED, //
    SEI, //
    STA, //
    STX, //
    STY, //
    TAX, //
    TAY, //
    TSX, //
    TXA, //
    TXS, //
    TYA, //

    // Illegal opcodes below
    ALR, //
    ANC, //
    AN2, //
    ANE, //
    ARR, //
    DCP, //
    ISC, //
    LAS, //
    LAX, //
    LXA, //
    RLA, //
    RRA, //
    SAX, //
    SBX, //
    SHA, //
    SHX, //
    SHY, //
    SLO, //
    SRE, //
    TAS, //
    USB, //
    JAM, //
}

pub fn get_instruction(opcode: u8) -> (Instruction, AddressingMode) {
    match opcode {
        0x00 => (Instruction::BRK, AddressingMode::Impl),
        0x10 => (Instruction::BPL, AddressingMode::Rel),
        0x20 => (Instruction::JSR, AddressingMode::Abs),
        0x30 => (Instruction::BMI, AddressingMode::Rel),
        0x40 => (Instruction::RTI, AddressingMode::Impl),
        0x50 => (Instruction::BVC, AddressingMode::Rel),
        0x60 => (Instruction::RTS, AddressingMode::Impl),
        0x70 => (Instruction::BVS, AddressingMode::Rel),
        0x80 => (Instruction::NOP, AddressingMode::Imm),
        0x90 => (Instruction::BCC, AddressingMode::Rel),
        0xA0 => (Instruction::LDY, AddressingMode::Imm),
        0xB0 => (Instruction::BCS, AddressingMode::Rel),
        0xC0 => (Instruction::CPY, AddressingMode::Imm),
        0xD0 => (Instruction::BNE, AddressingMode::Rel),
        0xE0 => (Instruction::CPX, AddressingMode::Imm),
        0xF0 => (Instruction::BEQ, AddressingMode::Rel),

        0x01 => (Instruction::ORA, AddressingMode::XInd),
        0x11 => (Instruction::ORA, AddressingMode::IndY),
        0x21 => (Instruction::AND, AddressingMode::XInd),
        0x31 => (Instruction::AND, AddressingMode::IndY),
        0x41 => (Instruction::EOR, AddressingMode::XInd),
        0x51 => (Instruction::EOR, AddressingMode::IndY),
        0x61 => (Instruction::ADC, AddressingMode::XInd),
        0x71 => (Instruction::ADC, AddressingMode::IndY),
        0x81 => (Instruction::STA, AddressingMode::XInd),
        0x91 => (Instruction::STA, AddressingMode::IndY),
        0xA1 => (Instruction::LDA, AddressingMode::XInd),
        0xB1 => (Instruction::LDA, AddressingMode::IndY),
        0xC1 => (Instruction::CMP, AddressingMode::XInd),
        0xD1 => (Instruction::CMP, AddressingMode::IndY),
        0xE1 => (Instruction::SBC, AddressingMode::XInd),
        0xF1 => (Instruction::SBC, AddressingMode::IndY),

        0x02 => (Instruction::JAM, AddressingMode::Impl),
        0x12 => (Instruction::JAM, AddressingMode::Impl),
        0x22 => (Instruction::JAM, AddressingMode::Impl),
        0x32 => (Instruction::JAM, AddressingMode::Impl),
        0x42 => (Instruction::JAM, AddressingMode::Impl),
        0x52 => (Instruction::JAM, AddressingMode::Impl),
        0x62 => (Instruction::JAM, AddressingMode::Impl),
        0x72 => (Instruction::JAM, AddressingMode::Impl),
        0x82 => (Instruction::NOP, AddressingMode::Imm),
        0x92 => (Instruction::JAM, AddressingMode::Impl),
        0xA2 => (Instruction::LDX, AddressingMode::Imm),
        0xB2 => (Instruction::JAM, AddressingMode::Impl),
        0xC2 => (Instruction::NOP, AddressingMode::Imm),
        0xD2 => (Instruction::JAM, AddressingMode::Impl),
        0xE2 => (Instruction::NOP, AddressingMode::Imm),
        0xF2 => (Instruction::JAM, AddressingMode::Impl),

        0x03 => (Instruction::SLO, AddressingMode::XInd),
        0x13 => (Instruction::SLO, AddressingMode::IndY),
        0x23 => (Instruction::RLA, AddressingMode::XInd),
        0x33 => (Instruction::RLA, AddressingMode::IndY),
        0x43 => (Instruction::SRE, AddressingMode::XInd),
        0x53 => (Instruction::SRE, AddressingMode::IndY),
        0x63 => (Instruction::RRA, AddressingMode::XInd),
        0x73 => (Instruction::RRA, AddressingMode::IndY),
        0x83 => (Instruction::SAX, AddressingMode::XInd),
        0x93 => (Instruction::SHA, AddressingMode::IndY),
        0xA3 => (Instruction::LAX, AddressingMode::XInd),
        0xB3 => (Instruction::LAX, AddressingMode::IndY),
        0xC3 => (Instruction::DCP, AddressingMode::XInd),
        0xD3 => (Instruction::DCP, AddressingMode::IndY),
        0xE3 => (Instruction::ISC, AddressingMode::XInd),
        0xF3 => (Instruction::ISC, AddressingMode::IndY),

        0x04 => (Instruction::NOP, AddressingMode::Zpg),
        0x14 => (Instruction::NOP, AddressingMode::ZpgX),
        0x24 => (Instruction::BIT, AddressingMode::Zpg),
        0x34 => (Instruction::NOP, AddressingMode::ZpgX),
        0x44 => (Instruction::NOP, AddressingMode::Zpg),
        0x54 => (Instruction::NOP, AddressingMode::ZpgX),
        0x64 => (Instruction::NOP, AddressingMode::Zpg),
        0x74 => (Instruction::NOP, AddressingMode::ZpgX),
        0x84 => (Instruction::STY, AddressingMode::Zpg),
        0x94 => (Instruction::STY, AddressingMode::ZpgX),
        0xA4 => (Instruction::LDY, AddressingMode::Zpg),
        0xB4 => (Instruction::LDY, AddressingMode::ZpgX),
        0xC4 => (Instruction::CPY, AddressingMode::Zpg),
        0xD4 => (Instruction::NOP, AddressingMode::ZpgX),
        0xE4 => (Instruction::CPX, AddressingMode::Zpg),
        0xF4 => (Instruction::NOP, AddressingMode::ZpgX),

        0x05 => (Instruction::ORA, AddressingMode::Zpg),
        0x15 => (Instruction::ORA, AddressingMode::ZpgX),
        0x25 => (Instruction::AND, AddressingMode::Zpg),
        0x35 => (Instruction::AND, AddressingMode::ZpgX),
        0x45 => (Instruction::EOR, AddressingMode::Zpg),
        0x55 => (Instruction::EOR, AddressingMode::ZpgX),
        0x65 => (Instruction::ADC, AddressingMode::Zpg),
        0x75 => (Instruction::ADC, AddressingMode::ZpgX),
        0x85 => (Instruction::STA, AddressingMode::Zpg),
        0x95 => (Instruction::STA, AddressingMode::ZpgX),
        0xA5 => (Instruction::LDA, AddressingMode::Zpg),
        0xB5 => (Instruction::LDA, AddressingMode::ZpgX),
        0xC5 => (Instruction::CMP, AddressingMode::Zpg),
        0xD5 => (Instruction::CMP, AddressingMode::ZpgX),
        0xE5 => (Instruction::SBC, AddressingMode::Zpg),
        0xF5 => (Instruction::SBC, AddressingMode::ZpgX),

        0x06 => (Instruction::ASL, AddressingMode::Zpg),
        0x16 => (Instruction::ASL, AddressingMode::ZpgX),
        0x26 => (Instruction::ROL, AddressingMode::Zpg),
        0x36 => (Instruction::ROL, AddressingMode::ZpgX),
        0x46 => (Instruction::LSR, AddressingMode::Zpg),
        0x56 => (Instruction::LSR, AddressingMode::ZpgX),
        0x66 => (Instruction::ROR, AddressingMode::Zpg),
        0x76 => (Instruction::ROR, AddressingMode::ZpgX),
        0x86 => (Instruction::STX, AddressingMode::Zpg),
        0x96 => (Instruction::STX, AddressingMode::ZpgY),
        0xA6 => (Instruction::LDX, AddressingMode::Zpg),
        0xB6 => (Instruction::LDX, AddressingMode::ZpgY),
        0xC6 => (Instruction::DEC, AddressingMode::Zpg),
        0xD6 => (Instruction::DEC, AddressingMode::ZpgX),
        0xE6 => (Instruction::INC, AddressingMode::Zpg),
        0xF6 => (Instruction::INC, AddressingMode::ZpgX),

        0x07 => (Instruction::SLO, AddressingMode::Zpg),
        0x17 => (Instruction::SLO, AddressingMode::ZpgX),
        0x27 => (Instruction::RLA, AddressingMode::Zpg),
        0x37 => (Instruction::RLA, AddressingMode::ZpgX),
        0x47 => (Instruction::SRE, AddressingMode::Zpg),
        0x57 => (Instruction::SRE, AddressingMode::ZpgX),
        0x67 => (Instruction::RRA, AddressingMode::Zpg),
        0x77 => (Instruction::RRA, AddressingMode::ZpgX),
        0x87 => (Instruction::SAX, AddressingMode::Zpg),
        0x97 => (Instruction::SAX, AddressingMode::ZpgY),
        0xA7 => (Instruction::LAX, AddressingMode::Zpg),
        0xB7 => (Instruction::LAX, AddressingMode::ZpgY),
        0xC7 => (Instruction::DCP, AddressingMode::Zpg),
        0xD7 => (Instruction::DCP, AddressingMode::ZpgX),
        0xE7 => (Instruction::ISC, AddressingMode::Zpg),
        0xF7 => (Instruction::ISC, AddressingMode::ZpgX),

        0x08 => (Instruction::PHP, AddressingMode::Impl),
        0x18 => (Instruction::CLC, AddressingMode::Impl),
        0x28 => (Instruction::PLP, AddressingMode::Impl),
        0x38 => (Instruction::SEC, AddressingMode::Impl),
        0x48 => (Instruction::PHA, AddressingMode::Impl),
        0x58 => (Instruction::CLI, AddressingMode::Impl),
        0x68 => (Instruction::PLA, AddressingMode::Impl),
        0x78 => (Instruction::SEI, AddressingMode::Impl),
        0x88 => (Instruction::DEY, AddressingMode::Impl),
        0x98 => (Instruction::TYA, AddressingMode::Impl),
        0xA8 => (Instruction::TAY, AddressingMode::Impl),
        0xB8 => (Instruction::CLV, AddressingMode::Impl),
        0xC8 => (Instruction::INY, AddressingMode::Impl),
        0xD8 => (Instruction::CLD, AddressingMode::Impl),
        0xE8 => (Instruction::INX, AddressingMode::Impl),
        0xF8 => (Instruction::SED, AddressingMode::Impl),
        // 0x00 => (Instruction::, AddressingMode::),
        // 0x10 => (Instruction::, AddressingMode::),
        // 0x20 => (Instruction::, AddressingMode::),
        // 0x30 => (Instruction::, AddressingMode::),
        // 0x40 => (Instruction::, AddressingMode::),
        // 0x50 => (Instruction::, AddressingMode::),
        // 0x60 => (Instruction::, AddressingMode::),
        // 0x70 => (Instruction::, AddressingMode::),
        // 0x80 => (Instruction::, AddressingMode::),
        // 0x90 => (Instruction::, AddressingMode::),
        // 0xA0 => (Instruction::, AddressingMode::),
        // 0xB0 => (Instruction::, AddressingMode::),
        // 0xC0 => (Instruction::, AddressingMode::),
        // 0xD0 => (Instruction::, AddressingMode::),
        // 0xE0 => (Instruction::, AddressingMode::),
        // 0xF0 => (Instruction::, AddressingMode::),

        // 0x00 => (Instruction::, AddressingMode::),
        // 0x10 => (Instruction::, AddressingMode::),
        // 0x20 => (Instruction::, AddressingMode::),
        // 0x30 => (Instruction::, AddressingMode::),
        // 0x40 => (Instruction::, AddressingMode::),
        // 0x50 => (Instruction::, AddressingMode::),
        // 0x60 => (Instruction::, AddressingMode::),
        // 0x70 => (Instruction::, AddressingMode::),
        // 0x80 => (Instruction::, AddressingMode::),
        // 0x90 => (Instruction::, AddressingMode::),
        // 0xA0 => (Instruction::, AddressingMode::),
        // 0xB0 => (Instruction::, AddressingMode::),
        // 0xC0 => (Instruction::, AddressingMode::),
        // 0xD0 => (Instruction::, AddressingMode::),
        // 0xE0 => (Instruction::, AddressingMode::),
        // 0xF0 => (Instruction::, AddressingMode::),

        // 0x00 => (Instruction::, AddressingMode::),
        // 0x10 => (Instruction::, AddressingMode::),
        // 0x20 => (Instruction::, AddressingMode::),
        // 0x30 => (Instruction::, AddressingMode::),
        // 0x40 => (Instruction::, AddressingMode::),
        // 0x50 => (Instruction::, AddressingMode::),
        // 0x60 => (Instruction::, AddressingMode::),
        // 0x70 => (Instruction::, AddressingMode::),
        // 0x80 => (Instruction::, AddressingMode::),
        // 0x90 => (Instruction::, AddressingMode::),
        // 0xA0 => (Instruction::, AddressingMode::),
        // 0xB0 => (Instruction::, AddressingMode::),
        // 0xC0 => (Instruction::, AddressingMode::),
        // 0xD0 => (Instruction::, AddressingMode::),
        // 0xE0 => (Instruction::, AddressingMode::),
        // 0xF0 => (Instruction::, AddressingMode::),

        // 0x00 => (Instruction::, AddressingMode::),
        // 0x10 => (Instruction::, AddressingMode::),
        // 0x20 => (Instruction::, AddressingMode::),
        // 0x30 => (Instruction::, AddressingMode::),
        // 0x40 => (Instruction::, AddressingMode::),
        // 0x50 => (Instruction::, AddressingMode::),
        // 0x60 => (Instruction::, AddressingMode::),
        // 0x70 => (Instruction::, AddressingMode::),
        // 0x80 => (Instruction::, AddressingMode::),
        // 0x90 => (Instruction::, AddressingMode::),
        // 0xA0 => (Instruction::, AddressingMode::),
        // 0xB0 => (Instruction::, AddressingMode::),
        // 0xC0 => (Instruction::, AddressingMode::),
        // 0xD0 => (Instruction::, AddressingMode::),
        // 0xE0 => (Instruction::, AddressingMode::),
        // 0xF0 => (Instruction::, AddressingMode::),

        // 0x00 => (Instruction::, AddressingMode::),
        // 0x10 => (Instruction::, AddressingMode::),
        // 0x20 => (Instruction::, AddressingMode::),
        // 0x30 => (Instruction::, AddressingMode::),
        // 0x40 => (Instruction::, AddressingMode::),
        // 0x50 => (Instruction::, AddressingMode::),
        // 0x60 => (Instruction::, AddressingMode::),
        // 0x70 => (Instruction::, AddressingMode::),
        // 0x80 => (Instruction::, AddressingMode::),
        // 0x90 => (Instruction::, AddressingMode::),
        // 0xA0 => (Instruction::, AddressingMode::),
        // 0xB0 => (Instruction::, AddressingMode::),
        // 0xC0 => (Instruction::, AddressingMode::),
        // 0xD0 => (Instruction::, AddressingMode::),
        // 0xE0 => (Instruction::, AddressingMode::),
        // 0xF0 => (Instruction::, AddressingMode::),

        // 0x00 => (Instruction::, AddressingMode::),
        // 0x10 => (Instruction::, AddressingMode::),
        // 0x20 => (Instruction::, AddressingMode::),
        // 0x30 => (Instruction::, AddressingMode::),
        // 0x40 => (Instruction::, AddressingMode::),
        // 0x50 => (Instruction::, AddressingMode::),
        // 0x60 => (Instruction::, AddressingMode::),
        // 0x70 => (Instruction::, AddressingMode::),
        // 0x80 => (Instruction::, AddressingMode::),
        // 0x90 => (Instruction::, AddressingMode::),
        // 0xA0 => (Instruction::, AddressingMode::),
        // 0xB0 => (Instruction::, AddressingMode::),
        // 0xC0 => (Instruction::, AddressingMode::),
        // 0xD0 => (Instruction::, AddressingMode::),
        // 0xE0 => (Instruction::, AddressingMode::),
        // 0xF0 => (Instruction::, AddressingMode::),

        // 0x00 => (Instruction::, AddressingMode::),
        // 0x10 => (Instruction::, AddressingMode::),
        // 0x20 => (Instruction::, AddressingMode::),
        // 0x30 => (Instruction::, AddressingMode::),
        // 0x40 => (Instruction::, AddressingMode::),
        // 0x50 => (Instruction::, AddressingMode::),
        // 0x60 => (Instruction::, AddressingMode::),
        // 0x70 => (Instruction::, AddressingMode::),
        // 0x80 => (Instruction::, AddressingMode::),
        // 0x90 => (Instruction::, AddressingMode::),
        // 0xA0 => (Instruction::, AddressingMode::),
        // 0xB0 => (Instruction::, AddressingMode::),
        // 0xC0 => (Instruction::, AddressingMode::),
        // 0xD0 => (Instruction::, AddressingMode::),
        // 0xE0 => (Instruction::, AddressingMode::),
        // 0xF0 => (Instruction::, AddressingMode::),
    }
}
