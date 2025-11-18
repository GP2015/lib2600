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
