use crate::core::cpu::instructions::Register;
use crate::core::cpu::{CPU, CPULines};

pub fn imm_rising(cpu: &mut CPU) {
    cpu.effective_address = cpu.program_counter;
    cpu.increment_program_counter();
    cpu.end_addressing();
}

pub fn abs_rising(cpu: &mut CPU, lines: &mut CPULines) {
    match cpu.addressing_cycle {
        0 => {
            cpu.read_from_address(cpu.program_counter, lines);
            cpu.increment_program_counter();
        }
        1 => {
            cpu.read_from_address(cpu.program_counter, lines);
            cpu.increment_program_counter();
        }
        2 => {
            cpu.end_addressing();
        }
        _ => (),
    }
}

pub fn abs_falling(cpu: &mut CPU, lines: &mut CPULines) {
    match cpu.addressing_cycle {
        0 => {
            // Grab the low byte of the address.
            cpu.effective_address = cpu.read_from_data_bus(lines) as u16;
        }
        1 => {
            // OR in the high byte of the address.
            cpu.effective_address |= (cpu.read_from_data_bus(lines) as u16) << 8;
        }
        _ => (),
    }

    cpu.addressing_cycle += 1;
}

pub fn abs_indexed_rising(cpu: &mut CPU, lines: &mut CPULines) {
    match cpu.addressing_cycle {
        0 => {
            cpu.read_from_address(cpu.program_counter, lines);
            cpu.increment_program_counter();
        }
        1 => {
            cpu.read_from_address(cpu.program_counter, lines);
            cpu.increment_program_counter();
        }
        2 => {
            if !cpu.page_boundary_crossed {
                cpu.end_addressing();
            }
        }
        3 => {
            cpu.end_addressing();
        }
        _ => (),
    }
}

pub fn abs_indexed_falling(reg: Register, cpu: &mut CPU, lines: &mut CPULines) {
    match cpu.addressing_cycle {
        0 => {
            // Grab the low byte of the address.
            cpu.effective_address = cpu.read_from_data_bus(lines) as u16;
        }
        1 => {
            // OR in the high byte of the address.
            cpu.effective_address |= (cpu.read_from_data_bus(lines) as u16) << 8;

            let reg_value = match reg {
                Register::X => cpu.x_register,
                Register::Y => cpu.y_register,
                _ => panic!("Error: Invalid register."),
            };

            let new_address = cpu.effective_address.wrapping_add(reg_value as u16);
            cpu.page_boundary_crossed = new_address & 0xFF00 != cpu.effective_address & 0xFF00;
            cpu.effective_address = new_address;
        }
        _ => (),
    }

    cpu.addressing_cycle += 1;
}

pub fn zpg_rising(cpu: &mut CPU, lines: &mut CPULines) {
    match cpu.addressing_cycle {
        0 => {
            cpu.read_from_address(cpu.program_counter, lines);
            cpu.increment_program_counter();
        }
        1 => {
            cpu.end_addressing();
        }
        _ => (),
    }
}

pub fn zpg_falling(cpu: &mut CPU, lines: &mut CPULines) {
    match cpu.addressing_cycle {
        0 => {
            // Grab the zeropage address.
            cpu.effective_address = cpu.read_from_data_bus(lines) as u16;
        }
        _ => (),
    }

    cpu.addressing_cycle += 1;
}

pub fn zpg_indexed_rising(cpu: &mut CPU, lines: &mut CPULines) {
    match cpu.addressing_cycle {
        0 => {
            cpu.read_from_address(cpu.program_counter, lines);
            cpu.increment_program_counter();
        }
        2 => {
            cpu.end_addressing();
        }
        _ => (),
    }
}

pub fn zpg_indexed_falling(reg: Register, cpu: &mut CPU, lines: &mut CPULines) {
    match cpu.addressing_cycle {
        0 => {
            // Grab the zeropage address.
            cpu.effective_address = cpu.read_from_data_bus(lines) as u16;
        }
        1 => {
            cpu.effective_address += match reg {
                Register::X => cpu.x_register as u16,
                Register::Y => cpu.y_register as u16,
                _ => panic!("Error: Invalid register."),
            };

            cpu.effective_address %= 256;
        }
        _ => (),
    }

    cpu.addressing_cycle += 1;
}

