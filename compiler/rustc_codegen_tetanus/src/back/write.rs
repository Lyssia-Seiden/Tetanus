use crate::TetanusCodegenBackend;
use rustc_codegen_ssa::CompiledModule;
use rustc_codegen_ssa::ModuleKind;
use rustc_codegen_ssa::ModuleCodegen;
use rustc_codegen_ssa::back::write::{CodegenContext, ModuleConfig};
use std::{thread, time};

pub fn codegen(
    _cgcx: &CodegenContext<TetanusCodegenBackend>,
    _module: ModuleCodegen<
        <TetanusCodegenBackend as rustc_codegen_ssa::traits::WriteBackendMethods>::Module,
    >,
    _config: &ModuleConfig,
) -> CompiledModule {
    thread::sleep(time::Duration::from_millis(500));
    CompiledModule {
        name: "Test".to_string(),
        kind: ModuleKind::Regular,
        object: None,
        dwarf_object: None,
        bytecode: None,
        assembly: None,
        llvm_ir: None,
        links_from_incr_cache: vec![],
    }
}
