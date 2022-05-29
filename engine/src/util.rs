use std::path::Path;

pub fn path_filename(path: &str) -> &str {
    Path::new(path).file_name().unwrap().to_str().unwrap()
}
