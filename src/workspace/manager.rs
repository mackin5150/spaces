use crate::models::workspace_request::{Language, PackageManager, Stack};
use crate::workspace::cpp_llama_cpp::CppLlamaCppWorkspace;
use crate::workspace::go::GoWorkspace;
use crate::workspace::huggingface::HuggingFaceWorkspace;
use crate::workspace::lua::LuaWorkspace;
use crate::workspace::node_express::NodeExpressWorkspace;
use crate::workspace::node_hardhat::NodeHardhatWorkspace;
use crate::workspace::node_nextjs::NodeNextjsWorkspace;
use crate::workspace::node_sveltekit::NodeSveltekitWorkspace;
use crate::workspace::node_vite::NodeViteWorkspace;
use crate::workspace::python::PythonWorkspace;
use crate::workspace::python_django::PythonDjangoWorkspace;
use crate::workspace::python_fastapi::PythonFastapiWorkspace;
use crate::workspace::python_flask::PythonFlaskWorkspace;
use crate::workspace::react::NodeReactWorkspace;
use crate::workspace::traits::WorkspaceAdapter;

pub fn adapters() -> Vec<Box<dyn WorkspaceAdapter>> {
    vec![
        Box::new(PythonWorkspace::new()),
        Box::new(PythonFlaskWorkspace::new()),
        Box::new(PythonDjangoWorkspace::new()),
        Box::new(PythonFastapiWorkspace::new()),
        Box::new(HuggingFaceWorkspace::new()),
        Box::new(NodeExpressWorkspace::new()),
        Box::new(NodeHardhatWorkspace::new()),
        Box::new(NodeNextjsWorkspace::new()),
        Box::new(NodeReactWorkspace::new()),
        Box::new(NodeSveltekitWorkspace::new()),
        Box::new(NodeViteWorkspace::new()),
        Box::new(GoWorkspace::new()),
        Box::new(LuaWorkspace::new()),
        Box::new(CppLlamaCppWorkspace::new()),
    ]
}

pub fn get_adapter(language: &Language, stack: &Stack) -> Option<Box<dyn WorkspaceAdapter>> {
    adapters()
        .into_iter()
        .find(|adapter| &adapter.language() == language && &adapter.stack() == stack)
}

pub fn default_package_manager(language: &Language, stack: &Stack) -> PackageManager {
    get_adapter(language, stack)
        .map(|adapter| adapter.package_manager())
        .unwrap_or_else(|| match language {
            Language::Python => PackageManager::Uv,
            Language::Node => PackageManager::Npm,
            Language::Go => PackageManager::Go,
            Language::Lua => PackageManager::Make,
            Language::Cpp => PackageManager::Make,
        })
}
