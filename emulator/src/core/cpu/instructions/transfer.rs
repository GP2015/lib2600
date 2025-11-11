use crate::core::bus::Bus;
use crate::core::cpu::CPU;
use crate::core::cpu::instructions;

pub fn lda_rising(cpu: &mut CPU, address_bus: &mut Bus, rw_line: &mut bool) {
    instructions::execute_addressing_rising(cpu, address_bus, rw_line);
}

pub fn lda_falling(cpu: &mut CPU, data_bus: &mut Bus) {
    if cpu.finished_addressing {
        cpu.accumulator = data_bus.get_combined() as u8;
        cpu.set_zero_flag_from_byte(cpu.accumulator);
        cpu.set_negative_flag_from_byte(cpu.accumulator);
        cpu.end_instruction();
    } else {
        instructions::execute_addressing_falling(cpu, data_bus);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::cpu::instructions::{AddressingMode, Instruction};

    fn create_valid_objects() -> (CPU, Bus, Bus, bool) {
        let cpu = CPU::new();
        let address_bus = Bus::new(13);
        let data_bus = Bus::new(8);
        let rw_line = false;
        (cpu, address_bus, data_bus, rw_line)
    }

    #[test]
    fn lda() {
        let (mut cpu, mut address_bus, mut data_bus, mut rw_line) = create_valid_objects();
        cpu.program_counter = 0x67;
        cpu.current_instruction = Instruction::LDA;
        cpu.current_addressing_mode = AddressingMode::Imm;

        cpu.tick_rising(&mut address_bus, &mut rw_line);
        data_bus.set_combined(0b10010110);
        cpu.tick_falling(&mut data_bus);

        assert_eq!(cpu.accumulator, 0b10010110);
        assert_eq!(cpu.get_negative_flag(), true);
        assert_eq!(cpu.get_zero_flag(), false);
    }
}
