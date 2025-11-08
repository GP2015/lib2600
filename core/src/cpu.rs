mod instructions;

use crate::bus::Bus;
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
        }
    }

    pub fn tick_rising_edge(&mut self, address_bus: &mut Bus, data_bus: &mut Bus) {
        instructions::execute_instruction_rising_edge(self, address_bus, data_bus);
    }

    pub fn tick_falling_edge(&mut self, address_bus: &mut Bus, data_bus: &mut Bus) {
        instructions::execute_instruction_falling_edge(self, address_bus, data_bus);
    }

    pub fn reset(&mut self) {
        self.current_instruction = Instruction::Reset;
        self.reset_cycle_counters();
    }

    fn end_instruction(&mut self) {
        self.current_instruction = Instruction::Fetch;
        self.reset_cycle_counters();
    }

    fn reset_cycle_counters(&mut self) {
        self.instruction_cycle = 0;
        self.addressing_cycle = 0;
    }
}
