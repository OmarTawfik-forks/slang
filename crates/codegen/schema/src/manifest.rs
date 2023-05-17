use std::{collections::BTreeSet, path::PathBuf};

use codegen_utils::{
    context::CodegenContext,
    errors::{CodegenErrors, CodegenResult},
};
use indexmap::IndexMap;
use schemars::JsonSchema;
use semver::Version;
use serde::{Deserialize, Serialize};

use super::{
    production::{Production, ProductionRef},
    yaml::parser::{parse_yaml_file, WithSourceLocation},
    SourceLocation,
};

#[derive(Deserialize, Serialize, JsonSchema, Clone, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Manifest {
    #[schemars(skip)]
    #[serde(default, skip)]
    pub source_location: Option<SourceLocation>,

    #[schemars(skip)]
    #[serde(default, skip)]
    pub productions: IndexMap<String, ProductionRef>,

    pub title: String,
    pub root_production: String,
    pub sections: Vec<ManifestSection>,
    #[schemars(with = "Vec<String>")]
    pub versions: Vec<Version>,
}

#[derive(Deserialize, Serialize, JsonSchema, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ManifestSection {
    pub title: String,
    pub path: String,
    pub topics: Vec<ManifestTopic>,
}

#[derive(Deserialize, Serialize, JsonSchema, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ManifestTopic {
    pub title: String,
    pub path: String,

    #[schemars(skip)]
    #[serde(default, skip)]
    pub productions: IndexMap<String, ProductionRef>,
}

impl WithSourceLocation for Manifest {
    fn set_source_location(&mut self, source_location: SourceLocation) {
        self.source_location = Some(source_location);
    }
}

impl Manifest {
    #[allow(dead_code)]
    pub fn load_from_dir(
        manifest_dir: PathBuf,
        codegen: &mut CodegenContext,
    ) -> CodegenResult<Self> {
        let manifest_path = manifest_dir.join("manifest.yml");

        let mut manifest: Manifest = parse_yaml_file(codegen, &manifest_path)?;
        let manifest_node = manifest.source_location.clone().unwrap().node;

        let mut errors = CodegenErrors::new();

        let sections_node = manifest_node.value_of_field("sections").unwrap();
        for (index, section) in manifest.sections.iter_mut().enumerate() {
            let section_path = manifest_dir.join(&section.path);
            let section_node = sections_node.value_at_index(index);
            if !section_path.exists() {
                errors.push(
                    &manifest_path,
                    &section_node.range(),
                    Errors::PathNotFound(section_path.clone()),
                );
                continue;
            }
            let topics_node = section_node.value_of_field("topics").unwrap();
            for (index, topic) in section.topics.iter_mut().enumerate() {
                let topic_path = section_path.join(&topic.path);
                let topic_node = topics_node.value_at_index(index);
                if !topic_path.exists() {
                    errors.push(
                        &manifest_path,
                        &topic_node.range(),
                        Errors::PathNotFound(topic_path.clone()),
                    );
                    continue;
                }

                let productions_path = topic_path.join("productions.yml");
                if !productions_path.exists() {
                    errors.push(
                        &manifest_path,
                        &topic_node.range(),
                        Errors::PathNotFound(productions_path.clone()),
                    );
                    continue;
                }

                match parse_yaml_file::<Vec<Production>>(codegen, &productions_path) {
                    Ok(productions) => {
                        for production in productions {
                            if manifest.productions.contains_key(production.name()) {
                                let source_location =
                                    production.source_location().as_ref().unwrap();
                                errors.push(
                                    &source_location.path,
                                    &source_location.node.range(),
                                    Errors::DuplicateProductionName(production.name().clone()),
                                );
                            } else {
                                let production_ref = ProductionRef::new(production);
                                topic
                                    .productions
                                    .insert(production_ref.name().clone(), production_ref.clone());
                                manifest
                                    .productions
                                    .insert(production_ref.name().clone(), production_ref);
                            }
                        }
                    }
                    Err(error) => {
                        errors.extend(error);
                    }
                }
            }
        }

        manifest.validate().err_or(manifest)
    }

    fn validate(&self) -> CodegenErrors {
        let mut errors = CodegenErrors::new();

        if !self.productions.contains_key(&self.root_production) {
            errors.push(
                &self.source_location.as_ref().unwrap().path,
                &self
                    .source_location
                    .as_ref()
                    .unwrap()
                    .node
                    .value_of_field("rootProduction")
                    .unwrap()
                    .range(),
                Errors::ProductionNotFound(self.root_production.clone()),
            );
        }

        errors
    }

    #[allow(dead_code)]
    pub fn collect_version_breaks(&self) -> BTreeSet<Version> {
        let mut version_breaks = BTreeSet::new();
        version_breaks.insert(self.versions.first().cloned().unwrap());

        for production in self.productions.values() {
            if let Some(versions) = production.versions() {
                version_breaks.extend(versions.into_iter().cloned());
            }
        }

        return version_breaks;
    }
}

#[derive(thiserror::Error, Debug)]
enum Errors {
    #[error("Path not found: {0}")]
    PathNotFound(PathBuf),
    #[error("Duplicate production: {0}")]
    DuplicateProductionName(String),
    #[error("Production not found: {0}")]
    ProductionNotFound(String),
}
