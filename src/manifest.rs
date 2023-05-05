use cargo_metadata::MetadataCommand;

/// Get GDPR Manifest Version from Cargo.toml
pub fn get_version() -> String {
    let metadata = MetadataCommand::new().exec().unwrap();
    let manifest_version = metadata
        .root_package()
        .unwrap()
        .metadata
        .get("manifest_version")
        .expect("manifest_version not found in Cargo.toml")
        .as_str()
        .expect("manifest_version is not a string");

    return manifest_version.to_owned();
}
