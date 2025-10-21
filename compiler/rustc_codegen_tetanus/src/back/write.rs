use crate::TetanusCodegenBackend;
use rustc_codegen_ssa::CompiledModule;
// use rustc_codegen_ssa::ModuleKind;
use rustc_codegen_ssa::ModuleCodegen;
use rustc_codegen_ssa::back::write::{CodegenContext, ModuleConfig};
use rustc_session::config::OutputType;
use std::fs::File;
use std::io::Write;

pub fn codegen(
    cgcx: &CodegenContext<TetanusCodegenBackend>,
    module: ModuleCodegen<
        <TetanusCodegenBackend as rustc_codegen_ssa::traits::WriteBackendMethods>::Module,
    >,
    _config: &ModuleConfig,
) -> CompiledModule {
    let path = cgcx.output_filenames.temp_path_for_cgu(
        OutputType::Assembly,
        &module.name,
        cgcx.invocation_temp.as_deref(),
    );
    eprintln!("writing to {:?}", path);
    let mut file = File::create(path).unwrap();
    file.write_all(b"hello from tetanus\n").unwrap();

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
