mod mapper;

use anyhow::{Result, anyhow};
use clap::Parser;
use mapper::MapperKind;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    program_path: String,
    mapper: MapperKind,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let Ok(program) = std::fs::read(&args.program_path) else {
        return Err(anyhow!(
            "Could not find valid program at {}.",
            args.program_path
        ));
    };

    let mut cartridge = MapperKind::to_cartridge(args.mapper, program)?;

    console::run_console_with_cartridge(&mut *cartridge);

    Ok(())
}
