mod file_loading;
use file_loading::obj::{IOMethods, ObjSerialization, File, RawObjData};

const BASE_INP_DIR: &str = "/Users/schlomoh/Documents/GitHub/rust_remeshing/_inp";
const BASE_FILENAME: &str = "test.obj";

fn main() {
    let file = File {
        file_path: BASE_INP_DIR.to_string(),
        file_name: BASE_FILENAME.to_string(),
    };

    let raw_data = file.load_raw_file();
    println!("{}", raw_data);
    let geometry = RawObjData::new(raw_data).parse_geometry();
    println!("{:?}", geometry.vertices)
}
