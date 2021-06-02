use std::env;


fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    let exclude = [".proto/README.md".to_owned(), ".proto/.git".to_owned()];

    let out_dir = format!("{}/protos", out_dir);

    let files: Vec<_> = walkdir::WalkDir::new(".proto")
        .into_iter()
        .filter_map(|p| {
            let dent = p.expect("Error happened when search protos");
            if !dent.file_type().is_file() {
                return None;
            }

            let filename = format!("{}", dent.path().display()).replace('\\', "/");

            if exclude.contains(&filename) {
                return None;
            }

            Some(filename)
        })
        .collect();

    protobuf_build::Builder::new()
        .includes(&[".proto".to_owned()])
        .files(&files)
        .out_dir(&out_dir)
        .generate();
}
