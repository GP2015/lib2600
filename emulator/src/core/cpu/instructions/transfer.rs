use crate::core::cpu::instructions::{self, Register};
use crate::core::cpu::{CPU, CPULines};

pub fn load_rising(cpu: &mut CPU, lines: &mut CPULines) {
    if !instructions::execute_addressing_rising(cpu, lines) {
        return;
    }

    cpu.read_from_address(cpu.effective_address, lines);
}

pub fn load_falling(reg: Register, cpu: &mut CPU, lines: &mut CPULines) {
    if !cpu.finished_addressing {
        instructions::execute_addressing_falling(cpu, lines);
        return;
    }

    let new_value = cpu.read_from_data_bus(lines);

    match reg {
        Register::A => cpu.set_accumulator(new_value),
        Register::X => cpu.set_x_register(new_value),
        Register::Y => cpu.set_y_register(new_value),
        _ => panic!("Error: Invalid register."),
    }

    cpu.end_instruction();
}

pub fn store_rising(reg: Register, cpu: &mut CPU, lines: &mut CPULines) {
    if !instructions::execute_addressing_rising(cpu, lines) {
        return;
    }

    let reg_value = match reg {
        Register::A => cpu.accumulator,
        Register::X => cpu.x_register,
        Register::Y => cpu.y_register,
        _ => panic!("Error: Invalid register."),
    };

    cpu.write_to_address(cpu.effective_address, reg_value, lines);
}

pub fn store_falling(cpu: &mut CPU, lines: &mut CPULines) {
    if !cpu.finished_addressing {
        instructions::execute_addressing_falling(cpu, lines);
    }

    cpu.end_instruction();
}

pub fn transfer_rising(from_reg: Register, to_reg: Register, cpu: &mut CPU, lines: &mut CPULines) {
    if !instructions::execute_addressing_rising(cpu, lines) {
        return;
    }

    let value = match from_reg {
        Register::A => cpu.accumulator,
        Register::X => cpu.x_register,
        Register::Y => cpu.y_register,
        Register::SP => cpu.stack_pointer,
        _ => panic!("Error: Invalid register."),
    };

    match to_reg {
        Register::A => cpu.set_accumulator(value),
        Register::X => cpu.set_x_register(value),
        Register::Y => cpu.set_y_register(value),
        Register::SP => cpu.stack_pointer = value,
        _ => panic!("Error: Invalid register."),
    }
}

pub fn transfer_falling(cpu: &mut CPU, lines: &mut CPULines) {
    if !cpu.finished_addressing {
        instructions::execute_addressing_falling(cpu, lines);
    }

    cpu.end_instruction();
}

#[cfg(test)]
mod tests {
    use crate::core::ReadOrWrite;
    use crate::core::cpu::instructions::{AddressingMode, Instruction, Register};
    use crate::core::cpu::test_functions::*;

    #[test]
    fn lda() {
        ld_generic(Register::A);
    }

    #[test]
    fn ldx() {
        ld_generic(Register::X);
    }

    #[test]
    fn ldy() {
        ld_generic(Register::Y);
    }

    fn ld_generic(reg: Register) {
        let (mut cpu, mut address_bus, mut data_bus, mut rw_line) = create_test_objects();
        cpu.current_addressing_mode = AddressingMode::Imm;

        cpu.current_instruction = match reg {
            Register::A => Instruction::LDA,
            Register::X => Instruction::LDX,
            Register::Y => Instruction::LDY,
            _ => panic!(),
        };

        tick_rising_test(&mut cpu, &mut address_bus, &mut data_bus, &mut rw_line);
        data_bus.set_combined(0b10010110);
        tick_falling_test(&mut cpu, &mut address_bus, &mut data_bus, &mut rw_line);

        assert_eq!(cpu.current_instruction, Instruction::Fetch);
        assert_eq!(cpu.get_negative_flag(), true);
        assert_eq!(cpu.get_zero_flag(), false);

        assert_eq!(
            match reg {
                Register::A => cpu.accumulator,
                Register::X => cpu.x_register,
                Register::Y => cpu.y_register,
                _ => panic!(),
            },
            0b10010110
        );
    }

