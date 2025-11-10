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
            cpu.mid_instruction_address_hold |= data_bus.get_combined() as u16;
        }
        _ => panic!("Invalid address cycle reached in abs_falling_edge."),
    }

    cpu.addressing_cycle += 1;
}
