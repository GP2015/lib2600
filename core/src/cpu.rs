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
    current_instruction: Option<Instruction>,
    current_addressing_mode: Option<AddressingMode>,
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
            current_instruction: None,
            current_addressing_mode: None,
            instruction_cycle: 0,
            addressing_cycle: 0,
        }
    }

    pub fn tick_rising_edge(&mut self, address_bus: &mut Bus, data_bus: &mut Bus) {
        let Some(instruction) = self.current_instruction.as_mut() else {
            address_bus.set_combined(self.program_counter as usize);
            return;
        };
    }

    pub fn tick_falling_edge(&mut self, address_bus: &mut Bus, data_bus: &mut Bus) {
        if self.current_instruction.is_none() {
            let opcode = data_bus.get_combined() as u8;
            let (instruction, addressing_mode) = instructions::fetch_instruction(opcode);
            self.current_instruction = Some(instruction);
            self.current_addressing_mode = Some(addressing_mode);
        }

        let instruction = self
            .current_instruction
            .as_mut()
            .expect("Current instruction should not be None here.");
    }

    pub fn reset(&mut self) {
        instructions::reset(self);
    }

    // Jobs are scheduled in a "first in, first out" manner.
    pub fn schedule_jobs(&mut self, mut jobs: Vec<Job>) {
        jobs.reverse();
        self.job_stack.append(&mut jobs);
    }

    pub fn get_program_counter(&self) -> u16 {
        self.program_counter
    }
}
