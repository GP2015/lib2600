mod instructions;

use crate::bus::Bus;

const PC_RESET_ADDR_LOW_BYTE: usize = 0xFFFC;
const PC_RESET_ADDR_HIGH_BYTE: usize = 0xFFFD;

const PROGRAM_COUNTER_RESET_VALUE: u16 = 0x0000;
const ACCUMULATOR_RESET_VALUE: u8 = 0x00;
const X_REGISTER_RESET_VALUE: u8 = 0x00;
const Y_REGISTER_RESET_VALUE: u8 = 0x00;
const STATUS_REGISTER_RESET_VALUE: u8 = 0x00;
const STACK_POINTER_RESET_VALUE: u8 = 0xFD;



type BasicOperationFnPtr = fn(cpu: &mut CPU, address_bus: &mut Bus, data_bus: &mut Bus);

enum BasicOperation{
    Nothing(BasicOperationFnPtr),
    ReadMemory(BasicOperationFnPtr, usize),
    WriteMemory(BasicOperationFnPtr, usize),
}

pub struct CPU {
    operations_to_do: Vec<Operation>,
    program_counter: u16,
    accumulator: u8,
    x_register: u8,
    y_register: u8,
    status_register: u8,
    stack_pointer: u8,
}

impl CPU {
    pub fn new() -> Self {
        let mut operations_to_do: vec![];

        operations_to_do[6] = ;

        Self {
            operations_to_do: [do_nothing; MAX_OPERATIONS_TO_DO],
            program_counter: PROGRAM_COUNTER_RESET_VALUE,
            accumulator: ACCUMULATOR_RESET_VALUE,
            x_register: X_REGISTER_RESET_VALUE,
            y_register: Y_REGISTER_RESET_VALUE,
            status_register: STATUS_REGISTER_RESET_VALUE,
            stack_pointer: STACK_POINTER_RESET_VALUE,
        }
    }

    pub fn tick(&mut self, address_bus: &mut Bus, data_bus: &mut Bus) {}

    fn schedule_operations(&mut self, mut operations: Vec<OperationFnPtr>) {
        self.operations_to_do_ptr = operations.len();
        operations.reverse();
        self.operations_to_do[0..operations.len()].copy_from_slice(&operations);
    }
}

fn do_nothing(_: &mut CPU, _: &mut Bus, _: &mut Bus) {
    return;
}
