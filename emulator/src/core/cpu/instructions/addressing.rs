use crate::core::bus::Bus;
use crate::core::cpu::CPU;
use crate::core::cpu::instructions::Register;

pub fn imm_rising_edge(cpu: &mut CPU, address_bus: &mut Bus) {
    address_bus.set_combined(cpu.program_counter as usize);
    cpu.increment_program_counter();
    cpu.end_addressing();
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
            cpu.end_addressing();
        }
        _ => (),
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
        _ => (),
    }

    cpu.addressing_cycle += 1;
}

pub fn abs_indexed_rising_edge(cpu: &mut CPU, address_bus: &mut Bus, reg: Register) {
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
            if !cpu.page_boundary_crossed {
                address_bus.set_combined(cpu.mid_instruction_address_hold as usize);
                cpu.end_addressing();
            }
        }
        3 => {
            address_bus.set_combined(cpu.mid_instruction_address_hold as usize);
            cpu.end_addressing();
        }
        _ => (),
    }
}

pub fn abs_indexed_falling_edge(cpu: &mut CPU, data_bus: &mut Bus, reg: Register) {
    match cpu.addressing_cycle {
        0 => {
            // Grab the low byte of the address.
            cpu.mid_instruction_address_hold = data_bus.get_combined() as u16;
        }
        1 => {
            // OR in the high byte of the address.
            cpu.mid_instruction_address_hold |= (data_bus.get_combined() as u16) << 8;

            let reg_value = match reg {
                Register::X => cpu.x_register,
                Register::Y => cpu.y_register,
            };

            let new_address = cpu
                .mid_instruction_address_hold
                .wrapping_add(reg_value as u16);

            cpu.page_boundary_crossed =
                new_address & 0xFF00 != cpu.mid_instruction_address_hold & 0xFF00;

            cpu.mid_instruction_address_hold = new_address;
        }
        _ => (),
    }

    cpu.addressing_cycle += 1;
}

pub fn zpg_rising_edge(cpu: &mut CPU, address_bus: &mut Bus) {
    match cpu.addressing_cycle {
        0 => {
            address_bus.set_combined(cpu.program_counter as usize);
            cpu.increment_program_counter();
        }
        1 => {
            address_bus.set_combined(cpu.mid_instruction_address_hold as usize);
            cpu.end_addressing();
        }
        _ => (),
    }
}

pub fn zpg_falling_edge(cpu: &mut CPU, data_bus: &mut Bus) {
    match cpu.addressing_cycle {
        0 => {
            // Grab the zeropage address.
            cpu.mid_instruction_address_hold = data_bus.get_combined() as u16;
        }
        _ => (),
    }

    cpu.addressing_cycle += 1;
}

pub fn zpg_indexed_rising_edge(cpu: &mut CPU, address_bus: &mut Bus, reg: Register) {
    match cpu.addressing_cycle {
        0 => {
            address_bus.set_combined(cpu.program_counter as usize);
            cpu.increment_program_counter();
        }
        2 => {
            address_bus.set_combined(cpu.mid_instruction_address_hold as usize);
            cpu.end_addressing();
        }
        _ => (),
    }
}