#[cfg(test)]
mod tests {
    use crate::core::cpu::instructions::{AddressingMode, Instruction};
    use crate::core::cpu::test_functions::*;

    #[derive(Clone, Copy, Debug, PartialEq)]
    enum XOrY {
        X,
        Y,
    }

    #[test]
    fn imm() {
        let (mut cpu, mut address_bus, mut data_bus, mut rw_line) = create_test_objects();
        cpu.program_counter = 0x67;
        cpu.current_instruction = Instruction::LDA;
        cpu.current_addressing_mode = AddressingMode::Imm;

        tick_rising_test(&mut cpu, &mut address_bus, &mut data_bus, &mut rw_line);
        assert!(cpu.finished_addressing);
        assert_eq!(cpu.effective_address, 0x67);
        assert_eq!(cpu.program_counter, 0x68);
    }

    #[test]
    fn abs() {
        let (mut cpu, mut address_bus, mut data_bus, mut rw_line) = create_test_objects();
        cpu.program_counter = 0x67;
        cpu.current_instruction = Instruction::LDA;
        cpu.current_addressing_mode = AddressingMode::Abs;

        tick_rising_test(&mut cpu, &mut address_bus, &mut data_bus, &mut rw_line);
        assert_eq!(address_bus.get_combined(), 0x67);
        data_bus.set_combined(0x23);
        tick_falling_test(&mut cpu, &mut address_bus, &mut data_bus, &mut rw_line);

        tick_rising_test(&mut cpu, &mut address_bus, &mut data_bus, &mut rw_line);
        assert_eq!(address_bus.get_combined(), 0x68);
        data_bus.set_combined(0x01);
        tick_falling_test(&mut cpu, &mut address_bus, &mut data_bus, &mut rw_line);

        assert_eq!(cpu.effective_address, 0x0123);
        assert_eq!(cpu.program_counter, 0x69);

        tick_rising_test(&mut cpu, &mut address_bus, &mut data_bus, &mut rw_line);
        assert!(cpu.finished_addressing);
    }

    #[test]
    fn abs_x_indexed() {
        abs_indexed_generic(XOrY::X);
    }

    #[test]
    fn abs_y_indexed() {
        abs_indexed_generic(XOrY::Y);
    }

    fn abs_indexed_generic(x_or_y: XOrY) {
        let (mut cpu, mut address_bus, mut data_bus, mut rw_line) = create_test_objects();
        cpu.program_counter = 0x67;
        cpu.current_instruction = Instruction::LDA;

        match x_or_y {
            XOrY::X => cpu.x_register = 2,
            XOrY::Y => cpu.y_register = 2,
        }

        cpu.current_addressing_mode = match x_or_y {
            XOrY::X => AddressingMode::AbsX,
            XOrY::Y => AddressingMode::AbsY,
        };

        tick_rising_test(&mut cpu, &mut address_bus, &mut data_bus, &mut rw_line);
        assert_eq!(address_bus.get_combined(), 0x67);
        data_bus.set_combined(0x23);
        tick_falling_test(&mut cpu, &mut address_bus, &mut data_bus, &mut rw_line);

        tick_rising_test(&mut cpu, &mut address_bus, &mut data_bus, &mut rw_line);
        assert_eq!(address_bus.get_combined(), 0x68);
        data_bus.set_combined(0x01);
        tick_falling_test(&mut cpu, &mut address_bus, &mut data_bus, &mut rw_line);

        assert_eq!(cpu.effective_address, 0x0125);
        assert_eq!(cpu.program_counter, 0x69);

        tick_rising_test(&mut cpu, &mut address_bus, &mut data_bus, &mut rw_line);
        assert!(cpu.finished_addressing);
    }

