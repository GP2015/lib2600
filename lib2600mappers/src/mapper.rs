mod m2k;
mod m4k;

use console::{self, Bus, CartridgeHandler};

macro_rules! define_mappers {
    (
        $(
            $variant:ident => $module:ident :: $struct_name:ident
        ),* $(,)?
    ) => {
        #[derive(clap::ValueEnum, Copy, Clone, Debug, serde::Serialize)]
        #[serde(rename_all = "lowercase")]
        pub enum MapperKind {
            $( $variant ),*
        }

        impl MapperKind {
            pub fn to_cartridge(self, program: Vec<u8>) -> anyhow::Result<Box<dyn CartridgeHandler>> {
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
    M2K => m2k::Mapper2K,
    M4K => m4k::Mapper4K,
}
