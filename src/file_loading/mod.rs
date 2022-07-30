pub mod obj {
    use std::fs;
    use std::path::Path;
    pub struct File {
        pub file_name: String,
        pub file_path: String,
    }

    pub trait IOMethods {
        fn load_raw_file(&self) -> String;
    }

    impl IOMethods for File {
        fn load_raw_file(&self) -> String {
            let source = Path::new(&self.file_path).join(&self.file_name);
            return fs::read_to_string(source).expect("Failed loading file.");
        }
    }

    pub struct GeometryData {
        pub vertices: Vec<f32>,
        pub normals: Vec<f32>,
        pub texcoords: Vec<f32>,
        pub faces: Vec<Vec<u32>>,
    }

    pub struct RawObjData {
        pub raw_data: String,
    }

    pub trait ObjSerialization {
        fn new(raw_data: String) -> Self;
        fn parse_lines(&self) -> Vec<String>;
        fn parse_geometry(&self) -> GeometryData;
    }

    impl ObjSerialization for RawObjData {
        fn new(raw_data: String) -> Self {
            return RawObjData { raw_data: raw_data };
        }
        fn parse_lines(&self) -> Vec<String> {
            return self.raw_data.lines().map(|x| x.to_string()).collect();
        }

        fn parse_geometry(&self) -> GeometryData {
            let lines = self.parse_lines();
            let mut vertices = Vec::new();
            let mut normals = Vec::new();
            let mut texcoords = Vec::new();
            let mut faces = Vec::new();

            fn add_coord(vertices: &mut Vec<f32>, coords: &Vec<&str>) {
                for coord in coords {
                    let float = coord.replace(",", ".").parse::<f32>();
                    match float {
                        Ok(c) => vertices.push(c),
                        Err(_) => eprintln!("Failed to convert coord"),

                    }

                }
            }
            fn add_faces(faces: &mut Vec<Vec<u32>>, parts: &Vec<&str>) {
                let face = parts[1].split("/").collect::<Vec<&str>>();
                let mut face_data = Vec::new();
                for i in 1..face.len() {
                    face_data.push(face[i].parse::<u32>().unwrap());
                }
                faces.push(face_data);
            }

            for line in lines {
                let parts = line.split(" ").collect::<Vec<&str>>();
                match parts[0] {
                    "v" => {
                        add_coord(&mut vertices, &parts[1..].to_vec());
                    }
                    "vn" => {
                        add_coord(&mut normals, &parts[1..].to_vec());
                    }
                    "vt" => {
                        add_coord(&mut texcoords, &parts[1..].to_vec());
                    }
                    "f" => add_faces(&mut faces, &parts),
                    _ => {}
                }
            }
            return GeometryData {
                vertices,
                normals,
                texcoords,
                faces,
            };
        }
    }
}
