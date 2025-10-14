use rustc_codegen_ssa::traits::ModuleBufferMethods;
use rustc_codegen_ssa::traits::ThinBufferMethods;

pub struct ModuleBuffer();

unsafe impl Send for ModuleBuffer {}
unsafe impl Sync for ModuleBuffer {}

impl ModuleBuffer {
    // pub(crate) fn new() -> ModuleBuffer {
    //     Self{}
    // }
}

impl ModuleBufferMethods for ModuleBuffer {
    fn data(&self) -> &[u8] {
        &[]
    }
}

impl Drop for ModuleBuffer {
    fn drop(&mut self) {}
}


pub struct ThinBuffer();

impl ThinBufferMethods for ThinBuffer {
    fn data(&self) -> &[u8] {
        &[]
    }
}

pub struct ThinData();