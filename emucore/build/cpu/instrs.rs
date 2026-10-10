#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Instruction {
    // -- Legal instructions --
    Adc, // Add Memory to Accumulator with Carry
    And, // AND Memory with Accumulator
    Asl, // Shift Left One Bit (Memory or Accumulator)
    Bcc, // Branch on Carry Clear
    Bcs, // Branch on Carry Set
    Beq, // Branch on Result Zero
    Bit, // Test Bits in Memory with Accumulator
    Bmi, // Branch on Result Minus
    Bne, // Branch on Result not Zero
    Bpl, // Branch on Result Plus
    Brk, // Force Break
    Bvc, // Branch on Overflow Clear
    Bvs, // Branch on Overflow Set
    Clc, // Clear Carry Flag
    Cld, // Clear Decimal Mode
    Cli, // Clear Interrupt Disable Bit
    Clv, // Clear Overflow Flag
    Cmp, // Compare Memory with Accumulator
    Cpx, // Compare Memory and Index X
    Cpy, // Compare Memory and Index Y
    Dec, // Decrement Memory by One
    Dex, // Decrement Index X by One
    Dey, // Decrement Index Y by One
    Eor, // Exclusive-OR Memory with Accumulator
    Inc, // Increment Memory by One
    Inx, // Increment Index X by One
    Iny, // Increment Index Y by One
    Jmp, // Jump to New Location
    Jsr, // Jump to New Location Saving Return Address
    Lda, // Load Accumulator with Memory
    Ldx, // Load Index X with Memory
    Ldy, // Load Index Y with Memory
    Lsr, // Shift One Bit Right (Memory or Accumulator)
    Nop, // No Operation
    Ora, // OR Memory with Accumulator
    Pha, // Push Accumulator on Stack
    Php, // Push Processor Status on Stack
    Pla, // Pull Accumulator from Stack
    Plp, // Pull Processor Status from Stack
    Rol, // Rotate One Bit Left (Memory or Accumulator)
    Ror, // Rotate One Bit Right (Memory or Accumulator)
    Rti, // Return from Interrupt
    Rts, // Return from Subroutine
    Sbc, // Subtract Memory from Accumulator with Borrow
    Sec, // Set Carry Flag
    Sed, // Set Decimal Flag
    Sei, // Set Interrupt Disable Status
    Sta, // Store Accumulator in Memory
    Stx, // Store Index X in Memory
    Sty, // Store Index Y in Memory
    Tax, // Transfer Accumulator to Index X
    Tay, // Transfer Accumulator to Index Y
    Tsx, // Transfer Stack Pointer to Index X
    Txa, // Transfer Index X to Accumulator
    Txs, // Transfer Index X to Stack Register
    Tya, // Transfer Index Y to Accumulator

    // -- Illegal instructions --
    Alr,  // AND oper + LSR
    Anc,  // AND oper + set C as ASL/ROL (effectively the same)
    Ane,  // * OR X + AND oper
    Arr,  // AND oper + ROR
    Dcp,  // DEC oper + CMP oper
    Isc,  // INC oper + SBC oper
    Las,  // LDA/TSX oper
    Lax,  // LDA oper + LDX oper
    Lxa,  // Store * AND oper in A and X
    Rla,  // ROL oper + AND oper
    Rra,  // ROR oper + ADC oper
    Sax,  // A and X put on bus (effectively AND) and stored in M
    Sbx,  // CMP and DEX at once, sets flags like CMP
    Sha,  // Stores A AND X AND (high-byte of addr. + 1) at addr.
    Shx,  // Stores X AND (high-byte of addr. + 1) at addr.
    Shy,  // Stores Y AND (high-byte of addr. + 1) at addr.
    Slo,  // ASL oper + ORA oper
    Sre,  // LSR oper + EOR oper
    Tas,  // Puts A AND X in SP and stores A AND X AND (high-byte of addr. + 1) at addr.
    Usbc, // SBC oper + NOP
    Jam,  // Freeze CPU in T1 phase with $FF on the data bus
}
