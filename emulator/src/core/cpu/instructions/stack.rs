use crate::core::cpu::instructions::Register;
use crate::core::cpu::{CPU, CPULines};

pub fn push_rising(reg: Register, cpu: &mut CPU, lines: &mut CPULines) {
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

pub fn push_falling(cpu: &mut CPU) {
    if cpu.instruction_cycle == 1 {
        cpu.end_instruction();
    }

    cpu.instruction_cycle += 1;
}

pub fn pull_rising(cpu: &mut CPU, lines: &mut CPULines) {
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

pub fn pull_falling(reg: Register, cpu: &mut CPU, lines: &mut CPULines) {
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
    use crate::core::ReadOrWrite;
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

        tick_rising_test(&mut cpu, &mut address_bus, &mut data_bus, &mut rw_line);
        assert_eq!(address_bus.get_combined(), 0x67);
        assert_eq!(rw_line, ReadOrWrite::READ);
        tick_falling_test(&mut cpu, &mut address_bus, &mut data_bus, &mut rw_line);

        tick_rising_test(&mut cpu, &mut address_bus, &mut data_bus, &mut rw_line);
        assert_eq!(address_bus.get_combined(), 0x0112);
        assert_eq!(rw_line, ReadOrWrite::WRITE);

        match reg {
            Register::A => {
                assert_eq!(data_bus.get_combined(), 0xab);
            }
            Register::SR => {
                assert_eq!(data_bus.get_combined(), 0xcd);
            }
            _ => panic!(),
        }

        tick_falling_test(&mut cpu, &mut address_bus, &mut data_bus, &mut rw_line);
        assert_eq!(cpu.current_instruction, Instruction::Fetch);
    }

    // fn pl_generic(reg: Register) {
    //     match reg {
    //         Register::A => {
    //             assert_eq!(cpu.accumulator, 0b10010110);
    //             assert_eq!(cpu.get_negative_flag(), true);
    //             assert_eq!(cpu.get_zero_flag(), false);
    //         }
    //         Register::SR => {
    //             assert_eq!(cpu.status_register, 0b10010110);
    //         }
    //         _ => panic!(),
    //     }
    // }
}
