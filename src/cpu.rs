mod instructions;

use crate::bus::Bus;
use instructions::CycleFunction;

const PC_RESET_ADDR_LOW_BYTE: usize = 0xfffc;
const PC_RESET_ADDR_HIGH_BYTE: usize = 0xfffd;

const PROGRAM_COUNTER_RESET_VALUE: u16 = 0x0000;
const ACCUMULATOR_RESET_VALUE: u8 = 0x00;
const X_REGISTER_RESET_VALUE: u8 = 0x00;
const Y_REGISTER_RESET_VALUE: u8 = 0x00;
const STATUS_REGISTER_RESET_VALUE: u8 = 0x00;
const STACK_POINTER_RESET_VALUE: u8 = 0xfd;

pub struct CPU {
    operations_to_do: Vec<CycleFunction>,
    program_counter: u16,
    accumulator: u8,
    x_register: u8,
    y_register: u8,
    status_register: u8,
    stack_pointer: u8,
}

impl CPU {
    pub fn new() -> Self {
        let mut operations_to_do = Vec::with_capacity(8);
        operations_to_do[0..6].copy_from_slice(&[instructions::do_nothing; 6]);
        operations_to_do.push(BasicOperation::Read(PC_RESET_ADDR_LOW_BYTE));
        operations_to_do.push(BasicOperation::Read(PC_RESET_ADDR_HIGH_BYTE));

        Self {
            operations_to_do,
            program_counter: PROGRAM_COUNTER_RESET_VALUE,
            accumulator: ACCUMULATOR_RESET_VALUE,
            x_register: X_REGISTER_RESET_VALUE,
            y_register: Y_REGISTER_RESET_VALUE,
            status_register: STATUS_REGISTER_RESET_VALUE,
            stack_pointer: STACK_POINTER_RESET_VALUE,
        }
    }

    pub fn tick(&mut self, address_bus: &mut Bus, data_bus: &mut Bus) {
        if self.operations_to_do.is_empty() {
            address_bus.set_combined(self.program_counter as usize);
            self.increment_program_counter();
        }

        let (instruction, addressing_mode) = instructions::get_instruction();
    }

    fn schedule_operations(&mut self, mut operations: Vec<BasicOperation>) {
        operations.reverse();
        self.operations_to_do.append(&mut operations);
    }

    fn increment_program_counter(&mut self) {
        self.program_counter.wrapping_add(1);
    }
}
