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
    job_stack: Vec<Job>,
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
            job_stack: Vec::new(),
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

    pub fn rising_edge(&mut self, address_bus: &mut Bus, data_bus: &mut Bus) {
        if self.job_stack.is_empty() {
            instructions::get_new_jobs(self);
        }

        if let Job::RequestRead(addr) = self
            .job_stack
            .last()
            .expect("Job queue shouldn't empty, so last() won't return None.")
        {
            address_bus.set_combined(*addr);
            self.job_stack.pop();
        }
    }

    pub fn falling_edge(&mut self, address_bus: &mut Bus, data_bus: &mut Bus) {
        loop {
            let job = self
                .job_stack
                .pop()
                .expect("Job queue shouldn't be empty, so pop() won't return None.");

            match job {
                Job::EndCycle => break,
                Job::RequestRead(_) => panic!("There should not be a read request here."),
                Job::FnInternal(fn_ptr) => fn_ptr(self),
                Job::FnWithDataBus(fn_ptr) => fn_ptr(self, data_bus),
            }
        }
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
