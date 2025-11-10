use crate::core::bus::Bus;
use crate::core::cpu::CPU;
use crate::core::cpu::instructions;

pub fn lda_rising_edge(cpu: &mut CPU, address_bus: &mut Bus) {
    instructions::execute_addressing_rising_edge(cpu, address_bus);
}

pub fn lda_falling_edge(cpu: &mut CPU, data_bus: &mut Bus) {
    if cpu.finished_addressing {
        cpu.accumulator = data_bus.get_combined() as u8;
        cpu.end_instruction();
    }
}
