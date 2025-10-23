
pub struct ElfBuilder {
    pub bytes: Vec<u8>,
}

impl ElfBuilder {
    pub fn new_with_header() -> Self {
        let bytes = include_bytes!("empty_elf.elf");
        Self { bytes: bytes.to_vec() }
    }
}