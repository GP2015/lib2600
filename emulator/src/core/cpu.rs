mod instructions;

use crate::core::bus::Bus;
use instructions::{AddressingMode, Instruction};

const PROGRAM_COUNTER_RESET_VALUE: u16 = 0x0000;
const ACCUMULATOR_RESET_VALUE: u8 = 0x00;
const X_REGISTER_RESET_VALUE: u8 = 0x00;
const Y_REGISTER_RESET_VALUE: u8 = 0x00;
const STATUS_REGISTER_RESET_VALUE: u8 = 0x00;
const STACK_POINTER_RESET_VALUE: u8 = 0xfd;

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
    mid_instruction_address_hold: u16,
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
            mid_instruction_address_hold: 0,
            page_boundary_crossed: false,
        }
    }

    pub fn tick_rising(&mut self, address_bus: &mut Bus, rw_line: &mut bool) {
        instructions::execute_instruction_rising(self, address_bus, rw_line);
    }

    pub fn tick_falling(&mut self, data_bus: &mut Bus) {
        instructions::execute_instruction_falling(self, data_bus);
    }

    pub fn increment_program_counter(&mut self) {
        self.program_counter = self.program_counter.wrapping_add(1);
    }

    pub fn reset(&mut self) {
        self.current_instruction = Instruction::Reset;
        self.reset_instruction_vars();
    }

    fn write_to_address(&mut self, value: u16, address_bus: &mut Bus, rw_line: &mut bool) {
        address_bus.set_combined(value as usize);
        *rw_line = false;
    }

    fn read_from_address(&mut self, value: u16, address_bus: &mut Bus, rw_line: &mut bool) {
        address_bus.set_combined(value as usize);
        *rw_line = true;
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
}
