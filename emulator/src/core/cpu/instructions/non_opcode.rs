use crate::core::bus::Bus;
use crate::core::cpu::CPU;
use crate::core::cpu::instructions;

const PC_RESET_ADDR_LOW_BYTE: usize = 0xfffc;
const PC_RESET_ADDR_HIGH_BYTE: usize = 0xfffd;

pub fn reset_rising_edge(cpu: &mut CPU, address_bus: &mut Bus) {
    match cpu.instruction_cycle {
        5 => {
            address_bus.set_combined(PC_RESET_ADDR_LOW_BYTE);
        }
        6 => {
            address_bus.set_combined(PC_RESET_ADDR_HIGH_BYTE);
        }
        _ => {}
    }
}

pub fn reset_falling_edge(cpu: &mut CPU, data_bus: &mut Bus) {
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

pub fn fetch_rising_edge(cpu: &mut CPU, address_bus: &mut Bus) {
    address_bus.set_combined(cpu.program_counter as usize);
}

pub fn fetch_falling_edge(cpu: &mut CPU, data_bus: &mut Bus) {
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

    fn create_valid_objects() -> (Bus, Bus, CPU) {
        let address_bus = Bus::new(13);
        let data_bus = Bus::new(8);
        let cpu = CPU::new();
        (address_bus, data_bus, cpu)
    }

    #[test]
    fn reset() {
        let (mut address_bus, mut data_bus, mut cpu) = create_valid_objects();

        for _ in 0..5 {
            cpu.tick_rising_edge(&mut address_bus);
            cpu.tick_falling_edge(&mut data_bus);
        }

        cpu.tick_rising_edge(&mut address_bus);
        assert_eq!(address_bus.get_combined(), 0x1ffc);
        cpu.tick_falling_edge(&mut data_bus);

        cpu.tick_rising_edge(&mut address_bus);
        assert_eq!(address_bus.get_combined(), 0x1ffd);
        cpu.tick_falling_edge(&mut data_bus);

        assert_eq!(cpu.current_instruction, Instruction::Fetch);
    }

    #[test]
    fn fetch() {
        let (mut address_bus, mut data_bus, mut cpu) = create_valid_objects();
        cpu.program_counter = 0x67;
        cpu.current_instruction = Instruction::Fetch;
        cpu.current_addressing_mode = AddressingMode::A;

        cpu.tick_rising_edge(&mut address_bus);
        assert_eq!(address_bus.get_combined(), 0x67);

        data_bus.set_combined(0xea);

        cpu.tick_falling_edge(&mut data_bus);
        assert_eq!(cpu.current_instruction, Instruction::NOP);
        assert_eq!(cpu.current_addressing_mode, AddressingMode::Impl);
        assert_eq!(cpu.program_counter, 0x68);
    }
}
