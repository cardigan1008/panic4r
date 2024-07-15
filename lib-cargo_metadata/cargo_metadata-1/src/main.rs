use cargo_metadata::MetadataCommand;

fn main() {
    MetadataCommand::new()
        .exec()
        .unwrap()
        .root_package()
        .expect("root package exists");
    MetadataCommand::new()
        .no_deps()
        .exec()
        .unwrap()
        .root_package()
        .expect("root package exists w/ no_deps called");
}