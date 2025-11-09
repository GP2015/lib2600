mod m2k;
mod m4k;

use crate::core::{Bus, Cartridge};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MapperError {
    #[error("provided program is not compatible with {mapper_name} mapper")]
    InvalidProgram { mapper_name: String },
}

macro_rules! define_mappers {
    (
        $(
            $string:literal => $variant:ident => $module:ident :: $struct_name:ident
        ),* $(,)?
    ) => {
        #[derive(Copy, Clone, Debug, serde::Deserialize, clap::ValueEnum)]
        pub enum MapperKind {
            $(
                #[value(name = $string)]
                #[serde(rename = $string)]
                $variant
            ),*
        }

        impl MapperKind {
            pub fn to_cartridge(self, program: Vec<u8>) -> Result<Box<dyn Cartridge>, MapperError> {
                match self {
                    $(
                        MapperKind::$variant => Ok(Box::new($module::$struct_name::new(program)?)),
                    )*
                }
            }
        }
    };
}

define_mappers! {
    "2k" => M2K => m2k::Mapper2K,
    "4k" => M4K => m4k::Mapper4K,
}
