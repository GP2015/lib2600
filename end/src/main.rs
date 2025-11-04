mod config;

use anyhow::{Result, anyhow};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    mapper_kind: lib2600mappers::MapperKind,
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

    let mut cartridge = args.mapper_kind.to_cartridge(program)?;
    lib2600core::run_console(&mut *cartridge);

    Ok(())
}
