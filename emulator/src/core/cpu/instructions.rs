mod addressing;
mod non_opcode;
mod transfer;

use crate::core::cpu::{CPU, CPULines};
use crate::core::lines::Bus;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AddressingMode {
    A,    // operand is accumulator (implied single byte instruction)
    Abs,  // operand is address $HHLL
    AbsX, // operand is address; effective address is address incremented by X with carry
    AbsY, // operand is address; effective address is address incremented by Y with carry
    Imm,  // operand is byte BB
    Impl, // operand implied
    Ind,  // operand is address; effective address is contents of word at address: C.w($HHLL)
    XInd, // operand is zeropage address; effective address is word in (LL + X, LL + X + 1), inc. without carry: C.w($00LL + X)
    IndY, // operand is zeropage address; effective address is word in (LL, LL + 1) incremented by Y with carry: C.w($00LL) + Y
    Rel,  // branch target is PC + signed offset BB
    Zpg,  // operand is zeropage address (hi-byte is zero, address = $00LL)
    ZpgX, // operand is zeropage address; effective address is address incremented by X without carry
    ZpgY, // operand is zeropage address; effective address is address incremented by Y without carry
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Instruction {
    // -- Non-opcode instructions --
    Reset, // Reset the CPU; called when powered on
    Fetch, // Fetch the next instruction; called after each instruction finishes

    // -- Legal opcodes --
    ADC, // Add Memory to Accumulator with Carry
    AND, // AND Memory with Accumulator
    ASL, // Shift Left One Bit (Memory or Accumulator)
    BCC, // Branch on Carry Clear
    BCS, // Branch on Carry Set
    BEQ, // Branch on Result Zero
    BIT, // Test Bits in Memory with Accumulator
    BMI, // Branch on Result Minus
    BNE, // Branch on Result not Zero
    BPL, // Branch on Result Plus
    BRK, // Force Break
    BVC, // Branch on Overflow Clear
    BVS, // Branch on Overflow Set
    CLC, // Clear Carry Flag
    CLD, // Clear Decimal Mode
    CLI, // Clear Interrupt Disable Bit
    CLV, // Clear Overflow Flag
    CMP, // Compare Memory with Accumulator
    CPX, // Compare Memory and Index X
    CPY, // Compare Memory and Index Y
    DEC, // Decrement Memory by One
    DEX, // Decrement Index X by One
    DEY, // Decrement Index Y by One
    EOR, // Exclusive-OR Memory with Accumulator
    INC, // Increment Memory by One
    INX, // Increment Index X by One
    INY, // Increment Index Y by One
    JMP, // Jump to New Location
    JSR, // Jump to New Location Saving Return Address
    LDA, // Load Accumulator with Memory
    LDX, // Load Index X with Memory
    LDY, // Load Index Y with Memory
    LSR, // Shift One Bit Right (Memory or Accumulator)
    NOP, // No Operation
    ORA, // OR Memory with Accumulator
    PHA, // Push Accumulator on Stack
    PHP, // Push Processor Status on Stack
    PLA, // Pull Accumulator from Stack
    PLP, // Pull Processor Status from Stack
    ROL, // Rotate One Bit Left (Memory or Accumulator)
    ROR, // Rotate One Bit Right (Memory or Accumulator)
    RTI, // Return from Interrupt
    RTS, // Return from Subroutine
    SBC, // Subtract Memory from Accumulator with Borrow
    SEC, // Set Carry Flag
    SED, // Set Decimal Flag
    SEI, // Set Interrupt Disable Status
    STA, // Store Accumulator in Memory
    STX, // Store Index X in Memory
    STY, // Store Index Y in Memory
    TAX, // Transfer Accumulator to Index X
    TAY, // Transfer Accumulator to Index Y
    TSX, // Transfer Stack Pointer to Index X
    TXA, // Transfer Index X to Accumulator
    TXS, // Transfer Index X to Stack Register
    TYA, // Transfer Index Y to Accumulator

    // -- Illegal opcodes --
    ALR,  // AND oper + LSR
    ANC,  // AND oper + set C as ASL/ROL (effectively the same)
    ANE,  // * OR X + AND oper
    ARR,  // AND oper + ROR
    DCP,  // DEC oper + CMP oper
    ISC,  // INC oper + SBC oper
    LAS,  // LDA/TSX oper
    LAX,  // LDA oper + LDX oper
    LXA,  // Store * AND oper in A and X
    RLA,  // ROL oper + AND oper
    RRA,  // ROR oper + ADC oper
    SAX,  // A and X put on bus (effectively AND) and stored in M
    SBX,  // CMP and DEX at once, sets flags like CMP
    SHA,  // Stores A AND X AND (high-byte of addr. + 1) at addr.
    SHX,  // Stores X AND (high-byte of addr. + 1) at addr.
    SHY,  // Stores Y AND (high-byte of addr. + 1) at addr.
    SLO,  // ASL oper + ORA oper
    SRE,  // LSR oper + EOR oper
    TAS,  // Puts A AND X in SP and stores A AND X AND (high-byte of addr. + 1) at addr.
    USBC, // SBC oper + NOP
    JAM,  // Freeze CPU in T1 phase with $FF on the data bus
}

pub fn fetch_instruction(opcode: u8) -> (Instruction, AddressingMode) {
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
        0xa0 => (Instruction::LDY, AddressingMode::Imm),
        0xb0 => (Instruction::BCS, AddressingMode::Rel),
        0xc0 => (Instruction::CPY, AddressingMode::Imm),
        0xd0 => (Instruction::BNE, AddressingMode::Rel),
        0xe0 => (Instruction::CPX, AddressingMode::Imm),
        0xf0 => (Instruction::BEQ, AddressingMode::Rel),

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
        0xa1 => (Instruction::LDA, AddressingMode::XInd),
        0xb1 => (Instruction::LDA, AddressingMode::IndY),
        0xc1 => (Instruction::CMP, AddressingMode::XInd),
        0xd1 => (Instruction::CMP, AddressingMode::IndY),
        0xe1 => (Instruction::SBC, AddressingMode::XInd),
        0xf1 => (Instruction::SBC, AddressingMode::IndY),

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
        0xa2 => (Instruction::LDX, AddressingMode::Imm),
        0xb2 => (Instruction::JAM, AddressingMode::Impl),
        0xc2 => (Instruction::NOP, AddressingMode::Imm),
        0xd2 => (Instruction::JAM, AddressingMode::Impl),
        0xe2 => (Instruction::NOP, AddressingMode::Imm),
        0xf2 => (Instruction::JAM, AddressingMode::Impl),

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
        0xa3 => (Instruction::LAX, AddressingMode::XInd),
        0xb3 => (Instruction::LAX, AddressingMode::IndY),
        0xc3 => (Instruction::DCP, AddressingMode::XInd),
        0xd3 => (Instruction::DCP, AddressingMode::IndY),
        0xe3 => (Instruction::ISC, AddressingMode::XInd),
        0xf3 => (Instruction::ISC, AddressingMode::IndY),

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
        0xa4 => (Instruction::LDY, AddressingMode::Zpg),
        0xb4 => (Instruction::LDY, AddressingMode::ZpgX),
        0xc4 => (Instruction::CPY, AddressingMode::Zpg),
        0xd4 => (Instruction::NOP, AddressingMode::ZpgX),
        0xe4 => (Instruction::CPX, AddressingMode::Zpg),
        0xf4 => (Instruction::NOP, AddressingMode::ZpgX),

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
        0xa5 => (Instruction::LDA, AddressingMode::Zpg),
        0xb5 => (Instruction::LDA, AddressingMode::ZpgX),
        0xc5 => (Instruction::CMP, AddressingMode::Zpg),
        0xd5 => (Instruction::CMP, AddressingMode::ZpgX),
        0xe5 => (Instruction::SBC, AddressingMode::Zpg),
        0xf5 => (Instruction::SBC, AddressingMode::ZpgX),

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
        0xa6 => (Instruction::LDX, AddressingMode::Zpg),
        0xb6 => (Instruction::LDX, AddressingMode::ZpgY),
        0xc6 => (Instruction::DEC, AddressingMode::Zpg),
        0xd6 => (Instruction::DEC, AddressingMode::ZpgX),
        0xe6 => (Instruction::INC, AddressingMode::Zpg),
        0xf6 => (Instruction::INC, AddressingMode::ZpgX),

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
        0xa7 => (Instruction::LAX, AddressingMode::Zpg),
        0xb7 => (Instruction::LAX, AddressingMode::ZpgY),
        0xc7 => (Instruction::DCP, AddressingMode::Zpg),
        0xd7 => (Instruction::DCP, AddressingMode::ZpgX),
        0xe7 => (Instruction::ISC, AddressingMode::Zpg),
        0xf7 => (Instruction::ISC, AddressingMode::ZpgX),

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
        0xa8 => (Instruction::TAY, AddressingMode::Impl),
        0xb8 => (Instruction::CLV, AddressingMode::Impl),
        0xc8 => (Instruction::INY, AddressingMode::Impl),
        0xd8 => (Instruction::CLD, AddressingMode::Impl),
        0xe8 => (Instruction::INX, AddressingMode::Impl),
        0xf8 => (Instruction::SED, AddressingMode::Impl),

        0x09 => (Instruction::ORA, AddressingMode::Imm),
        0x19 => (Instruction::ORA, AddressingMode::AbsY),
        0x29 => (Instruction::AND, AddressingMode::Imm),
        0x39 => (Instruction::AND, AddressingMode::AbsY),
        0x49 => (Instruction::EOR, AddressingMode::Imm),
        0x59 => (Instruction::EOR, AddressingMode::AbsY),
        0x69 => (Instruction::ADC, AddressingMode::Imm),
        0x79 => (Instruction::ADC, AddressingMode::AbsY),
        0x89 => (Instruction::NOP, AddressingMode::Imm),
        0x99 => (Instruction::STA, AddressingMode::AbsY),
        0xa9 => (Instruction::LDA, AddressingMode::Imm),
        0xb9 => (Instruction::LDA, AddressingMode::AbsY),
        0xc9 => (Instruction::CMP, AddressingMode::Imm),
        0xd9 => (Instruction::CMP, AddressingMode::AbsY),
        0xe9 => (Instruction::SBC, AddressingMode::Imm),
        0xf9 => (Instruction::SBC, AddressingMode::AbsY),

        0x0a => (Instruction::ASL, AddressingMode::A),
        0x1a => (Instruction::NOP, AddressingMode::Impl),
        0x2a => (Instruction::ROL, AddressingMode::A),
        0x3a => (Instruction::NOP, AddressingMode::Impl),
        0x4a => (Instruction::LSR, AddressingMode::A),
        0x5a => (Instruction::NOP, AddressingMode::Impl),
        0x6a => (Instruction::ROR, AddressingMode::A),
        0x7a => (Instruction::NOP, AddressingMode::Impl),
        0x8a => (Instruction::TXA, AddressingMode::Impl),
        0x9a => (Instruction::TXS, AddressingMode::Impl),
        0xaa => (Instruction::TAX, AddressingMode::Impl),
        0xba => (Instruction::TSX, AddressingMode::Impl),
        0xca => (Instruction::DEX, AddressingMode::Impl),
        0xda => (Instruction::NOP, AddressingMode::Impl),
        0xea => (Instruction::NOP, AddressingMode::Impl),
        0xfa => (Instruction::NOP, AddressingMode::Impl),

        0x0b => (Instruction::ANC, AddressingMode::Imm),
        0x1b => (Instruction::SLO, AddressingMode::AbsY),
        0x2b => (Instruction::ANC, AddressingMode::Imm),
        0x3b => (Instruction::RLA, AddressingMode::AbsY),
        0x4b => (Instruction::ALR, AddressingMode::Imm),
        0x5b => (Instruction::SRE, AddressingMode::AbsY),
        0x6b => (Instruction::ARR, AddressingMode::Imm),
        0x7b => (Instruction::RRA, AddressingMode::AbsY),
        0x8b => (Instruction::ANE, AddressingMode::Imm),
        0x9b => (Instruction::TAS, AddressingMode::AbsY),
        0xab => (Instruction::LXA, AddressingMode::Imm),
        0xbb => (Instruction::LAS, AddressingMode::AbsY),
        0xcb => (Instruction::SBX, AddressingMode::Imm),
        0xdb => (Instruction::DCP, AddressingMode::AbsY),
        0xeb => (Instruction::USBC, AddressingMode::Imm),
        0xfb => (Instruction::ISC, AddressingMode::AbsY),

        0x0c => (Instruction::NOP, AddressingMode::Abs),
        0x1c => (Instruction::NOP, AddressingMode::AbsX),
        0x2c => (Instruction::BIT, AddressingMode::Abs),
        0x3c => (Instruction::NOP, AddressingMode::AbsX),
        0x4c => (Instruction::JMP, AddressingMode::Abs),
        0x5c => (Instruction::NOP, AddressingMode::AbsX),
        0x6c => (Instruction::JMP, AddressingMode::Ind),
        0x7c => (Instruction::NOP, AddressingMode::AbsX),
        0x8c => (Instruction::STY, AddressingMode::Abs),
        0x9c => (Instruction::SHY, AddressingMode::AbsX),
        0xac => (Instruction::LDY, AddressingMode::Abs),
        0xbc => (Instruction::LDY, AddressingMode::AbsX),
        0xcc => (Instruction::CPY, AddressingMode::Abs),
        0xdc => (Instruction::NOP, AddressingMode::AbsX),
        0xec => (Instruction::CPX, AddressingMode::Abs),
        0xfc => (Instruction::NOP, AddressingMode::AbsX),

        0x0d => (Instruction::ORA, AddressingMode::Abs),
        0x1d => (Instruction::ORA, AddressingMode::AbsX),
        0x2d => (Instruction::AND, AddressingMode::Abs),
        0x3d => (Instruction::AND, AddressingMode::AbsX),
        0x4d => (Instruction::EOR, AddressingMode::Abs),
        0x5d => (Instruction::EOR, AddressingMode::AbsX),
        0x6d => (Instruction::ADC, AddressingMode::Abs),
        0x7d => (Instruction::ADC, AddressingMode::AbsX),
        0x8d => (Instruction::STA, AddressingMode::Abs),
        0x9d => (Instruction::STA, AddressingMode::AbsX),
        0xad => (Instruction::LDA, AddressingMode::Abs),
        0xbd => (Instruction::LDA, AddressingMode::AbsX),
        0xcd => (Instruction::CMP, AddressingMode::Abs),
        0xdd => (Instruction::CMP, AddressingMode::AbsX),
        0xed => (Instruction::SBC, AddressingMode::Abs),
        0xfd => (Instruction::SBC, AddressingMode::AbsX),

        0x0e => (Instruction::ASL, AddressingMode::Abs),
        0x1e => (Instruction::ASL, AddressingMode::AbsX),
        0x2e => (Instruction::ROL, AddressingMode::Abs),
        0x3e => (Instruction::ROL, AddressingMode::AbsX),
        0x4e => (Instruction::LSR, AddressingMode::Abs),
        0x5e => (Instruction::LSR, AddressingMode::AbsX),
        0x6e => (Instruction::ROR, AddressingMode::Abs),
        0x7e => (Instruction::ROR, AddressingMode::AbsX),
        0x8e => (Instruction::STX, AddressingMode::Abs),
        0x9e => (Instruction::SHX, AddressingMode::AbsY),
        0xae => (Instruction::LDX, AddressingMode::Abs),
        0xbe => (Instruction::LDX, AddressingMode::AbsY),
        0xce => (Instruction::DEC, AddressingMode::Abs),
        0xde => (Instruction::DEC, AddressingMode::AbsX),
        0xee => (Instruction::INC, AddressingMode::Abs),
        0xfe => (Instruction::INC, AddressingMode::AbsX),

        0x0f => (Instruction::SLO, AddressingMode::Abs),
        0x1f => (Instruction::SLO, AddressingMode::AbsX),
        0x2f => (Instruction::RLA, AddressingMode::Abs),
        0x3f => (Instruction::RLA, AddressingMode::AbsX),
        0x4f => (Instruction::SRE, AddressingMode::Abs),
        0x5f => (Instruction::SRE, AddressingMode::AbsX),
        0x6f => (Instruction::RRA, AddressingMode::Abs),
        0x7f => (Instruction::RRA, AddressingMode::AbsX),
        0x8f => (Instruction::SAX, AddressingMode::Abs),
        0x9f => (Instruction::SHA, AddressingMode::AbsY),
        0xaf => (Instruction::LAX, AddressingMode::Abs),
        0xbf => (Instruction::LAX, AddressingMode::AbsY),
        0xcf => (Instruction::DCP, AddressingMode::Abs),
        0xdf => (Instruction::DCP, AddressingMode::AbsX),
        0xef => (Instruction::ISC, AddressingMode::Abs),
        0xff => (Instruction::ISC, AddressingMode::AbsX),
    }
}

