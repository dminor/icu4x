use cargo_metadata::MetadataCommand;
use icu_locale::LanguageIdentifier;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct PackageMetadata {
    pub cldr_locales: Vec<LanguageIdentifier>,
    pub cldr_core_ref: String,
    pub cldr_dates_ref: String,
}

pub fn load() -> PackageMetadata {
    let metadata = MetadataCommand::new()
        .no_deps()
        .manifest_path(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml"))
        .exec()
        .unwrap();

    let target_directory = metadata.target_directory;

    let pkg = metadata
        .packages
        .iter()
        .filter(|x| x.name == "icu-testdata")
        .next()
        .unwrap();

    println!("{:?}", pkg);
    println!("{:?}", target_directory);

    let package_metadata: PackageMetadata = serde_json::from_value(
        pkg.metadata
            .as_object()
            .unwrap()
            .get("icu4x_testdata")
            .unwrap()
            .clone(),
    )
    .unwrap();
    return package_metadata;
}
