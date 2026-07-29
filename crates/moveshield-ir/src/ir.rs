//! Move Intermediate Representation (MoveIR) for engineering analysis.

use moveshield_core::models::{Location, Project};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Semantic representation of a project inside MoveIR.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveProject {
    /// Name of the project.
    pub name: String,
    /// Modules in the project, indexed by qualified name ("address::module_name").
    pub modules: HashMap<String, MoveModule>,
}

/// Semantic representation of a module in MoveIR.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveModule {
    /// Name of the module.
    pub name: String,
    /// Address of the module.
    pub address: String,
    /// Functions defined in this module.
    pub functions: HashMap<String, MoveFunction>,
}

/// Semantic representation of a function in MoveIR.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveFunction {
    /// Name of the function.
    pub name: String,
    /// Visibility of the function.
    pub visibility: String,
    /// Source location.
    pub location: Location,
    /// Cyclomatic complexity score.
    pub complexity: usize,
    /// Function calls made by this function.
    pub calls: Vec<String>,
}

/// The main representation of MoveIR.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveIR {
    /// The processed MoveProject.
    pub project: MoveProject,
}

impl MoveIR {
    /// Constructs MoveIR semantic model from the Core Data Model (CDM) Project.
    pub fn from_project(project: &Project) -> Self {
        let mut modules = HashMap::new();

        for package in &project.packages {
            for module in &package.modules {
                let module_key = format!("{}::{}", module.address, module.name);

                let mut functions = HashMap::new();
                for function in &module.functions {
                    let move_fn = MoveFunction {
                        name: function.name.clone(),
                        visibility: function.visibility.clone(),
                        location: function.location.clone(),
                        complexity: function.complexity,
                        calls: function.calls.clone(),
                    };
                    functions.insert(function.name.clone(), move_fn);
                }

                let move_mod = MoveModule {
                    name: module.name.clone(),
                    address: module.address.clone(),
                    functions,
                };

                modules.insert(module_key, move_mod);
            }
        }

        let move_project = MoveProject {
            name: project.name.clone(),
            modules,
        };

        Self { project: move_project }
    }
}
