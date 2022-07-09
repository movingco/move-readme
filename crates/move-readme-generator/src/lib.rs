//! Library for generating a readme from a Move package.
use std::path::Path;

use anyhow::*;
use module_id::ModuleIdData;
use move_core_types::{identifier::Identifier, language_storage::ModuleId};
use move_idl::{IDLBuilder, IDLPackage};
use rustdoc_to_markdown::process_docs;
use serde::{Deserialize, Serialize};
use tera::Tera;

#[derive(Serialize)]
pub struct ReadmeContext {
    pub title: String,
    pub body: String,
    pub address: String,
    pub idl: IDLPackage,
    pub license: Option<String>,
}

#[derive(Default, Serialize, Deserialize)]
pub struct ReadmeConfig {
    /// Title of the generated README. Defaults to the name of the package.
    pub title: Option<String>,
    /// Name of the main module's address. Defaults to the name of the package.
    pub address: Option<String>,
    /// Name of the main module. Defaults to the name of the package.
    pub main: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct PackageMeta {
    /// License.
    license: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct ManifestWithReadme {
    package: PackageMeta,
    /// Readme configuration.
    readme: Option<ReadmeConfig>,
}

/// Move readme.
struct ReadmeOptions {
    /// Title of the generated README. Defaults to the name of the package.
    title: String,
    /// The license if applicable.
    license: Option<String>,
    /// The main module.
    module_id: ModuleIdData,
}

/// Loads the readme associated with a manifest path.
fn load_from_manifest_path(idl: &IDLPackage, manifest_path: &Path) -> Result<ReadmeOptions> {
    let manifest_string: String = std::fs::read_to_string(manifest_path)?;
    let manifest_with_readme: ManifestWithReadme = toml::from_str(&manifest_string)?;
    let readme_cfg = manifest_with_readme.readme.unwrap_or_default();

    let title = readme_cfg.title.unwrap_or_else(|| idl.name.clone());
    let address_name = readme_cfg.address.unwrap_or_else(move || idl.name.clone());
    let address = *idl.aliases.get(&address_name).expect("address not found");

    let module_name = readme_cfg.main.unwrap_or_else(|| idl.name.clone());

    Ok(ReadmeOptions {
        title,
        license: manifest_with_readme.package.license,
        module_id: ModuleId::new(address, Identifier::new(module_name)?).into(),
    })
}

/// Generate the readme.
pub fn generate_readme(root_path: &Path) -> Result<String> {
    let idl = IDLBuilder::load(root_path)?.gen()?;
    let ReadmeOptions {
        title,
        license,
        module_id,
    } = load_from_manifest_path(&idl, &root_path.join("Move.toml"))?;

    let module = idl
        .modules
        .get(&module_id)
        .ok_or_else(|| anyhow!("module {} not found", &module_id))?;

    let markdown_preamble = process_docs(
        module
            .doc
            .clone()
            .unwrap_or_default()
            .lines()
            .collect::<Vec<_>>(),
        false,
    )
    .join("\n");

    let mut tera = Tera::default();
    tera.add_raw_template("default", include_str!("templates/default.tpl.md"))?;

    let context = ReadmeContext {
        title,
        body: markdown_preamble,
        address: module_id.address().to_hex_literal(),
        idl,
        license,
    };
    let result = tera.render("default", &tera::Context::from_serialize(context)?)?;

    Ok(result)
}