    #[test]
    fn abs_indexed_page_cross() {
        let (mut cpu, mut address_bus, mut data_bus, mut rw_line) = create_test_objects();
        cpu.program_counter = 0x67;
        cpu.x_register = 2;
        cpu.current_instruction = Instruction::LDA;
        cpu.current_addressing_mode = AddressingMode::AbsX;

        tick_rising_test(&mut cpu, &mut address_bus, &mut data_bus, &mut rw_line);
        assert_eq!(address_bus.get_combined(), 0x67);
        data_bus.set_combined(0xFF);
        tick_falling_test(&mut cpu, &mut address_bus, &mut data_bus, &mut rw_line);

        tick_rising_test(&mut cpu, &mut address_bus, &mut data_bus, &mut rw_line);
        assert_eq!(address_bus.get_combined(), 0x68);
        data_bus.set_combined(0x00);
        tick_falling_test(&mut cpu, &mut address_bus, &mut data_bus, &mut rw_line);

        tick_rising_test(&mut cpu, &mut address_bus, &mut data_bus, &mut rw_line);
        tick_falling_test(&mut cpu, &mut address_bus, &mut data_bus, &mut rw_line);

        assert_eq!(cpu.effective_address, 0x0101);
        assert_eq!(cpu.program_counter, 0x69);

        tick_rising_test(&mut cpu, &mut address_bus, &mut data_bus, &mut rw_line);
        assert!(cpu.finished_addressing);
    }

    #[test]
    fn zpg() {
        let (mut cpu, mut address_bus, mut data_bus, mut rw_line) = create_test_objects();
        cpu.program_counter = 0x67;
        cpu.current_instruction = Instruction::LDA;
        cpu.current_addressing_mode = AddressingMode::Zpg;

        tick_rising_test(&mut cpu, &mut address_bus, &mut data_bus, &mut rw_line);
        assert_eq!(address_bus.get_combined(), 0x67);
        data_bus.set_combined(0x12);
        tick_falling_test(&mut cpu, &mut address_bus, &mut data_bus, &mut rw_line);

        assert_eq!(cpu.effective_address, 0x0012);
        assert_eq!(cpu.program_counter, 0x68);

        tick_rising_test(&mut cpu, &mut address_bus, &mut data_bus, &mut rw_line);
        assert!(cpu.finished_addressing);
    }

    #[test]
    fn zpg_x_indexed() {
        zpg_indexed_generic(XOrY::X);
    }

    #[test]
    fn zpg_y_indexed() {
        zpg_indexed_generic(XOrY::Y);
    }

    fn zpg_indexed_generic(x_or_y: XOrY) {
        let (mut cpu, mut address_bus, mut data_bus, mut rw_line) = create_test_objects();
        cpu.program_counter = 0x67;
        cpu.current_instruction = Instruction::LDA;

        match x_or_y {
            XOrY::X => cpu.x_register = 2,
            XOrY::Y => cpu.y_register = 2,
        }

        cpu.current_addressing_mode = match x_or_y {
            XOrY::X => AddressingMode::ZpgX,
            XOrY::Y => AddressingMode::ZpgY,
        };

        tick_rising_test(&mut cpu, &mut address_bus, &mut data_bus, &mut rw_line);
        assert_eq!(address_bus.get_combined(), 0x67);
        data_bus.set_combined(0x12);
        tick_falling_test(&mut cpu, &mut address_bus, &mut data_bus, &mut rw_line);

        tick_rising_test(&mut cpu, &mut address_bus, &mut data_bus, &mut rw_line);
        tick_falling_test(&mut cpu, &mut address_bus, &mut data_bus, &mut rw_line);

        assert_eq!(cpu.effective_address, 0x0014);
        assert_eq!(cpu.program_counter, 0x68);

        tick_rising_test(&mut cpu, &mut address_bus, &mut data_bus, &mut rw_line);
        assert!(cpu.finished_addressing);
    }

    #[test]
    fn zpg_indexed_page_cross() {
        let (mut cpu, mut address_bus, mut data_bus, mut rw_line) = create_test_objects();
        cpu.program_counter = 0x67;
        cpu.x_register = 2;
        cpu.current_instruction = Instruction::LDA;
        cpu.current_addressing_mode = AddressingMode::ZpgX;

        tick_rising_test(&mut cpu, &mut address_bus, &mut data_bus, &mut rw_line);
        assert_eq!(address_bus.get_combined(), 0x67);
        data_bus.set_combined(0xFF);
        tick_falling_test(&mut cpu, &mut address_bus, &mut data_bus, &mut rw_line);

        tick_rising_test(&mut cpu, &mut address_bus, &mut data_bus, &mut rw_line);
        tick_falling_test(&mut cpu, &mut address_bus, &mut data_bus, &mut rw_line);

        assert_eq!(cpu.effective_address, 0x0001);
        assert_eq!(cpu.program_counter, 0x68);

        tick_rising_test(&mut cpu, &mut address_bus, &mut data_bus, &mut rw_line);
        assert!(cpu.finished_addressing);
    }
}
