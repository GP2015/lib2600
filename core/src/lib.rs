mod bus;
mod cpu;

pub use bus::Bus;
use cpu::CPU;

pub trait CartridgeHandler {
    fn tick(&mut self, address_bus: &mut Bus, data_bus: &mut Bus);
}

pub fn run_console(cartridge: &mut dyn CartridgeHandler) {
    let mut address_bus = Bus::new(13);
    let mut data_bus = Bus::new(8);
    let mut rw_line = false;
    let mut phi2_line = false;
    let mut rdy_line = false;

    let mut cpu = CPU::new();

    loop {
        cpu.tick_rising_edge(&mut address_bus, &mut data_bus);
        cartridge.tick(&mut address_bus, &mut data_bus);
        cpu.tick_falling_edge(&mut address_bus, &mut data_bus);
    }
}
