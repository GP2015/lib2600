mod config;

use anyhow::{Result, anyhow};
use clap::Parser;
use lib2600::core::Console;
use lib2600::mappers::MapperKind;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    mapper_kind: MapperKind,
    program_path: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let Ok(program) = std::fs::read(&args.program_path) else {
        return Err(anyhow!(
            "Could not find valid program at {}.",
            args.program_path
        ));
    };

    let mut console = Console::new();

    let cartridge = args.mapper_kind.to_cartridge(program)?;
    console.load_cartridge(cartridge);

    loop {
        console.tick();
    }

    Ok(())
}
