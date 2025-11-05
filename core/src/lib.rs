mod bus;
mod cpu;

pub use bus::Bus;
use cpu::CPU;

pub trait Cartridge {
    fn tick(&mut self, address_bus: &mut Bus, data_bus: &mut Bus);
}

pub struct Console {
    address_bus: Bus,
    data_bus: Bus,
    cpu: CPU,
    cartridge: Option<Box<dyn Cartridge>>,
}

impl Console {
    pub fn new() -> Self {
        Self {
            address_bus: Bus::new(13),
            data_bus: Bus::new(8),
            cpu: CPU::new(),
            cartridge: None,
        }
    }

    pub fn tick(&mut self) {
        self.cpu
            .tick_rising_edge(&mut self.address_bus, &mut self.data_bus);

        if let Some(cartridge) = self.cartridge.as_mut() {
            cartridge.tick(&mut self.address_bus, &mut self.data_bus);
        }

        self.cpu
            .tick_falling_edge(&mut self.address_bus, &mut self.data_bus);
    }

    pub fn load_cartridge(&mut self, cartridge: Box<dyn Cartridge>) {
        self.cartridge = Some(cartridge);
    }

    pub fn unload_cartridge(&mut self) {
        self.cartridge = None;
    }
}
