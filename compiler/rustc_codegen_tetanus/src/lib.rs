//! The Rust compiler.
//!
//! # Note
//!
//! This API is completely unstable and subject to change.

// tidy-alphabetical-start

// tidy-alphabetical-end

use std::any::Any;
use std::path::PathBuf;
use std::string::String;
use std::fmt::Write;

use rustc_codegen_ssa::traits::*;
use rustc_codegen_ssa::ModuleCodegen;
use rustc_middle::ty::TyCtxt;
use rustc_ast::expand::allocator::AllocatorKind;
use rustc_session::Session;
use rustc_middle::dep_graph::{WorkProduct, WorkProductId};
use rustc_session::config::{OutputFilenames, PrintRequest, OptLevel};
use rustc_codegen_ssa::back::write::{TargetMachineFactoryFn, CodegenContext, ModuleConfig, FatLtoInput};
use rustc_codegen_ssa::back::lto::{SerializedModule, ThinModule};
use rustc_codegen_ssa::{CodegenResults, TargetConfig};
use rustc_errors::DiagCtxtHandle;
use rustc_data_structures::fx::FxIndexMap;
use rustc_span::Symbol;
use rustc_codegen_ssa::CompiledModule;

mod back;

rustc_fluent_macro::fluent_messages! { "../messages.ftl" }

#[derive(Clone)]
pub struct TetanusCodegenBackend();

impl CodegenBackend for TetanusCodegenBackend {
    fn locale_resource(&self) -> &'static str {
        crate::DEFAULT_LOCALE_RESOURCE
    }

    fn name(&self) -> &'static str {
        "tetanus"
    }

    fn init(&self, _sess: &Session) {}

    fn print(&self, req: &PrintRequest, out: &mut String, _sess: &Session) {
        writeln!(out, "havent done print() yet").unwrap();
        writeln!(out, "req: {:?}", req.kind).unwrap();
    }

    fn target_config(&self, _sess: &Session) -> TargetConfig {
        TargetConfig {
            target_features: vec![],
            unstable_target_features: vec![],
            has_reliable_f16: false,
            has_reliable_f16_math: false,
            has_reliable_f128: false,
            has_reliable_f128_math: false,
        }
    }

    fn join_codegen(
        &self,
        _ongoing_codegen: Box<dyn Any>,
        _sess: &Session,
        _outputs: &OutputFilenames,
    ) -> (CodegenResults, FxIndexMap<WorkProductId, WorkProduct>) {
        panic!("codegen not implemented");
    }

    
    fn codegen_crate<'tcx>(&self, tcx: TyCtxt<'tcx>) -> Box<dyn Any> {
        Box::new(rustc_codegen_ssa::base::codegen_crate(
            TetanusCodegenBackend(),
            tcx,
            "hiii :3".to_string(),
        ))
    }
}

impl ExtraBackendMethods for TetanusCodegenBackend {
    fn codegen_allocator<'tcx>(
        &self,
        _tcx: TyCtxt<'tcx>,
        _module_name: &str,
        _kind: AllocatorKind,
        _alloc_error_handler_kind: AllocatorKind,
    ) -> Self::Module {
        panic!("not implemented yet!");
    }

    /// This generates the codegen unit and returns it along with
    /// a `u64` giving an estimate of the unit's processing cost.
    fn compile_codegen_unit(
        &self,
        _tcx: TyCtxt<'_>,
        _cgu_name: Symbol,
    ) -> (ModuleCodegen<Self::Module>, u64) {
        panic!("not implemented yet!");
    }

    fn target_machine_factory(
        &self,
        _sess: &Session,
        _opt_level: OptLevel,
        _target_features: &[String],
    ) -> TargetMachineFactoryFn<Self> {
        panic!("not implemented yet!");
    }

    fn supports_parallel(&self) -> bool {
        false
    }
}

impl WriteBackendMethods for TetanusCodegenBackend {
    type Module = ModuleTetanus;
    type ModuleBuffer = back::lto::ModuleBuffer;
    type TargetMachine = String;
    type TargetMachineError = Box<dyn Any>;
    type ThinData = back::lto::ThinData;
    type ThinBuffer = back::lto::ThinBuffer;
    /// Performs fat LTO by merging all modules into a single one, running autodiff
    /// if necessary and running any further optimizations
    fn run_and_optimize_fat_lto(
        _cgcx: &CodegenContext<Self>,
        _exported_symbols_for_lto: &[String],
        _each_linked_rlib_for_lto: &[PathBuf],
        _modules: Vec<FatLtoInput<Self>>,
    ) -> ModuleCodegen<Self::Module> {
        panic!("not implemented yet!");
    }
    /// Performs thin LTO by performing necessary global analysis and returning two
    /// lists, one of the modules that need optimization and another for modules that
    /// can simply be copied over from the incr. comp. cache.
    fn run_thin_lto(
        _cgcx: &CodegenContext<Self>,
        _exported_symbols_for_lto: &[String],
        _each_linked_rlib_for_lto: &[PathBuf],
        _modules: Vec<(String, Self::ThinBuffer)>,
        _cached_modules: Vec<(SerializedModule<Self::ModuleBuffer>, WorkProduct)>,
    ) -> (Vec<ThinModule<Self>>, Vec<WorkProduct>) {
        panic!("not implemented yet!");
    }
    fn print_pass_timings(&self) {
        panic!("not implemented yet!");
    }
    fn print_statistics(&self) {
        panic!("not implemented yet!");
    }
    fn optimize(
        _cgcx: &CodegenContext<Self>,
        _dcx: DiagCtxtHandle<'_>,
        _module: &mut ModuleCodegen<Self::Module>,
        _config: &ModuleConfig,
    ) {
        panic!("not implemented yet!");
    }
    fn optimize_thin(
        _cgcx: &CodegenContext<Self>,
        _thin: ThinModule<Self>,
    ) -> ModuleCodegen<Self::Module> {
        panic!("not implemented yet!");
    }
    fn codegen(
        _cgcx: &CodegenContext<Self>,
        _module: ModuleCodegen<Self::Module>,
        _config: &ModuleConfig,
    ) -> CompiledModule {
        panic!("not implemented yet!");
    }
    fn prepare_thin(_module: ModuleCodegen<Self::Module>) -> (String, Self::ThinBuffer) {
        panic!("not implemented yet!");
    }
    fn serialize_module(_module: ModuleCodegen<Self::Module>) -> (String, Self::ModuleBuffer) {
        panic!("not implemented yet!");
    }
}

pub struct ModuleTetanus {}

/// This is the entrypoint for a hot plugged rustc_codegen_cranelift
#[unsafe(no_mangle)]
pub fn __rustc_codegen_backend() -> Box<dyn CodegenBackend> {
    Box::new(TetanusCodegenBackend {})
}
