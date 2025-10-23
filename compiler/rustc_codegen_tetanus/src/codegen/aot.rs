// use rustc_codegen_ssa::ModuleCodegen;
use rustc_codegen_ssa::CompiledModule;
use rustc_middle::dep_graph::{WorkProduct, WorkProductId};
use rustc_codegen_ssa::CrateInfo;
use rustc_codegen_ssa::CodegenResults;
use rustc_data_structures::fx::FxIndexMap;
use rustc_session::Session;

#[allow(dead_code)]
pub(crate) struct ModuleCodegenResult {
    module_regular: CompiledModule,
    module_global_asm: Option<CompiledModule>,
    existing_work_product: Option<(WorkProductId, WorkProduct)>,
}

#[allow(dead_code)]
pub(crate) enum OngoingModuleCodegen {
    Sync(Result<ModuleCodegenResult, String>),
    // Async(JoinHandle<Result<ModuleCodegenResult, String>>),
}

#[allow(dead_code)]
pub(crate) struct OngoingCodegen {
    pub modules: Vec<OngoingModuleCodegen>,
    pub allocator_module: Option<CompiledModule>,
    pub crate_info: CrateInfo,
}

impl OngoingCodegen {
    pub(crate) fn join(
        self,
        _sess: &Session,
        // outputs: &OutputFilenames,
    ) -> (CodegenResults, FxIndexMap<WorkProductId, WorkProduct>) {
        (
            CodegenResults {
                modules: vec![],
                allocator_module: None,
                crate_info: self.crate_info,
            },
            FxIndexMap::default(),
        )
    }
}
