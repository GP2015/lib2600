mod instructions;

use crate::core::lines::{Bus, ReadOrWrite};
use instructions::{AddressingMode, Instruction};

const PROGRAM_COUNTER_RESET_VALUE: u16 = 0x0000;
const ACCUMULATOR_RESET_VALUE: u8 = 0x00;
const X_REGISTER_RESET_VALUE: u8 = 0x00;
const Y_REGISTER_RESET_VALUE: u8 = 0x00;
const STATUS_REGISTER_RESET_VALUE: u8 = 0x00;
const STACK_POINTER_RESET_VALUE: u8 = 0xfd;

const NEGATIVE_FLAG_BIT: u8 = 7;
const OVERFLOW_FLAG_BIT: u8 = 6;
const BREAK_FLAG_BIT: u8 = 4;
const DECIMAL_FLAG_BIT: u8 = 3;
const INTERRUPT_FLAG_BIT: u8 = 2;
const ZERO_FLAG_BIT: u8 = 1;
const CARRY_FLAG_BIT: u8 = 0;

pub struct CPULines<'a> {
    address_bus: &'a mut Bus,
    data_bus: &'a mut Bus,
    rw_line: &'a mut ReadOrWrite,
}

impl<'a> CPULines<'a> {
    pub fn new(
        address_bus: &'a mut Bus,
        data_bus: &'a mut Bus,
        rw_line: &'a mut ReadOrWrite,
    ) -> Self {
        Self {
            address_bus,
            data_bus,
            rw_line,
        }
    }
}

pub struct CPU {
    program_counter: u16,
    accumulator: u8,
    x_register: u8,
    y_register: u8,
    status_register: u8,
    stack_pointer: u8,

    current_instruction: Instruction,
    current_addressing_mode: AddressingMode,
    instruction_cycle: usize,
    addressing_cycle: usize,
    finished_addressing: bool,
    effective_address: u16,
    mid_instruction_byte_hold: u8,
    page_boundary_crossed: bool,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            program_counter: PROGRAM_COUNTER_RESET_VALUE,
            accumulator: ACCUMULATOR_RESET_VALUE,
            x_register: X_REGISTER_RESET_VALUE,
            y_register: Y_REGISTER_RESET_VALUE,
            status_register: STATUS_REGISTER_RESET_VALUE,
            stack_pointer: STACK_POINTER_RESET_VALUE,

            current_instruction: Instruction::Reset,
            current_addressing_mode: AddressingMode::Impl,
            instruction_cycle: 0,
            addressing_cycle: 0,
            finished_addressing: false,
            effective_address: 0,
            mid_instruction_byte_hold: 0,
            page_boundary_crossed: false,
        }
    }

    pub fn tick_rising(&mut self, mut lines: CPULines) {
        instructions::execute_instruction_rising(self, &mut lines);
    }

    pub fn tick_falling(&mut self, mut lines: CPULines) {
        instructions::execute_instruction_falling(self, &mut lines);
    }

    pub fn reset(&mut self) {
        self.current_instruction = Instruction::Reset;
        self.reset_instruction_vars();
    }

    fn end_addressing(&mut self) {
        self.finished_addressing = true;
    }

    fn end_instruction(&mut self) {
        self.current_instruction = Instruction::Fetch;
        self.reset_instruction_vars();
    }

    fn reset_instruction_vars(&mut self) {
        self.instruction_cycle = 0;
        self.addressing_cycle = 0;
        self.finished_addressing = false;
    }

    fn increment_program_counter(&mut self) {
        self.program_counter = self.program_counter.wrapping_add(1);
    }

    fn write_to_address(&mut self, addr: u16, value: u8, lines: &mut CPULines) {
        lines.address_bus.set_combined(addr as usize);
        lines.data_bus.set_combined(value as usize);
        *lines.rw_line = ReadOrWrite::WRITE;
    }

    fn read_from_address(&mut self, addr: u16, lines: &mut CPULines) {
        lines.address_bus.set_combined(addr as usize);
        *lines.rw_line = ReadOrWrite::READ;
    }

    fn read_from_data_bus(&mut self, lines: &mut CPULines) -> u8 {
        lines.data_bus.get_combined() as u8
    }

    fn set_accumulator(&mut self, value: u8) {
        self.accumulator = value;
        self.set_negative_flag_from_byte(value);
        self.set_zero_flag_from_byte(value);
    }

    fn set_x_register(&mut self, value: u8) {
        self.x_register = value;
        self.set_negative_flag_from_byte(value);
        self.set_zero_flag_from_byte(value);
    }

    fn set_y_register(&mut self, value: u8) {
        self.y_register = value;
        self.set_negative_flag_from_byte(value);
        self.set_zero_flag_from_byte(value);
    }

    fn set_status_line(&mut self, line: u8, state: bool) {
        self.status_register = match state {
            true => self.status_register | (1 << line),
            false => self.status_register & !(1 << line),
        };
    }

    fn get_status_line(&self, line: u8) -> bool {
        (self.status_register >> line) & 1 == 1
    }

    fn set_negative_flag_from_byte(&mut self, byte: u8) {
        let is_negative = (byte >> 7) == 1;
        self.set_status_line(NEGATIVE_FLAG_BIT, is_negative);
    }

    fn get_negative_flag(&self) -> bool {
        self.get_status_line(NEGATIVE_FLAG_BIT)
    }

    fn set_zero_flag_from_byte(&mut self, byte: u8) {
        self.set_status_line(ZERO_FLAG_BIT, byte == 0);
    }

    fn get_zero_flag(&self) -> bool {
        self.get_status_line(ZERO_FLAG_BIT)
    }
}

