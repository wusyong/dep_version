use cargo_metadata::MetadataCommand;

fn get_wry_version() -> String {
    let metadata = MetadataCommand::new()
        .manifest_path("./Cargo.toml")
        .exec()
        .unwrap();

    let wry = metadata.packages.iter().find(|x| x.name == "wry").unwrap();
    format!("{}", &wry.version)
}

fn main() {
    let version = get_wry_version();
    println!("{}", version);
}
