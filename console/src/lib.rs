mod bus;
mod cpu;
mod state;

pub use bus::Bus;
use cpu::CPU;
use state::State;

pub trait CartridgeHandler {
    fn tick(&mut self, address_bus: &mut Bus, data_bus: &mut Bus);
}

pub fn run_console_with_cartridge(cartridge: &mut dyn CartridgeHandler) {
    let mut state = State::new();

    let mut address_bus = Bus::new(13);
    let mut data_bus = Bus::new(8);
    let mut rw_line = false;
    let mut phi2_line = false;
    let mut rdy_line = false;

    let mut cpu = CPU::new();

    loop {
        cpu.rising_edge(&mut address_bus, &mut data_bus);
        cartridge.tick(&mut address_bus, &mut data_bus);
        cpu.falling_edge(&mut address_bus, &mut data_bus);
    }
}