#[cfg(test)]
mod test_functions {
    use super::*;

    pub fn create_test_objects() -> (CPU, Bus, Bus, ReadOrWrite) {
        let cpu = CPU::new();
        let address_bus = Bus::new(13);
        let data_bus = Bus::new(8);
        let rw_line = ReadOrWrite::READ;
        (cpu, address_bus, data_bus, rw_line)
    }

    pub fn tick_rising_test(
        cpu: &mut CPU,
        address_bus: &mut Bus,
        data_bus: &mut Bus,
        rw_line: &mut ReadOrWrite,
    ) {
        let lines = CPULines::new(address_bus, data_bus, rw_line);
        cpu.tick_rising(lines);
    }

    pub fn tick_falling_test(
        cpu: &mut CPU,
        address_bus: &mut Bus,
        data_bus: &mut Bus,
        rw_line: &mut ReadOrWrite,
    ) {
        let lines = CPULines::new(address_bus, data_bus, rw_line);
        cpu.tick_falling(lines);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::cpu::test_functions::*;

    #[test]
    fn reset() {
        let (mut cpu, _, _, _) = create_test_objects();
        cpu.current_instruction = Instruction::NOP;

        cpu.reset();

        assert_eq!(cpu.current_instruction, Instruction::Reset);
        assert_eq!(cpu.instruction_cycle, 0);
        assert_eq!(cpu.addressing_cycle, 0);
        assert!(!cpu.finished_addressing);
    }

    #[test]
    fn end_instruction() {
        let (mut cpu, _, _, _) = create_test_objects();
        cpu.current_instruction = Instruction::NOP;

        cpu.end_instruction();

        assert_eq!(cpu.current_instruction, Instruction::Fetch);
        assert_eq!(cpu.instruction_cycle, 0);
        assert_eq!(cpu.addressing_cycle, 0);
        assert!(!cpu.finished_addressing);
    }

    #[test]
    fn increment_program_counter() {
        let (mut cpu, _, _, _) = create_test_objects();
        cpu.program_counter = 0x67;

        cpu.increment_program_counter();

        assert_eq!(cpu.program_counter, 0x68);
    }

    #[test]
    fn increment_program_counter_wrapping() {
        let (mut cpu, _, _, _) = create_test_objects();
        cpu.program_counter = 0xFFFF;

        cpu.increment_program_counter();

        assert_eq!(cpu.program_counter, 0x0000);
    }

    #[test]
    fn write_to_address() {
        let (mut cpu, mut address_bus, mut data_bus, mut rw_line) = create_test_objects();
        let mut lines = CPULines::new(&mut address_bus, &mut data_bus, &mut rw_line);

        cpu.write_to_address(0x1234, 0x67, &mut lines);
        assert_eq!(address_bus.get_combined(), 0x1234);
        assert_eq!(data_bus.get_combined(), 0x67);
        assert_eq!(rw_line, ReadOrWrite::WRITE);
    }

    #[test]
    fn read_from_address() {
        let (mut cpu, mut address_bus, mut data_bus, mut rw_line) = create_test_objects();
        let mut lines = CPULines::new(&mut address_bus, &mut data_bus, &mut rw_line);

        cpu.read_from_address(0x1234, &mut lines);
        assert_eq!(address_bus.get_combined(), 0x1234);
        assert_eq!(rw_line, ReadOrWrite::READ);
    }

    #[test]
    fn read_from_data_bus() {
        let (mut cpu, mut address_bus, mut data_bus, mut rw_line) = create_test_objects();
        data_bus.set_combined(0x67);
        let mut lines = CPULines::new(&mut address_bus, &mut data_bus, &mut rw_line);

        assert_eq!(cpu.read_from_data_bus(&mut lines), 0x67);
    }
}
