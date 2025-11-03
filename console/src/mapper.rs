mod m2k;
mod m4k;

use crate::bus::Bus;
use anyhow::Result;

pub trait UseAsMapper {
    fn new(program: Vec<u8>) -> Result<Self>
    where
        Self: Sized;

    fn tick(&mut self, address_bus: &mut Bus, data_bus: &mut Bus);
}

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
            pub fn to_mapper(self, program: Vec<u8>) -> anyhow::Result<Box<dyn UseAsMapper>> {
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
