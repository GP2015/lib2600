mod instructions;

use crate::bus::Bus;
use instructions::Job;

const PROGRAM_COUNTER_RESET_VALUE: u16 = 0x0000;
const ACCUMULATOR_RESET_VALUE: u8 = 0x00;
const X_REGISTER_RESET_VALUE: u8 = 0x00;
const Y_REGISTER_RESET_VALUE: u8 = 0x00;
const STATUS_REGISTER_RESET_VALUE: u8 = 0x00;
const STACK_POINTER_RESET_VALUE: u8 = 0xfd;

pub struct CPU {
    job_queue: Vec<Job>,
    program_counter: u16,
    accumulator: u8,
    x_register: u8,
    y_register: u8,
    status_register: u8,
    stack_pointer: u8,
}

impl CPU {
    pub fn new() -> Self {
        let mut cpu = Self {
            job_queue: Vec::new(),
            program_counter: PROGRAM_COUNTER_RESET_VALUE,
            accumulator: ACCUMULATOR_RESET_VALUE,
            x_register: X_REGISTER_RESET_VALUE,
            y_register: Y_REGISTER_RESET_VALUE,
            status_register: STATUS_REGISTER_RESET_VALUE,
            stack_pointer: STACK_POINTER_RESET_VALUE,
        };

        instructions::reset(&mut cpu);

        return cpu;
    }

    pub fn tick(&mut self, address_bus: &mut Bus, data_bus: &mut Bus) {
        if self.job_queue.is_empty() {
            instructions::get_new_jobs(self);
        }

        while !self.job_queue.is_empty() {
            let job = self
                .job_queue
                .pop()
                .expect("Job queue isn't empty, therefore pop() can't return None.");

            match job {
                Job::EndCycle => break,
                Job::ReadFromAddress(addr) => _ = address_bus.set_combined(addr),
                Job::FnInternal(fn_ptr) => fn_ptr(self),
                Job::FnWithAddressBus(fn_ptr) => fn_ptr(self, address_bus),
                Job::FnWithDataBus(fn_ptr) => fn_ptr(self, data_bus),
                Job::FnWithAddressAndDataBus(fn_ptr) => fn_ptr(self, address_bus, data_bus),
            }
        }
    }

    fn schedule_jobs(&mut self, mut operations: Vec<Job>) {
        operations.reverse();
        self.job_queue.append(&mut operations);
    }

    fn increment_program_counter(&mut self) {
        self.program_counter = self.program_counter.wrapping_add(1);
    }
}
