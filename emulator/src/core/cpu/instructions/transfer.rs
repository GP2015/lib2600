use crate::core::cpu::instructions::{self, Register};
use crate::core::cpu::{CPU, CPULines};

pub fn load_rising(cpu: &mut CPU, lines: &mut CPULines) {
    instructions::execute_addressing_rising(cpu, lines);

    if cpu.finished_addressing {
        cpu.read_from_address(cpu.effective_address, lines);
    }
}

pub fn load_falling(reg: Register, cpu: &mut CPU, lines: &mut CPULines) {
    if !cpu.finished_addressing {
        instructions::execute_addressing_falling(cpu, lines);
        return;
    }

    let new_value = cpu.read_from_data_bus(lines);
    cpu.set_zero_flag_from_byte(new_value);
    cpu.set_negative_flag_from_byte(new_value);

    match reg {
        Register::A => cpu.accumulator = new_value,
        Register::X => cpu.x_register = new_value,
        Register::Y => cpu.y_register = new_value,
        _ => panic!("Error: Invalid register."),
    }

    cpu.end_instruction();
}

pub fn store_rising(cpu: &mut CPU, lines: &mut CPULines) {
    instructions::execute_addressing_rising(cpu, lines);
}

pub fn store_falling(reg: Register, cpu: &mut CPU, lines: &mut CPULines) {
    if !cpu.finished_addressing {
        instructions::execute_addressing_falling(cpu, lines);
        return;
    }

    let new_value = cpu.read_from_data_bus(lines);

    match reg {
        Register::A => cpu.accumulator = new_value,
        Register::X => cpu.x_register = new_value,
        Register::Y => cpu.y_register = new_value,
        _ => panic!("Error: Invalid register."),
    }

    cpu.end_instruction();
}

#[cfg(test)]
mod tests {
    use crate::core::cpu::instructions::{AddressingMode, Instruction};
    use crate::core::cpu::test_functions::*;

    #[test]
    fn lda() {
        let (mut cpu, mut address_bus, mut data_bus, mut rw_line) = create_test_objects();
        cpu.program_counter = 0x67;
        cpu.current_instruction = Instruction::LDA;
        cpu.current_addressing_mode = AddressingMode::Imm;

        tick_rising_test(&mut cpu, &mut address_bus, &mut data_bus, &mut rw_line);
        data_bus.set_combined(0b10010110);
        tick_falling_test(&mut cpu, &mut address_bus, &mut data_bus, &mut rw_line);

        assert_eq!(cpu.accumulator, 0b10010110);
        assert_eq!(cpu.get_negative_flag(), true);
        assert_eq!(cpu.get_zero_flag(), false);
    }

    #[test]
    fn ldx() {
        let (mut cpu, mut address_bus, mut data_bus, mut rw_line) = create_test_objects();
        cpu.program_counter = 0x67;
        cpu.current_instruction = Instruction::LDX;
        cpu.current_addressing_mode = AddressingMode::Imm;

        tick_rising_test(&mut cpu, &mut address_bus, &mut data_bus, &mut rw_line);
        data_bus.set_combined(0b10010110);
        tick_falling_test(&mut cpu, &mut address_bus, &mut data_bus, &mut rw_line);

        assert_eq!(cpu.x_register, 0b10010110);
        assert_eq!(cpu.get_negative_flag(), true);
        assert_eq!(cpu.get_zero_flag(), false);
    }

    #[test]
    fn ldy() {
        let (mut cpu, mut address_bus, mut data_bus, mut rw_line) = create_test_objects();
        cpu.program_counter = 0x67;
        cpu.current_instruction = Instruction::LDY;
        cpu.current_addressing_mode = AddressingMode::Imm;

        tick_rising_test(&mut cpu, &mut address_bus, &mut data_bus, &mut rw_line);
        data_bus.set_combined(0b10010110);
        tick_falling_test(&mut cpu, &mut address_bus, &mut data_bus, &mut rw_line);

        assert_eq!(cpu.y_register, 0b10010110);
        assert_eq!(cpu.get_negative_flag(), true);
        assert_eq!(cpu.get_zero_flag(), false);
    }
}
