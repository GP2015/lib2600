use ahash::AHashMap;
use serde::Deserialize;
use std::sync::LazyLock;

#[derive(Debug, Deserialize)]
struct InstrStructVariant {
    name: String,
    pattern: String,
}

#[derive(Debug, Deserialize)]
struct InstructionConfig {
    pub mnemonic: Vec<InstrStructVariant>,
    // pub addressing_mode: Vec<InstrStructVariant>,
    // pub addressing_mode_index: Vec<InstrStructVariant>,
}

static INSTR_CFG: LazyLock<InstructionConfig> = LazyLock::new(|| {
    let file_str = include_str!("data.toml");
    toml::from_str(file_str).unwrap()
});

pub static INSTR_MNEMONICS: LazyLock<AHashMap<String, String>> = LazyLock::new(|| {
    let mut map = AHashMap::new();
    for var in &INSTR_CFG.mnemonic {
        let res = map.insert(var.name.clone(), var.pattern.clone());
        assert!(res.is_none());
    }
    map
});
