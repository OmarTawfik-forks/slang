mod ast;
mod bindings;
mod kinds;
mod parser;

pub mod ir;

use std::collections::BTreeSet;
use std::path::Path;
use std::rc::Rc;

use anyhow::Result;
use codegen_language_definition::model::Language;
use infra_utils::cargo::CargoWorkspace;
use infra_utils::codegen::{CodegenFileSystem, CodegenRuntime};
use semver::Version;
use serde::Serialize;

use crate::ast::AstModel;
use crate::bindings::BindingsModel;
use crate::kinds::KindsModel;
use crate::parser::ParserModel;

pub struct RuntimeGenerator;

impl RuntimeGenerator {
    pub fn generate_product(
        language: &Rc<Language>,
        input_dir: &Path,
        output_dir: &Path,
    ) -> Result<CodegenFileSystem> {
        let model = ModelWrapper {
            rendering_in_stubs: false,
            model: RuntimeModel::from_language(language)?,
        };

        let runtime = CodegenRuntime::new(input_dir)?;

        runtime.render_product(model, output_dir)
    }

    pub fn generate_stubs(source_dir: &Path) -> Result<()> {
        let model = ModelWrapper {
            rendering_in_stubs: true,
            model: RuntimeModel::default(),
        };

        let mut runtime = CodegenRuntime::new(source_dir)?;

        runtime.render_stubs(&model)
    }
}

/// A utility wrapper to make it easier to write conditional code in templates
/// by checking `rendering_in_stubs`. Additionally, it makes sure that all
/// model properties are prefixed with `model.` to make it easier to read/refactor.
#[derive(Serialize)]
struct ModelWrapper {
    rendering_in_stubs: bool,
    model: RuntimeModel,
}

#[derive(Serialize)]
struct RuntimeModel {
    slang_version: Version,
    language_name: String,
    all_language_versions: BTreeSet<Version>,
    breaking_language_versions: BTreeSet<Version>,

    ast: AstModel,
    bindings: BindingsModel,
    kinds: KindsModel,
    parser: ParserModel,
}

impl RuntimeModel {
    fn from_language(language: &Rc<Language>) -> Result<Self> {
        Ok(Self {
            slang_version: CargoWorkspace::local_version()?,
            language_name: language.name.to_string(),
            all_language_versions: language.versions.iter().cloned().collect(),
            breaking_language_versions: language.collect_breaking_versions(),

            ast: AstModel::from_language(language),
            bindings: BindingsModel::from_language(language)?,
            parser: ParserModel::from_language(language),
            kinds: KindsModel::from_language(language),
        })
    }
}

impl Default for RuntimeModel {
    fn default() -> Self {
        Self {
            slang_version: Version::new(0, 0, 0),
            language_name: "CodegenRuntime".to_string(),
            all_language_versions: BTreeSet::from([Version::new(0, 0, 0)]),
            breaking_language_versions: BTreeSet::from([Version::new(0, 0, 0)]),

            ast: AstModel::default(),
            bindings: BindingsModel::default(),
            kinds: KindsModel::default(),
            parser: ParserModel::default(),
        }
    }
}
