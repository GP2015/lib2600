use crate::core::cpu::instructions::Register;
use crate::core::cpu::{CPU, CPULines};

pub fn push_rise(reg: Register, cpu: &mut CPU, lines: &mut CPULines) {
    match cpu.instruction_cycle {
        0 => {
            // Dummy read
            cpu.read_from_address(cpu.program_counter, lines);
        }
        1 => {
            let reg_value = match reg {
                Register::A => cpu.accumulator,
                Register::SR => cpu.status_register,
                _ => panic!("Error: Invalid register."),
            };

            let addr = cpu.get_stack_address();
            cpu.write_to_address(addr, reg_value, lines);

            cpu.decrement_stack_pointer();
        }
        _ => (),
    }
}

pub fn push_fall(cpu: &mut CPU) {
    if cpu.instruction_cycle == 1 {
        cpu.end_instruction();
    }

    cpu.instruction_cycle += 1;
}

pub fn pull_rise(cpu: &mut CPU, lines: &mut CPULines) {
    match cpu.instruction_cycle {
        0 => {
            // Dummy read
            cpu.read_from_address(cpu.program_counter, lines);
        }
        1 => {
            // Dummy read
            let addr = cpu.get_stack_address();
            cpu.read_from_address(addr, lines);

            cpu.increment_stack_pointer();
        }
        2 => {
            let addr = cpu.get_stack_address();
            cpu.read_from_address(addr, lines);
        }
        _ => (),
    }
}

pub fn pull_fall(reg: Register, cpu: &mut CPU, lines: &mut CPULines) {
    if cpu.instruction_cycle == 2 {
        let reg_value = cpu.read_from_data_bus(lines);

        match reg {
            Register::A => cpu.set_accumulator(reg_value),
            Register::SR => cpu.status_register = reg_value,
            _ => panic!("Error: Invalid register."),
        }

        cpu.end_instruction();
    }

    cpu.instruction_cycle += 1;
}

#[cfg(test)]
mod tests {
    use crate::core::cpu::instructions::{AddressingMode, Instruction, Register};
    use crate::core::cpu::test_functions::*;

    #[test]
    fn pha() {
        ph_generic(Register::A);
    }

    #[test]
    fn php() {
        ph_generic(Register::SR);
    }

    fn ph_generic(reg: Register) {
        let (mut cpu, mut address_bus, mut data_bus, mut rw_line) = create_test_objects();
        cpu.program_counter = 0x67;
        cpu.stack_pointer = 0x12;
        cpu.accumulator = 0xab;
        cpu.status_register = 0xcd;
        cpu.current_addressing_mode = AddressingMode::Impl;

        cpu.current_instruction = match reg {
            Register::A => Instruction::PHA,
            Register::SR => Instruction::PHP,
            _ => panic!(),
        };

        tick_rise_test(&mut cpu, &mut address_bus, &mut data_bus, &mut rw_line);
        check_addr_read(0x67, &mut address_bus, &mut rw_line);
        tick_fall_test(&mut cpu, &mut address_bus, &mut data_bus, &mut rw_line);

        tick_rise_test(&mut cpu, &mut address_bus, &mut data_bus, &mut rw_line);

        let data = match reg {
            Register::A => 0xab,
            Register::SR => 0xcd,
            _ => panic!(),
        };

        check_addr_write(0x0112, data, &mut address_bus, &mut data_bus, &mut rw_line);

        tick_fall_test(&mut cpu, &mut address_bus, &mut data_bus, &mut rw_line);
        assert_eq!(cpu.program_counter, 0x67);
        assert_eq!(cpu.stack_pointer, 0x11);
        assert_eq!(cpu.current_instruction, Instruction::Fetch);
    }

    #[test]
    fn pla() {
        pl_generic(Register::A);
    }

    #[test]
    fn plp() {
        pl_generic(Register::SR);
    }

    fn pl_generic(reg: Register) {
        let (mut cpu, mut address_bus, mut data_bus, mut rw_line) = create_test_objects();
        cpu.program_counter = 0x67;
        cpu.stack_pointer = 0x12;
        cpu.current_addressing_mode = AddressingMode::Impl;

        cpu.current_instruction = match reg {
            Register::A => Instruction::PLA,
            Register::SR => Instruction::PLP,
            _ => panic!(),
        };

        tick_rise_test(&mut cpu, &mut address_bus, &mut data_bus, &mut rw_line);
        check_addr_read(0x67, &mut address_bus, &mut rw_line);
        tick_fall_test(&mut cpu, &mut address_bus, &mut data_bus, &mut rw_line);

        tick_rise_test(&mut cpu, &mut address_bus, &mut data_bus, &mut rw_line);
        check_addr_read(0x0112, &mut address_bus, &mut rw_line);
        tick_fall_test(&mut cpu, &mut address_bus, &mut data_bus, &mut rw_line);
        assert_eq!(cpu.stack_pointer, 0x13);

        tick_rise_test(&mut cpu, &mut address_bus, &mut data_bus, &mut rw_line);
        check_addr_read(0x0113, &mut address_bus, &mut rw_line);
        data_bus.set_combined(0b10010110);

        tick_fall_test(&mut cpu, &mut address_bus, &mut data_bus, &mut rw_line);
        assert_eq!(cpu.program_counter, 0x67);
        assert_eq!(cpu.current_instruction, Instruction::Fetch);

        match reg {
            Register::A => {
                assert_eq!(cpu.accumulator, 0b10010110);
                assert_eq!(cpu.get_negative_flag(), true);
                assert_eq!(cpu.get_zero_flag(), false);
            }
            Register::SR => {
                assert_eq!(cpu.status_register, 0b10010110);
            }
            _ => panic!(),
        }
    }
}
