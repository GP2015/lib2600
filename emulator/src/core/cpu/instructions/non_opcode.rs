use crate::core::cpu::instructions;
use crate::core::cpu::{CPU, CPULines};

const PC_RESET_ADDR_LOW_BYTE: u16 = 0xfffc;
const PC_RESET_ADDR_HIGH_BYTE: u16 = 0xfffd;

pub fn reset_rising(cpu: &mut CPU, lines: &mut CPULines) {
    match cpu.instruction_cycle {
        5 => {
            cpu.read_from_address(PC_RESET_ADDR_LOW_BYTE, lines);
        }
        6 => {
            cpu.read_from_address(PC_RESET_ADDR_HIGH_BYTE, lines);
        }
        _ => {}
    }
}

pub fn reset_falling(cpu: &mut CPU, lines: &mut CPULines) {
    match cpu.instruction_cycle {
        5 => {
            // Set the low byte of the program counter
            cpu.program_counter &= 0xFF00;
            cpu.program_counter |= cpu.read_from_data_bus(lines) as u16;
        }
        6 => {
            // Set the high byte of the program counter
            cpu.program_counter &= 0x00FF;
            cpu.program_counter |= (cpu.read_from_data_bus(lines) as u16) << 8;
            cpu.end_instruction();
        }
        _ => {}
    }

    cpu.instruction_cycle += 1;
}

pub fn fetch_rising(cpu: &mut CPU, lines: &mut CPULines) {
    cpu.read_from_address(cpu.program_counter, lines);
}

pub fn fetch_falling(cpu: &mut CPU, lines: &mut CPULines) {
    let opcode = cpu.read_from_data_bus(lines);
    let (instruction, addressing_mode) = instructions::fetch_instruction(opcode);

    cpu.current_instruction = instruction;
    cpu.current_addressing_mode = addressing_mode;

    cpu.increment_program_counter();
    cpu.reset_instruction_vars();
}

#[cfg(test)]
mod tests {
    use crate::core::cpu::instructions::{AddressingMode, Instruction};
    use crate::core::cpu::test_functions::*;

    #[test]
    fn reset() {
        let (mut cpu, mut address_bus, mut data_bus, mut rw_line) = create_test_objects();

        for _ in 0..5 {
            tick_rising_test(&mut cpu, &mut address_bus, &mut data_bus, &mut rw_line);
            tick_falling_test(&mut cpu, &mut address_bus, &mut data_bus, &mut rw_line);
        }

        tick_rising_test(&mut cpu, &mut address_bus, &mut data_bus, &mut rw_line);
        assert_eq!(address_bus.get_combined(), 0x1ffc);
        tick_falling_test(&mut cpu, &mut address_bus, &mut data_bus, &mut rw_line);

        tick_rising_test(&mut cpu, &mut address_bus, &mut data_bus, &mut rw_line);
        assert_eq!(address_bus.get_combined(), 0x1ffd);
        tick_falling_test(&mut cpu, &mut address_bus, &mut data_bus, &mut rw_line);

        assert_eq!(cpu.current_instruction, Instruction::Fetch);
    }

    #[test]
    fn fetch() {
        let (mut cpu, mut address_bus, mut data_bus, mut rw_line) = create_test_objects();
        cpu.program_counter = 0x67;
        cpu.current_instruction = Instruction::Fetch;
        cpu.current_addressing_mode = AddressingMode::A;

        tick_rising_test(&mut cpu, &mut address_bus, &mut data_bus, &mut rw_line);
        assert_eq!(address_bus.get_combined(), 0x67);

        data_bus.set_combined(0xea);

        tick_falling_test(&mut cpu, &mut address_bus, &mut data_bus, &mut rw_line);
        assert_eq!(cpu.current_instruction, Instruction::NOP);
        assert_eq!(cpu.current_addressing_mode, AddressingMode::Impl);
        assert_eq!(cpu.program_counter, 0x68);
    }
}
