//! Here, [`CodegenRuntime::input_dir`] will have three kinds of files:
//!
//! 1) `input_dir/template.rs.jinja2`     -> a template file to render
//! 2) `input_dir/generated/template.rs`  -> the stub file rendered by this template in the source crate
//! 3) `input_dir/world.wit`              -> WIT files to generate Rust bindings from.
//! 4) `input_dir/generated/world.rs`     -> the stub Rust bindings generated from WIT files.
//! 5) `input_dir/other.rs`               -> other source files to be copied as-is
//!
//! For [`CodegenRuntime::render_stubs()`]:
//! First pass will render each template (#1) to its stub file (#2) under the same directory.
//! Second pass will render WIT files (#3) to their generated Rust bindings (#4) under the same directory.
//!
//! For [`CodegenRuntime::render_directory()`]:
//! First pass will render the template (#1) to `output_dir/generated/template.rs`, and skip the stub (#2).
//! Second pass will render WIT files (#3) to `output_dir/generated/world.rs`, and skip the stub (#4).
//! Third pass will copy the remaining sources (#5) to `output_dir/other.rs` as-is.

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use anyhow::Result;
use serde::Serialize;

use crate::codegen::tera::TeraWrapper;
use crate::codegen::wit::generate_rust_bindings;
use crate::codegen::{CodegenFileSystem, JINJA_GLOB, WIT_GLOB};
use crate::paths::{FileWalker, PathExtensions};

pub struct CodegenRuntime {
    input_dir: PathBuf,
    fs: CodegenFileSystem,
    tera: TeraWrapper,
}

impl CodegenRuntime {
    pub fn new(input_dir: impl Into<PathBuf>) -> Result<Self> {
        let input_dir = input_dir.into();
        let fs = CodegenFileSystem::new(&input_dir)?;
        let tera = TeraWrapper::new(&input_dir)?;

        Ok(Self {
            input_dir,
            fs,
            tera,
        })
    }

    pub fn render_stubs(&mut self, model: impl Serialize) -> Result<()> {
        let output_dir = self.input_dir.clone();

        let mut already_handled = HashSet::new();

        self.run_jinja_pass(&output_dir, &mut already_handled, model)?;
        self.run_wit_pass(&output_dir, &mut already_handled)?;

        Ok(())
    }

    pub fn render_directory(
        &mut self,
        model: impl Serialize,
        output_dir: impl AsRef<Path>,
    ) -> Result<()> {
        let output_dir = output_dir.as_ref();

        let mut already_handled = HashSet::new();

        self.run_jinja_pass(output_dir, &mut already_handled, model)?;
        self.run_wit_pass(output_dir, &mut already_handled)?;

        for source_path in FileWalker::from_directory(&self.input_dir).find_all()? {
            // Skip files that are already handled:
            if already_handled.contains(&source_path) {
                continue;
            }

            let output_path = output_dir.join(source_path.strip_prefix(&self.input_dir)?);

            // Preserve the source of otherwise-generated files:
            if source_path.generated_dir().is_ok() {
                self.fs.mark_generated_file(output_path)?;
            } else {
                self.fs.copy_file(source_path, output_path)?;
            }
        }

        Ok(())
    }

    fn run_jinja_pass(
        &mut self,
        output_dir: &Path,
        already_handled: &mut HashSet<PathBuf>,
        model: impl Serialize,
    ) -> Result<()> {
        let context = tera::Context::from_serialize(model)?;

        for template_path in FileWalker::from_directory(&self.input_dir).find([JINJA_GLOB])? {
            let stub_path = Self::get_stub_path(&template_path, "jinja2");

            let output_path = output_dir.join(stub_path.strip_prefix(&self.input_dir)?);

            let output = self.tera.render(&template_path, &context)?;

            self.fs.write_file(&output_path, output)?;

            already_handled.insert(template_path);
            already_handled.insert(stub_path);
        }

        Ok(())
    }

    fn run_wit_pass(
        &mut self,
        output_dir: &Path,
        already_handled: &mut HashSet<PathBuf>,
    ) -> Result<()> {
        for wit_path in FileWalker::from_directory(&self.input_dir).find([WIT_GLOB])? {
            let stub_path = Self::get_stub_path(&wit_path, "wit");

            let output_path = output_dir
                .join(stub_path.strip_prefix(&self.input_dir)?)
                .with_extension("rs");

            let output = generate_rust_bindings(&wit_path)?;

            self.fs.write_file(&output_path, output)?;

            already_handled.insert(wit_path);
            already_handled.insert(stub_path);
        }

        Ok(())
    }

    fn get_stub_path(template_path: &Path, extension: &str) -> PathBuf {
        assert_eq!(template_path.extension().unwrap(), extension);

        let without_extension = template_path.with_extension("");
        let base_name = without_extension.unwrap_name();

        template_path
            .unwrap_parent()
            .join("generated")
            .join(base_name)
    }
}
