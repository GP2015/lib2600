#![warn(clippy::pedantic, clippy::nursery)]

mod cpu_instr;

use proc_macro::TokenStream;

macro_rules! pats {
    ($(($fn_name:ident, $lut:path)),+ $(,)?) => {$(
        #[proc_macro]
        pub fn $fn_name(input: TokenStream) -> TokenStream {
            cpu_instr::pat(input, &$lut)
        }
    )+};
}

pats!(
    (mnem_pat, cpu_instr::MNEM),
    (addr_mode_pat, cpu_instr::ADDR_MODE),
    (addr_mode_idx_pat, cpu_instr::ADDR_MODE_IDX),
);
