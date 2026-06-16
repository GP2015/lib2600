#![feature(prelude_import)]
#![warn(clippy::pedantic, clippy::nursery)]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
enum BaseInstruction {
    Adc,
    And,
    Asl,
    Bcc,
    Bcs,
    Beq,
    Bit,
    Bmi,
    Bne,
    Bpl,
    Brk,
    Bvc,
    Bvs,
    Clc,
    Cld,
    Cli,
    Clv,
    Cmp,
    Cpx,
    Cpy,
    Dec,
    Dex,
    Dey,
    Eor,
    Inc,
    Inx,
    Iny,
    Jmp,
    Jsr,
    Lda,
    Ldx,
    Ldy,
    Lsr,
    Nop,
    Ora,
    Pha,
    Php,
    Pla,
    Plp,
    Rol,
    Ror,
    Rti,
    Rts,
    Sbc,
    Sec,
    Sed,
    Sei,
    Sta,
    Stx,
    Sty,
    Tax,
    Tay,
    Tsx,
    Txa,
    Txs,
    Tya,
    Alr,
    Anc,
    Ane,
    Arr,
    Dcp,
    Isc,
    Las,
    Lax,
    Lxa,
    Rla,
    Rra,
    Sax,
    Sbx,
    Sha,
    Shx,
    Shy,
    Slo,
    Sre,
    Tas,
    Usbc,
    Jam,
}
#[automatically_derived]
#[doc(hidden)]
unsafe impl ::core::clone::TrivialClone for BaseInstruction {}
#[automatically_derived]
impl ::core::clone::Clone for BaseInstruction {
    #[inline]
    fn clone(&self) -> BaseInstruction {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for BaseInstruction {}
#[automatically_derived]
impl ::core::fmt::Debug for BaseInstruction {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(
            f,
            match self {
                BaseInstruction::Adc => "Adc",
                BaseInstruction::And => "And",
                BaseInstruction::Asl => "Asl",
                BaseInstruction::Bcc => "Bcc",
                BaseInstruction::Bcs => "Bcs",
                BaseInstruction::Beq => "Beq",
                BaseInstruction::Bit => "Bit",
                BaseInstruction::Bmi => "Bmi",
                BaseInstruction::Bne => "Bne",
                BaseInstruction::Bpl => "Bpl",
                BaseInstruction::Brk => "Brk",
                BaseInstruction::Bvc => "Bvc",
                BaseInstruction::Bvs => "Bvs",
                BaseInstruction::Clc => "Clc",
                BaseInstruction::Cld => "Cld",
                BaseInstruction::Cli => "Cli",
                BaseInstruction::Clv => "Clv",
                BaseInstruction::Cmp => "Cmp",
                BaseInstruction::Cpx => "Cpx",
                BaseInstruction::Cpy => "Cpy",
                BaseInstruction::Dec => "Dec",
                BaseInstruction::Dex => "Dex",
                BaseInstruction::Dey => "Dey",
                BaseInstruction::Eor => "Eor",
                BaseInstruction::Inc => "Inc",
                BaseInstruction::Inx => "Inx",
                BaseInstruction::Iny => "Iny",
                BaseInstruction::Jmp => "Jmp",
                BaseInstruction::Jsr => "Jsr",
                BaseInstruction::Lda => "Lda",
                BaseInstruction::Ldx => "Ldx",
                BaseInstruction::Ldy => "Ldy",
                BaseInstruction::Lsr => "Lsr",
                BaseInstruction::Nop => "Nop",
                BaseInstruction::Ora => "Ora",
                BaseInstruction::Pha => "Pha",
                BaseInstruction::Php => "Php",
                BaseInstruction::Pla => "Pla",
                BaseInstruction::Plp => "Plp",
                BaseInstruction::Rol => "Rol",
                BaseInstruction::Ror => "Ror",
                BaseInstruction::Rti => "Rti",
                BaseInstruction::Rts => "Rts",
                BaseInstruction::Sbc => "Sbc",
                BaseInstruction::Sec => "Sec",
                BaseInstruction::Sed => "Sed",
                BaseInstruction::Sei => "Sei",
                BaseInstruction::Sta => "Sta",
                BaseInstruction::Stx => "Stx",
                BaseInstruction::Sty => "Sty",
                BaseInstruction::Tax => "Tax",
                BaseInstruction::Tay => "Tay",
                BaseInstruction::Tsx => "Tsx",
                BaseInstruction::Txa => "Txa",
                BaseInstruction::Txs => "Txs",
                BaseInstruction::Tya => "Tya",
                BaseInstruction::Alr => "Alr",
                BaseInstruction::Anc => "Anc",
                BaseInstruction::Ane => "Ane",
                BaseInstruction::Arr => "Arr",
                BaseInstruction::Dcp => "Dcp",
                BaseInstruction::Isc => "Isc",
                BaseInstruction::Las => "Las",
                BaseInstruction::Lax => "Lax",
                BaseInstruction::Lxa => "Lxa",
                BaseInstruction::Rla => "Rla",
                BaseInstruction::Rra => "Rra",
                BaseInstruction::Sax => "Sax",
                BaseInstruction::Sbx => "Sbx",
                BaseInstruction::Sha => "Sha",
                BaseInstruction::Shx => "Shx",
                BaseInstruction::Shy => "Shy",
                BaseInstruction::Slo => "Slo",
                BaseInstruction::Sre => "Sre",
                BaseInstruction::Tas => "Tas",
                BaseInstruction::Usbc => "Usbc",
                BaseInstruction::Jam => "Jam",
            },
        )
    }
}
enum AddrMode {
    A,
    Abs,
    AbsIdx,
    Imm,
    Impl,
    Ind,
    IndIdx,
    Rel,
    Zpg,
    ZpgIdx,
    Jam,
}
#[automatically_derived]
#[doc(hidden)]
unsafe impl ::core::clone::TrivialClone for AddrMode {}
#[automatically_derived]
impl ::core::clone::Clone for AddrMode {
    #[inline]
    fn clone(&self) -> AddrMode {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for AddrMode {}
#[automatically_derived]
impl ::core::fmt::Debug for AddrMode {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(
            f,
            match self {
                AddrMode::A => "A",
                AddrMode::Abs => "Abs",
                AddrMode::AbsIdx => "AbsIdx",
                AddrMode::Imm => "Imm",
                AddrMode::Impl => "Impl",
                AddrMode::Ind => "Ind",
                AddrMode::IndIdx => "IndIdx",
                AddrMode::Rel => "Rel",
                AddrMode::Zpg => "Zpg",
                AddrMode::ZpgIdx => "ZpgIdx",
                AddrMode::Jam => "Jam",
            },
        )
    }
}
enum AddrModeIdx {
    X,
    Y,
    None,
}
#[automatically_derived]
#[doc(hidden)]
unsafe impl ::core::clone::TrivialClone for AddrModeIdx {}
#[automatically_derived]
impl ::core::clone::Clone for AddrModeIdx {
    #[inline]
    fn clone(&self) -> AddrModeIdx {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for AddrModeIdx {}
#[automatically_derived]
impl ::core::fmt::Debug for AddrModeIdx {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(
            f,
            match self {
                AddrModeIdx::X => "X",
                AddrModeIdx::Y => "Y",
                AddrModeIdx::None => "None",
            },
        )
    }
}
struct Instruction {
    base_instr: BaseInstruction,
    addr_mode: AddrMode,
    addr_idx: AddrModeIdx,
}
#[automatically_derived]
#[doc(hidden)]
unsafe impl ::core::clone::TrivialClone for Instruction {}
#[automatically_derived]
impl ::core::clone::Clone for Instruction {
    #[inline]
    fn clone(&self) -> Instruction {
        let _: ::core::clone::AssertParamIsClone<BaseInstruction>;
        let _: ::core::clone::AssertParamIsClone<AddrMode>;
        let _: ::core::clone::AssertParamIsClone<AddrModeIdx>;
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Instruction {}
#[automatically_derived]
impl ::core::fmt::Debug for Instruction {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "Instruction",
            "base_instr",
            &self.base_instr,
            "addr_mode",
            &self.addr_mode,
            "addr_idx",
            &&self.addr_idx,
        )
    }
}
const OPCODE_LUT: [Instruction; 256] = [
    Instruction {
        base_instr: BaseInstruction::Brk,
        addr_mode: AddrMode::Impl,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Nop,
        addr_mode: AddrMode::Imm,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Rti,
        addr_mode: AddrMode::Impl,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Cpy,
        addr_mode: AddrMode::Imm,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Jsr,
        addr_mode: AddrMode::Abs,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Ldy,
        addr_mode: AddrMode::Imm,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Rts,
        addr_mode: AddrMode::Impl,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Cpx,
        addr_mode: AddrMode::Imm,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Bpl,
        addr_mode: AddrMode::Rel,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Bcc,
        addr_mode: AddrMode::Rel,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Bvc,
        addr_mode: AddrMode::Rel,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Bne,
        addr_mode: AddrMode::Rel,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Bmi,
        addr_mode: AddrMode::Rel,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Bcs,
        addr_mode: AddrMode::Rel,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Bvs,
        addr_mode: AddrMode::Rel,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Beq,
        addr_mode: AddrMode::Rel,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Php,
        addr_mode: AddrMode::Impl,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Dey,
        addr_mode: AddrMode::Impl,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Pha,
        addr_mode: AddrMode::Impl,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Iny,
        addr_mode: AddrMode::Impl,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Plp,
        addr_mode: AddrMode::Impl,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Tay,
        addr_mode: AddrMode::Impl,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Pla,
        addr_mode: AddrMode::Impl,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Inx,
        addr_mode: AddrMode::Impl,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Clc,
        addr_mode: AddrMode::Impl,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Tya,
        addr_mode: AddrMode::Impl,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Cli,
        addr_mode: AddrMode::Impl,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Cld,
        addr_mode: AddrMode::Impl,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Sec,
        addr_mode: AddrMode::Impl,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Clv,
        addr_mode: AddrMode::Impl,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Sei,
        addr_mode: AddrMode::Impl,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Sed,
        addr_mode: AddrMode::Impl,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Nop,
        addr_mode: AddrMode::Zpg,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Sty,
        addr_mode: AddrMode::Zpg,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Nop,
        addr_mode: AddrMode::Zpg,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Cpy,
        addr_mode: AddrMode::Zpg,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Bit,
        addr_mode: AddrMode::Zpg,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Ldy,
        addr_mode: AddrMode::Zpg,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Nop,
        addr_mode: AddrMode::Zpg,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Cpx,
        addr_mode: AddrMode::Zpg,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Nop,
        addr_mode: AddrMode::ZpgIdx,
        addr_idx: AddrModeIdx::X,
    },
    Instruction {
        base_instr: BaseInstruction::Sty,
        addr_mode: AddrMode::ZpgIdx,
        addr_idx: AddrModeIdx::X,
    },
    Instruction {
        base_instr: BaseInstruction::Nop,
        addr_mode: AddrMode::ZpgIdx,
        addr_idx: AddrModeIdx::X,
    },
    Instruction {
        base_instr: BaseInstruction::Nop,
        addr_mode: AddrMode::ZpgIdx,
        addr_idx: AddrModeIdx::X,
    },
    Instruction {
        base_instr: BaseInstruction::Nop,
        addr_mode: AddrMode::ZpgIdx,
        addr_idx: AddrModeIdx::X,
    },
    Instruction {
        base_instr: BaseInstruction::Ldy,
        addr_mode: AddrMode::ZpgIdx,
        addr_idx: AddrModeIdx::X,
    },
    Instruction {
        base_instr: BaseInstruction::Nop,
        addr_mode: AddrMode::ZpgIdx,
        addr_idx: AddrModeIdx::X,
    },
    Instruction {
        base_instr: BaseInstruction::Nop,
        addr_mode: AddrMode::ZpgIdx,
        addr_idx: AddrModeIdx::X,
    },
    Instruction {
        base_instr: BaseInstruction::Nop,
        addr_mode: AddrMode::Abs,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Sty,
        addr_mode: AddrMode::Abs,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Jmp,
        addr_mode: AddrMode::Abs,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Cpy,
        addr_mode: AddrMode::Abs,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Bit,
        addr_mode: AddrMode::Abs,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Ldy,
        addr_mode: AddrMode::Abs,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Jmp,
        addr_mode: AddrMode::Ind,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Cpx,
        addr_mode: AddrMode::Abs,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Nop,
        addr_mode: AddrMode::AbsIdx,
        addr_idx: AddrModeIdx::X,
    },
    Instruction {
        base_instr: BaseInstruction::Shy,
        addr_mode: AddrMode::AbsIdx,
        addr_idx: AddrModeIdx::X,
    },
    Instruction {
        base_instr: BaseInstruction::Nop,
        addr_mode: AddrMode::AbsIdx,
        addr_idx: AddrModeIdx::X,
    },
    Instruction {
        base_instr: BaseInstruction::Nop,
        addr_mode: AddrMode::AbsIdx,
        addr_idx: AddrModeIdx::X,
    },
    Instruction {
        base_instr: BaseInstruction::Nop,
        addr_mode: AddrMode::AbsIdx,
        addr_idx: AddrModeIdx::X,
    },
    Instruction {
        base_instr: BaseInstruction::Ldy,
        addr_mode: AddrMode::AbsIdx,
        addr_idx: AddrModeIdx::X,
    },
    Instruction {
        base_instr: BaseInstruction::Nop,
        addr_mode: AddrMode::AbsIdx,
        addr_idx: AddrModeIdx::X,
    },
    Instruction {
        base_instr: BaseInstruction::Nop,
        addr_mode: AddrMode::AbsIdx,
        addr_idx: AddrModeIdx::X,
    },
    Instruction {
        base_instr: BaseInstruction::Jam,
        addr_mode: AddrMode::Jam,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Nop,
        addr_mode: AddrMode::Imm,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Jam,
        addr_mode: AddrMode::Jam,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Nop,
        addr_mode: AddrMode::Imm,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Jam,
        addr_mode: AddrMode::Jam,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Ldx,
        addr_mode: AddrMode::Imm,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Jam,
        addr_mode: AddrMode::Jam,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Nop,
        addr_mode: AddrMode::Imm,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Jam,
        addr_mode: AddrMode::Jam,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Jam,
        addr_mode: AddrMode::Jam,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Jam,
        addr_mode: AddrMode::Jam,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Jam,
        addr_mode: AddrMode::Jam,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Jam,
        addr_mode: AddrMode::Jam,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Jam,
        addr_mode: AddrMode::Jam,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Jam,
        addr_mode: AddrMode::Jam,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Jam,
        addr_mode: AddrMode::Jam,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Asl,
        addr_mode: AddrMode::A,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Txa,
        addr_mode: AddrMode::Impl,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Lsr,
        addr_mode: AddrMode::A,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Dex,
        addr_mode: AddrMode::Impl,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Rol,
        addr_mode: AddrMode::A,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Tax,
        addr_mode: AddrMode::Impl,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Ror,
        addr_mode: AddrMode::A,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Nop,
        addr_mode: AddrMode::Impl,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Nop,
        addr_mode: AddrMode::Impl,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Txs,
        addr_mode: AddrMode::Impl,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Nop,
        addr_mode: AddrMode::Impl,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Nop,
        addr_mode: AddrMode::Impl,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Nop,
        addr_mode: AddrMode::Impl,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Tsx,
        addr_mode: AddrMode::Impl,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Nop,
        addr_mode: AddrMode::Impl,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Nop,
        addr_mode: AddrMode::Impl,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Asl,
        addr_mode: AddrMode::Zpg,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Stx,
        addr_mode: AddrMode::Zpg,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Lsr,
        addr_mode: AddrMode::Zpg,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Dec,
        addr_mode: AddrMode::Zpg,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Rol,
        addr_mode: AddrMode::Zpg,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Ldx,
        addr_mode: AddrMode::Zpg,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Ror,
        addr_mode: AddrMode::Zpg,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Inc,
        addr_mode: AddrMode::Zpg,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Asl,
        addr_mode: AddrMode::ZpgIdx,
        addr_idx: AddrModeIdx::X,
    },
    Instruction {
        base_instr: BaseInstruction::Stx,
        addr_mode: AddrMode::ZpgIdx,
        addr_idx: AddrModeIdx::Y,
    },
    Instruction {
        base_instr: BaseInstruction::Lsr,
        addr_mode: AddrMode::ZpgIdx,
        addr_idx: AddrModeIdx::X,
    },
    Instruction {
        base_instr: BaseInstruction::Dec,
        addr_mode: AddrMode::ZpgIdx,
        addr_idx: AddrModeIdx::X,
    },
    Instruction {
        base_instr: BaseInstruction::Rol,
        addr_mode: AddrMode::ZpgIdx,
        addr_idx: AddrModeIdx::X,
    },
    Instruction {
        base_instr: BaseInstruction::Ldx,
        addr_mode: AddrMode::ZpgIdx,
        addr_idx: AddrModeIdx::Y,
    },
    Instruction {
        base_instr: BaseInstruction::Ror,
        addr_mode: AddrMode::ZpgIdx,
        addr_idx: AddrModeIdx::X,
    },
    Instruction {
        base_instr: BaseInstruction::Inc,
        addr_mode: AddrMode::ZpgIdx,
        addr_idx: AddrModeIdx::X,
    },
    Instruction {
        base_instr: BaseInstruction::Asl,
        addr_mode: AddrMode::Abs,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Stx,
        addr_mode: AddrMode::Abs,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Lsr,
        addr_mode: AddrMode::Abs,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Dec,
        addr_mode: AddrMode::Abs,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Rol,
        addr_mode: AddrMode::Abs,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Ldx,
        addr_mode: AddrMode::Abs,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Ror,
        addr_mode: AddrMode::Abs,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Inc,
        addr_mode: AddrMode::Abs,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Asl,
        addr_mode: AddrMode::AbsIdx,
        addr_idx: AddrModeIdx::X,
    },
    Instruction {
        base_instr: BaseInstruction::Shx,
        addr_mode: AddrMode::AbsIdx,
        addr_idx: AddrModeIdx::Y,
    },
    Instruction {
        base_instr: BaseInstruction::Lsr,
        addr_mode: AddrMode::AbsIdx,
        addr_idx: AddrModeIdx::X,
    },
    Instruction {
        base_instr: BaseInstruction::Dec,
        addr_mode: AddrMode::AbsIdx,
        addr_idx: AddrModeIdx::X,
    },
    Instruction {
        base_instr: BaseInstruction::Rol,
        addr_mode: AddrMode::AbsIdx,
        addr_idx: AddrModeIdx::X,
    },
    Instruction {
        base_instr: BaseInstruction::Ldx,
        addr_mode: AddrMode::AbsIdx,
        addr_idx: AddrModeIdx::Y,
    },
    Instruction {
        base_instr: BaseInstruction::Ror,
        addr_mode: AddrMode::AbsIdx,
        addr_idx: AddrModeIdx::X,
    },
    Instruction {
        base_instr: BaseInstruction::Inc,
        addr_mode: AddrMode::AbsIdx,
        addr_idx: AddrModeIdx::X,
    },
    Instruction {
        base_instr: BaseInstruction::Ora,
        addr_mode: AddrMode::IndIdx,
        addr_idx: AddrModeIdx::X,
    },
    Instruction {
        base_instr: BaseInstruction::Sta,
        addr_mode: AddrMode::IndIdx,
        addr_idx: AddrModeIdx::X,
    },
    Instruction {
        base_instr: BaseInstruction::Eor,
        addr_mode: AddrMode::IndIdx,
        addr_idx: AddrModeIdx::X,
    },
    Instruction {
        base_instr: BaseInstruction::Cmp,
        addr_mode: AddrMode::IndIdx,
        addr_idx: AddrModeIdx::X,
    },
    Instruction {
        base_instr: BaseInstruction::And,
        addr_mode: AddrMode::IndIdx,
        addr_idx: AddrModeIdx::X,
    },
    Instruction {
        base_instr: BaseInstruction::Lda,
        addr_mode: AddrMode::IndIdx,
        addr_idx: AddrModeIdx::X,
    },
    Instruction {
        base_instr: BaseInstruction::Adc,
        addr_mode: AddrMode::IndIdx,
        addr_idx: AddrModeIdx::X,
    },
    Instruction {
        base_instr: BaseInstruction::Sbc,
        addr_mode: AddrMode::IndIdx,
        addr_idx: AddrModeIdx::X,
    },
    Instruction {
        base_instr: BaseInstruction::Ora,
        addr_mode: AddrMode::IndIdx,
        addr_idx: AddrModeIdx::Y,
    },
    Instruction {
        base_instr: BaseInstruction::Sta,
        addr_mode: AddrMode::IndIdx,
        addr_idx: AddrModeIdx::Y,
    },
    Instruction {
        base_instr: BaseInstruction::Eor,
        addr_mode: AddrMode::IndIdx,
        addr_idx: AddrModeIdx::Y,
    },
    Instruction {
        base_instr: BaseInstruction::Cmp,
        addr_mode: AddrMode::IndIdx,
        addr_idx: AddrModeIdx::Y,
    },
    Instruction {
        base_instr: BaseInstruction::And,
        addr_mode: AddrMode::IndIdx,
        addr_idx: AddrModeIdx::Y,
    },
    Instruction {
        base_instr: BaseInstruction::Lda,
        addr_mode: AddrMode::IndIdx,
        addr_idx: AddrModeIdx::Y,
    },
    Instruction {
        base_instr: BaseInstruction::Adc,
        addr_mode: AddrMode::IndIdx,
        addr_idx: AddrModeIdx::Y,
    },
    Instruction {
        base_instr: BaseInstruction::Sbc,
        addr_mode: AddrMode::IndIdx,
        addr_idx: AddrModeIdx::Y,
    },
    Instruction {
        base_instr: BaseInstruction::Ora,
        addr_mode: AddrMode::Imm,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Nop,
        addr_mode: AddrMode::Imm,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Eor,
        addr_mode: AddrMode::Imm,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Cmp,
        addr_mode: AddrMode::Imm,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::And,
        addr_mode: AddrMode::Imm,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Lda,
        addr_mode: AddrMode::Imm,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Adc,
        addr_mode: AddrMode::Imm,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Sbc,
        addr_mode: AddrMode::Imm,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Ora,
        addr_mode: AddrMode::AbsIdx,
        addr_idx: AddrModeIdx::Y,
    },
    Instruction {
        base_instr: BaseInstruction::Sta,
        addr_mode: AddrMode::AbsIdx,
        addr_idx: AddrModeIdx::Y,
    },
    Instruction {
        base_instr: BaseInstruction::Eor,
        addr_mode: AddrMode::AbsIdx,
        addr_idx: AddrModeIdx::Y,
    },
    Instruction {
        base_instr: BaseInstruction::Cmp,
        addr_mode: AddrMode::AbsIdx,
        addr_idx: AddrModeIdx::Y,
    },
    Instruction {
        base_instr: BaseInstruction::And,
        addr_mode: AddrMode::AbsIdx,
        addr_idx: AddrModeIdx::Y,
    },
    Instruction {
        base_instr: BaseInstruction::Lda,
        addr_mode: AddrMode::AbsIdx,
        addr_idx: AddrModeIdx::Y,
    },
    Instruction {
        base_instr: BaseInstruction::Adc,
        addr_mode: AddrMode::AbsIdx,
        addr_idx: AddrModeIdx::Y,
    },
    Instruction {
        base_instr: BaseInstruction::Sbc,
        addr_mode: AddrMode::AbsIdx,
        addr_idx: AddrModeIdx::Y,
    },
    Instruction {
        base_instr: BaseInstruction::Ora,
        addr_mode: AddrMode::Zpg,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Sta,
        addr_mode: AddrMode::Zpg,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Eor,
        addr_mode: AddrMode::Zpg,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Cmp,
        addr_mode: AddrMode::Zpg,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::And,
        addr_mode: AddrMode::Zpg,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Lda,
        addr_mode: AddrMode::Zpg,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Adc,
        addr_mode: AddrMode::Zpg,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Sbc,
        addr_mode: AddrMode::Zpg,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Ora,
        addr_mode: AddrMode::ZpgIdx,
        addr_idx: AddrModeIdx::X,
    },
    Instruction {
        base_instr: BaseInstruction::Sta,
        addr_mode: AddrMode::ZpgIdx,
        addr_idx: AddrModeIdx::X,
    },
    Instruction {
        base_instr: BaseInstruction::Eor,
        addr_mode: AddrMode::ZpgIdx,
        addr_idx: AddrModeIdx::X,
    },
    Instruction {
        base_instr: BaseInstruction::Cmp,
        addr_mode: AddrMode::ZpgIdx,
        addr_idx: AddrModeIdx::X,
    },
    Instruction {
        base_instr: BaseInstruction::And,
        addr_mode: AddrMode::ZpgIdx,
        addr_idx: AddrModeIdx::X,
    },
    Instruction {
        base_instr: BaseInstruction::Lda,
        addr_mode: AddrMode::ZpgIdx,
        addr_idx: AddrModeIdx::X,
    },
    Instruction {
        base_instr: BaseInstruction::Adc,
        addr_mode: AddrMode::ZpgIdx,
        addr_idx: AddrModeIdx::X,
    },
    Instruction {
        base_instr: BaseInstruction::Sbc,
        addr_mode: AddrMode::ZpgIdx,
        addr_idx: AddrModeIdx::X,
    },
    Instruction {
        base_instr: BaseInstruction::Ora,
        addr_mode: AddrMode::Abs,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Sta,
        addr_mode: AddrMode::Abs,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Eor,
        addr_mode: AddrMode::Abs,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Cmp,
        addr_mode: AddrMode::Abs,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::And,
        addr_mode: AddrMode::Abs,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Lda,
        addr_mode: AddrMode::Abs,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Adc,
        addr_mode: AddrMode::Abs,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Sbc,
        addr_mode: AddrMode::Abs,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Ora,
        addr_mode: AddrMode::AbsIdx,
        addr_idx: AddrModeIdx::X,
    },
    Instruction {
        base_instr: BaseInstruction::Sta,
        addr_mode: AddrMode::AbsIdx,
        addr_idx: AddrModeIdx::X,
    },
    Instruction {
        base_instr: BaseInstruction::Eor,
        addr_mode: AddrMode::AbsIdx,
        addr_idx: AddrModeIdx::X,
    },
    Instruction {
        base_instr: BaseInstruction::Cmp,
        addr_mode: AddrMode::AbsIdx,
        addr_idx: AddrModeIdx::X,
    },
    Instruction {
        base_instr: BaseInstruction::And,
        addr_mode: AddrMode::AbsIdx,
        addr_idx: AddrModeIdx::X,
    },
    Instruction {
        base_instr: BaseInstruction::Lda,
        addr_mode: AddrMode::AbsIdx,
        addr_idx: AddrModeIdx::X,
    },
    Instruction {
        base_instr: BaseInstruction::Adc,
        addr_mode: AddrMode::AbsIdx,
        addr_idx: AddrModeIdx::X,
    },
    Instruction {
        base_instr: BaseInstruction::Sbc,
        addr_mode: AddrMode::AbsIdx,
        addr_idx: AddrModeIdx::X,
    },
    Instruction {
        base_instr: BaseInstruction::Slo,
        addr_mode: AddrMode::IndIdx,
        addr_idx: AddrModeIdx::X,
    },
    Instruction {
        base_instr: BaseInstruction::Sax,
        addr_mode: AddrMode::IndIdx,
        addr_idx: AddrModeIdx::X,
    },
    Instruction {
        base_instr: BaseInstruction::Sre,
        addr_mode: AddrMode::IndIdx,
        addr_idx: AddrModeIdx::X,
    },
    Instruction {
        base_instr: BaseInstruction::Dcp,
        addr_mode: AddrMode::IndIdx,
        addr_idx: AddrModeIdx::X,
    },
    Instruction {
        base_instr: BaseInstruction::Rla,
        addr_mode: AddrMode::IndIdx,
        addr_idx: AddrModeIdx::X,
    },
    Instruction {
        base_instr: BaseInstruction::Lax,
        addr_mode: AddrMode::IndIdx,
        addr_idx: AddrModeIdx::X,
    },
    Instruction {
        base_instr: BaseInstruction::Rra,
        addr_mode: AddrMode::IndIdx,
        addr_idx: AddrModeIdx::X,
    },
    Instruction {
        base_instr: BaseInstruction::Isc,
        addr_mode: AddrMode::IndIdx,
        addr_idx: AddrModeIdx::X,
    },
    Instruction {
        base_instr: BaseInstruction::Slo,
        addr_mode: AddrMode::IndIdx,
        addr_idx: AddrModeIdx::Y,
    },
    Instruction {
        base_instr: BaseInstruction::Sha,
        addr_mode: AddrMode::IndIdx,
        addr_idx: AddrModeIdx::Y,
    },
    Instruction {
        base_instr: BaseInstruction::Sre,
        addr_mode: AddrMode::IndIdx,
        addr_idx: AddrModeIdx::Y,
    },
    Instruction {
        base_instr: BaseInstruction::Dcp,
        addr_mode: AddrMode::IndIdx,
        addr_idx: AddrModeIdx::Y,
    },
    Instruction {
        base_instr: BaseInstruction::Rla,
        addr_mode: AddrMode::IndIdx,
        addr_idx: AddrModeIdx::Y,
    },
    Instruction {
        base_instr: BaseInstruction::Lax,
        addr_mode: AddrMode::IndIdx,
        addr_idx: AddrModeIdx::Y,
    },
    Instruction {
        base_instr: BaseInstruction::Rra,
        addr_mode: AddrMode::IndIdx,
        addr_idx: AddrModeIdx::Y,
    },
    Instruction {
        base_instr: BaseInstruction::Isc,
        addr_mode: AddrMode::IndIdx,
        addr_idx: AddrModeIdx::Y,
    },
    Instruction {
        base_instr: BaseInstruction::Anc,
        addr_mode: AddrMode::Imm,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Ane,
        addr_mode: AddrMode::Imm,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Alr,
        addr_mode: AddrMode::Imm,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Sbx,
        addr_mode: AddrMode::Imm,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Anc,
        addr_mode: AddrMode::Imm,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Lxa,
        addr_mode: AddrMode::Imm,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Arr,
        addr_mode: AddrMode::Imm,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Usbc,
        addr_mode: AddrMode::Imm,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Slo,
        addr_mode: AddrMode::AbsIdx,
        addr_idx: AddrModeIdx::Y,
    },
    Instruction {
        base_instr: BaseInstruction::Tas,
        addr_mode: AddrMode::AbsIdx,
        addr_idx: AddrModeIdx::Y,
    },
    Instruction {
        base_instr: BaseInstruction::Sre,
        addr_mode: AddrMode::AbsIdx,
        addr_idx: AddrModeIdx::Y,
    },
    Instruction {
        base_instr: BaseInstruction::Dcp,
        addr_mode: AddrMode::AbsIdx,
        addr_idx: AddrModeIdx::Y,
    },
    Instruction {
        base_instr: BaseInstruction::Rla,
        addr_mode: AddrMode::AbsIdx,
        addr_idx: AddrModeIdx::Y,
    },
    Instruction {
        base_instr: BaseInstruction::Las,
        addr_mode: AddrMode::AbsIdx,
        addr_idx: AddrModeIdx::Y,
    },
    Instruction {
        base_instr: BaseInstruction::Rra,
        addr_mode: AddrMode::AbsIdx,
        addr_idx: AddrModeIdx::Y,
    },
    Instruction {
        base_instr: BaseInstruction::Isc,
        addr_mode: AddrMode::AbsIdx,
        addr_idx: AddrModeIdx::Y,
    },
    Instruction {
        base_instr: BaseInstruction::Slo,
        addr_mode: AddrMode::Zpg,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Sax,
        addr_mode: AddrMode::Zpg,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Sre,
        addr_mode: AddrMode::Zpg,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Dcp,
        addr_mode: AddrMode::Zpg,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Rla,
        addr_mode: AddrMode::Zpg,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Lax,
        addr_mode: AddrMode::Zpg,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Rra,
        addr_mode: AddrMode::Zpg,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Isc,
        addr_mode: AddrMode::Zpg,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Slo,
        addr_mode: AddrMode::ZpgIdx,
        addr_idx: AddrModeIdx::X,
    },
    Instruction {
        base_instr: BaseInstruction::Sax,
        addr_mode: AddrMode::ZpgIdx,
        addr_idx: AddrModeIdx::Y,
    },
    Instruction {
        base_instr: BaseInstruction::Sre,
        addr_mode: AddrMode::ZpgIdx,
        addr_idx: AddrModeIdx::X,
    },
    Instruction {
        base_instr: BaseInstruction::Dcp,
        addr_mode: AddrMode::ZpgIdx,
        addr_idx: AddrModeIdx::X,
    },
    Instruction {
        base_instr: BaseInstruction::Rla,
        addr_mode: AddrMode::ZpgIdx,
        addr_idx: AddrModeIdx::X,
    },
    Instruction {
        base_instr: BaseInstruction::Lax,
        addr_mode: AddrMode::ZpgIdx,
        addr_idx: AddrModeIdx::Y,
    },
    Instruction {
        base_instr: BaseInstruction::Rra,
        addr_mode: AddrMode::ZpgIdx,
        addr_idx: AddrModeIdx::X,
    },
    Instruction {
        base_instr: BaseInstruction::Isc,
        addr_mode: AddrMode::ZpgIdx,
        addr_idx: AddrModeIdx::X,
    },
    Instruction {
        base_instr: BaseInstruction::Slo,
        addr_mode: AddrMode::Abs,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Sax,
        addr_mode: AddrMode::Abs,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Sre,
        addr_mode: AddrMode::Abs,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Dcp,
        addr_mode: AddrMode::Abs,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Rla,
        addr_mode: AddrMode::Abs,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Lax,
        addr_mode: AddrMode::Abs,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Rra,
        addr_mode: AddrMode::Abs,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Isc,
        addr_mode: AddrMode::Abs,
        addr_idx: AddrModeIdx::None,
    },
    Instruction {
        base_instr: BaseInstruction::Slo,
        addr_mode: AddrMode::AbsIdx,
        addr_idx: AddrModeIdx::X,
    },
    Instruction {
        base_instr: BaseInstruction::Sha,
        addr_mode: AddrMode::AbsIdx,
        addr_idx: AddrModeIdx::Y,
    },
    Instruction {
        base_instr: BaseInstruction::Sre,
        addr_mode: AddrMode::AbsIdx,
        addr_idx: AddrModeIdx::X,
    },
    Instruction {
        base_instr: BaseInstruction::Dcp,
        addr_mode: AddrMode::AbsIdx,
        addr_idx: AddrModeIdx::X,
    },
    Instruction {
        base_instr: BaseInstruction::Rla,
        addr_mode: AddrMode::AbsIdx,
        addr_idx: AddrModeIdx::X,
    },
    Instruction {
        base_instr: BaseInstruction::Lax,
        addr_mode: AddrMode::AbsIdx,
        addr_idx: AddrModeIdx::Y,
    },
    Instruction {
        base_instr: BaseInstruction::Rra,
        addr_mode: AddrMode::AbsIdx,
        addr_idx: AddrModeIdx::X,
    },
    Instruction {
        base_instr: BaseInstruction::Isc,
        addr_mode: AddrMode::AbsIdx,
        addr_idx: AddrModeIdx::X,
    },
];
use anyhow::anyhow;
use std::io::Read;
struct Cpu {
    a: Option<u8>,
    x: Option<u8>,
    y: Option<u8>,
    pc: u16,
    s: Option<u8>,
    n: Option<bool>,
    v: Option<bool>,
    b: Option<bool>,
    d: Option<bool>,
    i: Option<bool>,
    z: Option<bool>,
    c: Option<bool>,
}
impl Cpu {
    pub fn new(rom: &[u8]) -> Self {
        Self {
            a: None,
            x: None,
            y: None,
            pc: (u16::from(rom[0xFFD]) << 8) | u16::from(rom[0xFFC]),
            s: None,
            n: None,
            v: None,
            b: None,
            d: None,
            i: None,
            z: None,
            c: None,
        }
    }
    pub fn tick(&self, rom: &[u8], ram: &mut [Option<u8>]) -> anyhow::Result<()> {
        let opcode = if (self.pc >> 12) & 1 == 0 {
            ram[(self.pc & 0xFFF) as usize]
                .ok_or_else(|| ::anyhow::__private::must_use({
                    let error = ::anyhow::__private::format_err(
                        format_args!("attempted to access uninitialised RAM"),
                    );
                    error
                }))?
        } else {
            rom[(self.pc & 0xFFF) as usize]
        };
        let instr = OPCODE_LUT[usize::from(opcode)];
        Ok(())
    }
}
fn main() -> anyhow::Result<()> {
    let Some(path) = std::env::args().nth(1) else {
        return Err(
            ::anyhow::__private::must_use({
                let error = ::anyhow::__private::format_err(
                    format_args!("no path provided"),
                );
                error
            }),
        );
    };
    let mut rom = [0; 4096];
    std::fs::File::open(path)?.read_exact(&mut rom)?;
    let mut ram = [None; 4096];
    let cpu = Cpu::new(&rom);
    loop {
        cpu.tick(&rom, &mut ram)?;
    }
}
