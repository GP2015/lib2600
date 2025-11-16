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
}
