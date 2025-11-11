use crate::core::bus::Bus;
use crate::core::cpu::CPU;
use crate::core::cpu::instructions;

pub fn lda_rising(cpu: &mut CPU, address_bus: &mut Bus, rw_line: &mut bool) {
    instructions::execute_addressing_rising(cpu, address_bus, rw_line);
}

pub fn lda_falling(cpu: &mut CPU, data_bus: &mut Bus, rw_line: &mut bool) {
    if cpu.finished_addressing {
        cpu.accumulator = data_bus.get_combined() as u8;
        cpu.end_instruction();
    } else {
        instructions::execute_addressing_falling(cpu, data_bus, rw_line);
    }
}
