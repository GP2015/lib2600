use crate::core::cpu::instructions::Register;
use crate::core::cpu::{CPU, CPULines};

pub enum Type {
    Inc,
    Dec,
}

pub fn inc_reg_rise(reg: Register, inc_type: Type, cpu: &mut CPU, lines: &mut CPULines) {
    // Dummy read
    cpu.read_from_address(cpu.program_counter, lines);

    let mut value = match reg {
        Register::X => cpu.x_register,
        Register::Y => cpu.y_register,
        _ => panic!("Error: Invalid register."),
    };

    value = match inc_type {
        Type::Inc => value.wrapping_add(1),
        Type::Dec => value.wrapping_sub(1),
    };

    match reg {
        Register::X => cpu.set_x_register(value),
        Register::Y => cpu.set_y_register(value),
        _ => panic!("Error: Invalid register."),
    }
}

pub fn inc_reg_fall(cpu: &mut CPU) {
    cpu.end_instruction();
}

#[cfg(test)]
mod tests {
    use crate::core::ReadOrWrite;
    use crate::core::cpu::instructions::increment::Type;
    use crate::core::cpu::instructions::{Instruction, Register};
    use crate::core::cpu::test_functions::*;

    #[test]
    fn inx() {
        inc_reg_generic(Register::X, Type::Inc, Instruction::INX);
    }

    #[test]
    fn iny() {
        inc_reg_generic(Register::Y, Type::Inc, Instruction::INY);
    }

    #[test]
    fn dex() {
        inc_reg_generic(Register::X, Type::Dec, Instruction::DEX);
    }

    #[test]
    fn dey() {
        inc_reg_generic(Register::Y, Type::Dec, Instruction::DEY);
    }

    fn inc_reg_generic(reg: Register, inc_type: Type, instruction: Instruction) {
        let (mut cpu, mut address_bus, mut data_bus, mut rw_line) = create_test_objects();
        cpu.program_counter = 0x67;
        cpu.x_register = 0x12;
        cpu.y_register = 0x34;
        cpu.current_instruction = instruction;

        tick_rise_test(&mut cpu, &mut address_bus, &mut data_bus, &mut rw_line);
        assert_eq!(address_bus.get_combined(), 0x67);
        assert_eq!(rw_line, ReadOrWrite::Read);

        tick_fall_test(&mut cpu, &mut address_bus, &mut data_bus, &mut rw_line);
        assert_eq!(cpu.program_counter, 0x67);
        assert_eq!(cpu.current_instruction, Instruction::Fetch);

        match reg {
            Register::X => {
                assert_eq!(cpu.y_register, 0x34);
                match inc_type {
                    Type::Inc => assert_eq!(cpu.x_register, 0x13),
                    Type::Dec => assert_eq!(cpu.x_register, 0x11),
                }
            }
            Register::Y => {
                assert_eq!(cpu.x_register, 0x12);
                match inc_type {
                    Type::Inc => assert_eq!(cpu.y_register, 0x35),
                    Type::Dec => assert_eq!(cpu.y_register, 0x33),
                }
            }
            _ => panic!(),
        }
    }

    #[test]
    fn inc_reg_wrapping() {
        let (mut cpu, mut address_bus, mut data_bus, mut rw_line) = create_test_objects();
        cpu.x_register = 0xFF;
        cpu.current_instruction = Instruction::INX;

        tick_rise_test(&mut cpu, &mut address_bus, &mut data_bus, &mut rw_line);
        tick_fall_test(&mut cpu, &mut address_bus, &mut data_bus, &mut rw_line);
        assert_eq!(cpu.x_register, 0x00);
    }

    #[test]
    fn dec_reg_wrapping() {
        let (mut cpu, mut address_bus, mut data_bus, mut rw_line) = create_test_objects();
        cpu.x_register = 0x00;
        cpu.current_instruction = Instruction::DEX;

        tick_rise_test(&mut cpu, &mut address_bus, &mut data_bus, &mut rw_line);
        tick_fall_test(&mut cpu, &mut address_bus, &mut data_bus, &mut rw_line);
        assert_eq!(cpu.x_register, 0xFF);
    }
}
