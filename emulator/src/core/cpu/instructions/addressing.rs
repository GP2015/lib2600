use crate::core::bus::Bus;
use crate::core::cpu::CPU;

pub fn imm_rising_edge(cpu: &mut CPU, address_bus: &mut Bus) {
    address_bus.set_combined(cpu.program_counter as usize);
    cpu.increment_program_counter();
    cpu.finished_addressing = true;
}

pub fn abs_rising_edge(cpu: &mut CPU, address_bus: &mut Bus) {
    match cpu.addressing_cycle {
        0 => {
            address_bus.set_combined(cpu.program_counter as usize);
            cpu.increment_program_counter();
        }
        1 => {
            address_bus.set_combined(cpu.program_counter as usize);
            cpu.increment_program_counter();
        }
        2 => {
            address_bus.set_combined(cpu.mid_instruction_address_hold as usize);
            cpu.finished_addressing = true;
        }
        _ => panic!("Invalid address cycle reached in abs_rising_edge."),
    }
}

pub fn abs_falling_edge(cpu: &mut CPU, data_bus: &mut Bus) {
    match cpu.addressing_cycle {
        0 => {
            // Grab the low byte of the address.
            cpu.mid_instruction_address_hold = data_bus.get_combined() as u16;
        }
        1 => {
            // OR in the high byte of the address.
            cpu.mid_instruction_address_hold |= (data_bus.get_combined() as u16) << 8;
        }
        _ => panic!("Invalid address cycle reached in abs_falling_edge."),
    }

    cpu.addressing_cycle += 1;
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
    fn imm_addressing() {
        let (mut address_bus, _, mut cpu) = create_valid_objects();
        cpu.program_counter = 0x67;
        cpu.current_instruction = Instruction::LDA;
        cpu.current_addressing_mode = AddressingMode::Imm;

        cpu.tick_rising_edge(&mut address_bus);
        assert_eq!(address_bus.get_combined(), 0x67);
    }

    #[test]
    fn abs_addressing() {
        let (mut address_bus, mut data_bus, mut cpu) = create_valid_objects();
        cpu.program_counter = 0x67;
        cpu.current_instruction = Instruction::LDA;
        cpu.current_addressing_mode = AddressingMode::Abs;

        cpu.tick_rising_edge(&mut address_bus);
        assert_eq!(address_bus.get_combined(), 0x67);
        data_bus.set_combined(0x23);
        cpu.tick_falling_edge(&mut data_bus);

        cpu.tick_rising_edge(&mut address_bus);
        assert_eq!(address_bus.get_combined(), 0x68);
        data_bus.set_combined(0x01);
        cpu.tick_falling_edge(&mut data_bus);

        cpu.tick_rising_edge(&mut address_bus);
        assert_eq!(address_bus.get_combined(), 0x0123);
    }
}