    #[test]
    fn sta() {
        st_generic(Register::A);
    }

    #[test]
    fn stx() {
        st_generic(Register::X);
    }

    #[test]
    fn sty() {
        st_generic(Register::Y);
    }

    fn st_generic(reg: Register) {
        let (mut cpu, mut address_bus, mut data_bus, mut rw_line) = create_test_objects();
        cpu.program_counter = 0x67;
        cpu.current_addressing_mode = AddressingMode::Imm;

        cpu.current_instruction = match reg {
            Register::A => Instruction::STA,
            Register::X => Instruction::STX,
            Register::Y => Instruction::STY,
            _ => panic!(),
        };

        match reg {
            Register::A => cpu.accumulator = 0x12,
            Register::X => cpu.x_register = 0x12,
            Register::Y => cpu.y_register = 0x12,
            _ => panic!(),
        };

        tick_rising_test(&mut cpu, &mut address_bus, &mut data_bus, &mut rw_line);
        assert_eq!(address_bus.get_combined(), 0x67);
        assert_eq!(data_bus.get_combined(), 0x12);
        assert_eq!(rw_line, ReadOrWrite::WRITE);

        tick_falling_test(&mut cpu, &mut address_bus, &mut data_bus, &mut rw_line);
        assert_eq!(cpu.current_instruction, Instruction::Fetch);
    }

    #[test]
    fn tax() {
        t_generic(Instruction::TAX, Register::A, Register::X);
    }

    #[test]
    fn tay() {
        t_generic(Instruction::TAY, Register::A, Register::Y);
    }

    #[test]
    fn tsx() {
        t_generic(Instruction::TSX, Register::SP, Register::X);
    }

    #[test]
    fn txa() {
        t_generic(Instruction::TXA, Register::X, Register::A);
    }

    #[test]
    fn txs() {
        t_generic(Instruction::TXS, Register::X, Register::SP);
    }

    #[test]
    fn tya() {
        t_generic(Instruction::TYA, Register::Y, Register::A);
    }

    fn t_generic(instruction: Instruction, from_reg: Register, to_reg: Register) {
        let (mut cpu, mut address_bus, mut data_bus, mut rw_line) = create_test_objects();
        cpu.program_counter = 0x67;
        cpu.current_instruction = instruction;
        cpu.current_addressing_mode = AddressingMode::Imm;
        cpu.set_negative_flag_from_byte(0);
        cpu.set_zero_flag_from_byte(0);

        match from_reg {
            Register::A => cpu.accumulator = 0b10010110,
            Register::X => cpu.x_register = 0b10010110,
            Register::Y => cpu.y_register = 0b10010110,
            Register::SP => cpu.stack_pointer = 0b10010110,
            _ => panic!(),
        };

        tick_rising_test(&mut cpu, &mut address_bus, &mut data_bus, &mut rw_line);
        tick_falling_test(&mut cpu, &mut address_bus, &mut data_bus, &mut rw_line);

        assert_eq!(cpu.current_instruction, Instruction::Fetch);

        if matches!(to_reg, Register::SP) {
            assert_eq!(cpu.get_negative_flag(), false);
            assert_eq!(cpu.get_zero_flag(), true);
        } else {
            assert_eq!(cpu.get_negative_flag(), true);
            assert_eq!(cpu.get_zero_flag(), false);
        }

        assert_eq!(
            match to_reg {
                Register::A => cpu.accumulator,
                Register::X => cpu.x_register,
                Register::Y => cpu.y_register,
                Register::SP => cpu.stack_pointer,
                _ => panic!(),
            },
            0b10010110
        );
    }
}
