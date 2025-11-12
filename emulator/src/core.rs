mod cpu;
mod lines;

use cpu::{CPU, CPULines};
pub use lines::{Bus, ReadOrWrite};

/// A cartridge that the console can interact with.
pub trait Cartridge {
    /// Called whenever the console is reading/writing to the cartridge.
    fn tick(&mut self, address_bus: &mut Bus, data_bus: &mut Bus);

    /// Optional method. Called whenever the console is reset.
    fn power_reset(&mut self) {}
}

pub enum TVType {
    Color,
    Monochrome,
}

pub enum Difficulty {
    A,
    B,
}

/// The emulated Atari 2600 console.
///
/// This is only the core console.
/// It interfaces with external elements "through the ports" by using trait objects.
/// Implementations of these trait objects need to be provided separately.
///
/// Maybe talk about all the traits here.
///
/// The console is run on a cycle-by-cycle basis.
/// Calling the [Console::tick] method advances the console one cycle.
pub struct Console {
    cpu: CPU,
    cartridge: Option<Box<dyn Cartridge>>,
    address_bus: Bus,
    data_bus: Bus,
    rw_line: ReadOrWrite,
    game_select: bool,
    game_reset: bool,
    left_difficulty: Difficulty,
    right_difficulty: Difficulty,
    tv_type: TVType,
}

impl Console {
    /// Returns a new Atari 2600 console object.
    pub fn new() -> Self {
        Self {
            cpu: CPU::new(),
            cartridge: None,
            address_bus: Bus::new(13),
            data_bus: Bus::new(8),
            rw_line: ReadOrWrite::READ,
            game_select: false,
            game_reset: false,
            left_difficulty: Difficulty::A,
            right_difficulty: Difficulty::A,
            tv_type: TVType::Color,
        }
    }

    /// Reset the console.
    pub fn power_reset(&mut self) {
        self.address_bus.set_combined(0);
        self.data_bus.set_combined(0);
        self.cpu.reset();

        if let Some(cartridge) = self.cartridge.as_mut() {
            cartridge.power_reset();
        }
    }

    /// Advance the console forward one cycle.
    pub fn tick(&mut self) {
        let cpu_lines = CPULines::new(&mut self.address_bus, &mut self.data_bus, &mut self.rw_line);
        self.cpu.tick_rising(cpu_lines);

        if let Some(cartridge) = self.cartridge.as_mut() {
            cartridge.tick(&mut self.address_bus, &mut self.data_bus);
        }

        let cpu_lines = CPULines::new(&mut self.address_bus, &mut self.data_bus, &mut self.rw_line);
        self.cpu.tick_falling(cpu_lines);
    }

    /// Load a cartridge into the console.
    /// If another cartridge is already loaded, it will be replaced.
    ///
    /// `cartridge` must implement the [Cartridge] trait
    /// to be considered a valid cartridge.
    ///
    /// Note that this does not reset the console, it only loads the cartridge.
    /// Continuing to tick the console afterwards without calling the
    pub fn load_cartridge(&mut self, cartridge: Box<dyn Cartridge>) {
        self.cartridge = Some(cartridge);
    }

    /// Unload the cartridge from the console.
    /// If no cartridge is loaded, this method has no effect.
    pub fn unload_cartridge(&mut self) {
        self.cartridge = None;
    }

    pub fn set_game_select(&mut self, state: bool) {
        self.game_select = state;
    }

    pub fn set_game_reset(&mut self, state: bool) {
        self.game_reset = state;
    }

    pub fn set_left_difficulty(&mut self, difficulty: Difficulty) {
        self.left_difficulty = difficulty;
    }

    pub fn set_right_difficulty(&mut self, difficulty: Difficulty) {
        self.right_difficulty = difficulty;
    }

    pub fn set_tv_type(&mut self, tv_type: TVType) {
        self.tv_type = tv_type;
    }
}
