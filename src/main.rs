mod file_loading;
use file_loading::obj::{LoadingMethods, ObjFile};

const BASE_INP_DIR: &str = "/Users/schlomoh/Documents/GitHub/rust_remeshing/_inp";
const BASE_FILENAME: &str = "test";

fn main() {
    let file = ObjFile {
        file_path: BASE_INP_DIR.to_string(),
        file_name: BASE_FILENAME.to_string(),
    };
    let raw_data = file.load_raw_file();
    for (i, c) in raw_data.lines().enumerate() {
        println!("line: {}, text: {}", i, c)
    }
}