pub fn zpg_indexed_falling_edge(cpu: &mut CPU, data_bus: &mut Bus, reg: Register) {
    match cpu.addressing_cycle {
        0 => {
            // Grab the zeropage address.
            cpu.mid_instruction_address_hold = data_bus.get_combined() as u16;
        }
        1 => {
            cpu.mid_instruction_address_hold += match reg {
                Register::X => cpu.x_register as u16,
                Register::Y => cpu.y_register as u16,
            };

            cpu.mid_instruction_address_hold %= 256;
        }
        _ => (),
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
    fn imm() {
        let (mut address_bus, _, mut cpu) = create_valid_objects();
        cpu.program_counter = 0x67;
        cpu.current_instruction = Instruction::LDA;
        cpu.current_addressing_mode = AddressingMode::Imm;

        cpu.tick_rising_edge(&mut address_bus);
        assert_eq!(address_bus.get_combined(), 0x67);
        assert_eq!(cpu.program_counter, 0x68);
    }

    #[test]
    fn abs() {
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
        assert_eq!(cpu.program_counter, 0x69);
    }

    #[test]
    fn abs_x_indexed() {
        let (mut address_bus, mut data_bus, mut cpu) = create_valid_objects();
        cpu.program_counter = 0x67;
        cpu.x_register = 2;
        cpu.current_instruction = Instruction::LDA;
        cpu.current_addressing_mode = AddressingMode::AbsX;

        cpu.tick_rising_edge(&mut address_bus);
        assert_eq!(address_bus.get_combined(), 0x67);
        data_bus.set_combined(0x23);
        cpu.tick_falling_edge(&mut data_bus);

        cpu.tick_rising_edge(&mut address_bus);
        assert_eq!(address_bus.get_combined(), 0x68);
        data_bus.set_combined(0x01);
        cpu.tick_falling_edge(&mut data_bus);

        cpu.tick_rising_edge(&mut address_bus);
        assert_eq!(address_bus.get_combined(), 0x0125);
        assert_eq!(cpu.program_counter, 0x69);
    }

    #[test]
    fn abs_y_indexed() {
        let (mut address_bus, mut data_bus, mut cpu) = create_valid_objects();
        cpu.program_counter = 0x67;
        cpu.y_register = 2;
        cpu.current_instruction = Instruction::LDA;
        cpu.current_addressing_mode = AddressingMode::AbsY;

        cpu.tick_rising_edge(&mut address_bus);
        assert_eq!(address_bus.get_combined(), 0x67);
        data_bus.set_combined(0x23);
        cpu.tick_falling_edge(&mut data_bus);

        cpu.tick_rising_edge(&mut address_bus);
        assert_eq!(address_bus.get_combined(), 0x68);
        data_bus.set_combined(0x01);
        cpu.tick_falling_edge(&mut data_bus);

        cpu.tick_rising_edge(&mut address_bus);
        assert_eq!(address_bus.get_combined(), 0x0125);
        assert_eq!(cpu.program_counter, 0x69);
    }

    #[test]
    fn abs_indexed_page_cross() {
        let (mut address_bus, mut data_bus, mut cpu) = create_valid_objects();
        cpu.program_counter = 0x67;
        cpu.x_register = 2;
        cpu.current_instruction = Instruction::LDA;
        cpu.current_addressing_mode = AddressingMode::AbsX;

        cpu.tick_rising_edge(&mut address_bus);
        assert_eq!(address_bus.get_combined(), 0x67);
        data_bus.set_combined(0xFF);
        cpu.tick_falling_edge(&mut data_bus);

        cpu.tick_rising_edge(&mut address_bus);
        assert_eq!(address_bus.get_combined(), 0x68);
        data_bus.set_combined(0x00);
        cpu.tick_falling_edge(&mut data_bus);

        cpu.tick_rising_edge(&mut address_bus);
        cpu.tick_falling_edge(&mut data_bus);

        cpu.tick_rising_edge(&mut address_bus);
        assert_eq!(address_bus.get_combined(), 0x0101);
        assert_eq!(cpu.program_counter, 0x69);
    }

    #[test]
    fn zpg() {
        let (mut address_bus, mut data_bus, mut cpu) = create_valid_objects();
        cpu.program_counter = 0x67;
        cpu.current_instruction = Instruction::LDA;
        cpu.current_addressing_mode = AddressingMode::Zpg;

        cpu.tick_rising_edge(&mut address_bus);
        assert_eq!(address_bus.get_combined(), 0x67);
        data_bus.set_combined(0x12);
        cpu.tick_falling_edge(&mut data_bus);

        cpu.tick_rising_edge(&mut address_bus);
        assert_eq!(address_bus.get_combined(), 0x0012);
        assert_eq!(cpu.program_counter, 0x68);
    }

    #[test]
    fn zpg_x_indexed() {
        let (mut address_bus, mut data_bus, mut cpu) = create_valid_objects();
        cpu.program_counter = 0x67;
        cpu.x_register = 2;
        cpu.current_instruction = Instruction::LDA;
        cpu.current_addressing_mode = AddressingMode::ZpgX;

        cpu.tick_rising_edge(&mut address_bus);
        assert_eq!(address_bus.get_combined(), 0x67);
        data_bus.set_combined(0x12);
        cpu.tick_falling_edge(&mut data_bus);

        cpu.tick_rising_edge(&mut address_bus);
        cpu.tick_falling_edge(&mut data_bus);

        cpu.tick_rising_edge(&mut address_bus);
        assert_eq!(address_bus.get_combined(), 0x0014);
        assert_eq!(cpu.program_counter, 0x68);
    }

    #[test]
    fn zpg_y_indexed() {
        let (mut address_bus, mut data_bus, mut cpu) = create_valid_objects();
        cpu.program_counter = 0x67;
        cpu.y_register = 2;
        cpu.current_instruction = Instruction::LDA;
        cpu.current_addressing_mode = AddressingMode::ZpgY;

        cpu.tick_rising_edge(&mut address_bus);
        assert_eq!(address_bus.get_combined(), 0x67);
        data_bus.set_combined(0x12);
        cpu.tick_falling_edge(&mut data_bus);

        cpu.tick_rising_edge(&mut address_bus);
        cpu.tick_falling_edge(&mut data_bus);

        cpu.tick_rising_edge(&mut address_bus);
        assert_eq!(address_bus.get_combined(), 0x0014);
        assert_eq!(cpu.program_counter, 0x68);
    }

    #[test]
    fn zpg_indexed_page_cross() {
        let (mut address_bus, mut data_bus, mut cpu) = create_valid_objects();
        cpu.program_counter = 0x67;
        cpu.x_register = 2;
        cpu.current_instruction = Instruction::LDA;
        cpu.current_addressing_mode = AddressingMode::ZpgX;

        cpu.tick_rising_edge(&mut address_bus);
        assert_eq!(address_bus.get_combined(), 0x67);
        data_bus.set_combined(0xFF);
        cpu.tick_falling_edge(&mut data_bus);

        cpu.tick_rising_edge(&mut address_bus);
        cpu.tick_falling_edge(&mut data_bus);

        cpu.tick_rising_edge(&mut address_bus);
        assert_eq!(address_bus.get_combined(), 0x0001);
        assert_eq!(cpu.program_counter, 0x68);
    }
}
