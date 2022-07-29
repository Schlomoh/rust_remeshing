pub mod obj {
    use std::fs;
    use std::path::Path;
    pub struct ObjFile {
        pub file_name: String,
        pub file_path: String,
    }

    pub trait LoadingMethods {
        fn load_raw_file(&self) -> String;
    }

    impl LoadingMethods for ObjFile {
        fn load_raw_file(&self) -> String {
            let source = Path::new(&self.file_path).join(&self.file_name);
            return fs::read_to_string(source).expect("Failed loading file.");
        }
    }
}
