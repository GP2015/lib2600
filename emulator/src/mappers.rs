mod m2k;
mod m4k;

use crate::core::Cartridge;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MapperError {
    #[error("provided program is not compatible with {mapper_name} mapper")]
    InvalidProgram { mapper_name: String },
}

macro_rules! define_mappers {
    ($(($de_str:literal, $enum_name:ident, $map_mod:ident::$map_struct:ident)),* $(,)?) => {
        #[derive(Copy, Clone, Debug, serde::Deserialize, clap::ValueEnum)]
        pub enum MapperKind {
            $(
                #[value(name = $de_str)]
                #[serde(rename = $de_str)]
                $enum_name
            ),*
        }

        impl MapperKind {
            pub fn to_cartridge(
                self,
                program: Vec<u8>
            ) -> Result<Box<dyn Cartridge>, MapperError> {
                match self {
                    $(
                        MapperKind::$enum_name =>
                            Ok(Box::new($map_mod::$map_struct::new(program)?)),
                    )*
                }
            }
        }
    };
}

define_mappers! {
    ("2k", M2K, m2k::Mapper2K),
    ("4k", M4K, m4k::Mapper4K),
}
