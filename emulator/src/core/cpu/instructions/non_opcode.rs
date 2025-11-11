use crate::core::bus::Bus;
use crate::core::cpu::CPU;
use crate::core::cpu::instructions;

const PC_RESET_ADDR_LOW_BYTE: u16 = 0xfffc;
const PC_RESET_ADDR_HIGH_BYTE: u16 = 0xfffd;

pub fn reset_rising(cpu: &mut CPU, address_bus: &mut Bus, rw_line: &mut bool) {
    match cpu.instruction_cycle {
        5 => {
            cpu.write_to_address(PC_RESET_ADDR_LOW_BYTE, address_bus, rw_line);
        }
        6 => {
            cpu.write_to_address(PC_RESET_ADDR_HIGH_BYTE, address_bus, rw_line);
        }
        _ => {}
    }
}

pub fn reset_falling(cpu: &mut CPU, data_bus: &mut Bus, rw_line: &mut bool) {
    match cpu.instruction_cycle {
        5 => {
            // Set the low byte of the program counter
            cpu.program_counter &= 0xFF00;
            cpu.program_counter |= data_bus.get_combined() as u16;
        }
        6 => {
            // Set the high byte of the program counter
            cpu.program_counter &= 0x00FF;
            cpu.program_counter |= (data_bus.get_combined() as u16) << 8;
            cpu.end_instruction();
        }
        _ => {}
    }

    cpu.instruction_cycle += 1;
}

pub fn fetch_rising(cpu: &mut CPU, address_bus: &mut Bus, rw_line: &mut bool) {
    cpu.write_to_address(cpu.program_counter, address_bus, rw_line);
}

pub fn fetch_falling(cpu: &mut CPU, data_bus: &mut Bus) {
    let opcode = data_bus.get_combined() as u8;
    let (instruction, addressing_mode) = instructions::fetch_instruction(opcode);

    cpu.current_instruction = instruction;
    cpu.current_addressing_mode = addressing_mode;

    cpu.increment_program_counter();
    cpu.reset_instruction_vars();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::cpu::instructions::{AddressingMode, Instruction};

    fn create_valid_objects() -> (CPU, Bus, Bus, bool) {
        let cpu = CPU::new();
        let address_bus = Bus::new(13);
        let data_bus = Bus::new(8);
        let rw_line = false;
        (cpu, address_bus, data_bus, rw_line)
    }

    #[test]
    fn reset() {
        let (mut cpu, mut address_bus, mut data_bus, mut rw_line) = create_valid_objects();

        for _ in 0..5 {
            cpu.tick_rising(&mut address_bus, &mut rw_line);
            cpu.tick_falling(&mut data_bus, &mut rw_line);
        }

        cpu.tick_rising(&mut address_bus, &mut rw_line);
        assert_eq!(address_bus.get_combined(), 0x1ffc);
        cpu.tick_falling(&mut data_bus, &mut rw_line);

        cpu.tick_rising(&mut address_bus, &mut rw_line);
        assert_eq!(address_bus.get_combined(), 0x1ffd);
        cpu.tick_falling(&mut data_bus, &mut rw_line);

        assert_eq!(cpu.current_instruction, Instruction::Fetch);
    }

    #[test]
    fn fetch() {
        let (mut cpu, mut address_bus, mut data_bus, mut rw_line) = create_valid_objects();
        cpu.program_counter = 0x67;
        cpu.current_instruction = Instruction::Fetch;
        cpu.current_addressing_mode = AddressingMode::A;

        cpu.tick_rising(&mut address_bus, &mut rw_line);
        assert_eq!(address_bus.get_combined(), 0x67);

        data_bus.set_combined(0xea);

        cpu.tick_falling(&mut data_bus, &mut rw_line);
        assert_eq!(cpu.current_instruction, Instruction::NOP);
        assert_eq!(cpu.current_addressing_mode, AddressingMode::Impl);
        assert_eq!(cpu.program_counter, 0x68);
    }
}