enum Register {
    A,
    X,
    Y,
    SR,
    SP,
}

pub fn execute_instruction_rising(cpu: &mut CPU, lines: &mut CPULines) {
    match cpu.current_instruction {
        Instruction::Reset => non_opcode::reset_rising(cpu, lines),
        Instruction::Fetch => non_opcode::fetch_rising(cpu, lines),

        Instruction::LDA => transfer::load_rising(cpu, lines),
        Instruction::LDX => transfer::load_rising(cpu, lines),
        Instruction::LDY => transfer::load_rising(cpu, lines),
        Instruction::STA => transfer::store_rising(Register::A, cpu, lines),
        Instruction::STX => transfer::store_rising(Register::X, cpu, lines),
        Instruction::STY => transfer::store_rising(Register::Y, cpu, lines),

        _ => (),
    }
}

pub fn execute_instruction_falling(cpu: &mut CPU, lines: &mut CPULines) {
    match cpu.current_instruction {
        Instruction::Reset => non_opcode::reset_falling(cpu, lines),
        Instruction::Fetch => non_opcode::fetch_falling(cpu, lines),

        Instruction::LDA => transfer::load_falling(Register::A, cpu, lines),
        Instruction::LDX => transfer::load_falling(Register::X, cpu, lines),
        Instruction::LDY => transfer::load_falling(Register::Y, cpu, lines),
        Instruction::STA => transfer::store_falling(cpu, lines),
        Instruction::STX => transfer::store_falling(cpu, lines),
        Instruction::STY => transfer::store_falling(cpu, lines),

        _ => (),
    }
}

