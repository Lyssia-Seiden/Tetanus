use crate::TetanusCodegenBackend;
use rustc_codegen_ssa::CompiledModule;
// use rustc_codegen_ssa::ModuleKind;
use rustc_codegen_ssa::ModuleCodegen;
use rustc_codegen_ssa::back::write::{CodegenContext, ModuleConfig};
use rustc_session::config::OutputType;
use std::fs::File;
use std::io::Write;
use super::elf_builder::ElfBuilder;

pub fn codegen(
    cgcx: &CodegenContext<TetanusCodegenBackend>,
    module: ModuleCodegen<
        <TetanusCodegenBackend as rustc_codegen_ssa::traits::WriteBackendMethods>::Module,
    >,
    _config: &ModuleConfig,
) -> CompiledModule {
    let path = cgcx.output_filenames.temp_path_for_cgu(
        OutputType::Object,
        &module.name,
        cgcx.invocation_temp.as_deref(),
    );
    eprintln!("writing to {:?}", path);
    let elf = ElfBuilder::new_with_header();
    let mut file = File::create(path).unwrap();
    file.write_all(&elf.bytes).unwrap();

    module.into_compiled_module(
        false,
        false,
        false,
        true,
        false,
        &cgcx.output_filenames,
        cgcx.invocation_temp.as_deref(),
    )
}