// Return true if addressing is finished, for convenience.
fn execute_addressing_rising(cpu: &mut CPU, lines: &mut CPULines) -> bool {
    match cpu.current_addressing_mode {
        AddressingMode::Imm => addressing::imm_rising(cpu),
        AddressingMode::Abs => addressing::abs_rising(cpu, lines),
        AddressingMode::AbsX => addressing::abs_indexed_rising(cpu, lines),
        AddressingMode::AbsY => addressing::abs_indexed_rising(cpu, lines),
        AddressingMode::Zpg => addressing::zpg_rising(cpu, lines),
        AddressingMode::ZpgX => addressing::zpg_indexed_rising(cpu, lines),
        AddressingMode::ZpgY => addressing::zpg_indexed_rising(cpu, lines),
        // AddressingMode::Ind => addressing::ind_rising(cpu, lines),
        // AddressingMode::XInd => addressing::xind_rising(cpu, lines),
        // AddressingMode::IndY => addressing::indy_rising(cpu, lines),
        // AddressingMode::Rel => addressing::rel_rising(cpu, lines),
        _ => cpu.finished_addressing = true,
    }

    return cpu.finished_addressing;
}

fn execute_addressing_falling(cpu: &mut CPU, lines: &mut CPULines) {
    match cpu.current_addressing_mode {
        AddressingMode::Abs => addressing::abs_falling(cpu, lines),
        AddressingMode::AbsX => addressing::abs_indexed_falling(Register::X, cpu, lines),
        AddressingMode::AbsY => addressing::abs_indexed_falling(Register::Y, cpu, lines),
        AddressingMode::Zpg => addressing::zpg_falling(cpu, lines),
        AddressingMode::ZpgX => addressing::zpg_indexed_falling(Register::X, cpu, lines),
        AddressingMode::ZpgY => addressing::zpg_indexed_falling(Register::Y, cpu, lines),
        // AddressingMode::Ind => addressing::ind_falling(cpu, lines),
        // AddressingMode::XInd => addressing::xind_falling(cpu, lines),
        // AddressingMode::IndY => addressing::indy_falling(cpu, lines),
        // AddressingMode::Rel => addressing::rel_falling(cpu, lines),
        _ => panic!("Invalid addressing mode reached on clock falling edge."),
    }
}
