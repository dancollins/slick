use log::info;
use serde::ser::{SerializeSeq, SerializeStruct};
use std::fs::OpenOptions;
use std::path::Path;
use three_d_asset::geometry::TriMesh;

pub struct Mesh {
    pub trimesh: TriMesh,
}

impl serde::Serialize for Mesh {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut positions = serializer.serialize_seq(Some(self.trimesh.positions.len()))?;
        match self.trimesh.positions {
            F32 => {
                for e in self.trimesh.positions.to_f32() {
                    let mut state = serializer.serialize_struct("Vector3", 3)?;
                    state.serialize_field("x", &e.x)?;
                    state.serialize_field("y", &e.y)?;
                    state.serialize_field("z", &e.z)?;
                    state.end();
                }
            }
            F64 => {
                for e in self.trimesh.positions.to_f64() {
                    let mut state = serializer.serialize_struct("Vector3", 3)?;
                    state.serialize_field("x", &e.x)?;
                    state.serialize_field("y", &e.y)?;
                    state.serialize_field("z", &e.z)?;
                    state.end();
                }
            }
        };
        positions.end()
    }
}

fn load_stl(path: &Path) -> TriMesh {
    let mut file = OpenOptions::new().read(true).open(path).unwrap();
    let stl = stl_io::read_stl(&mut file).unwrap();

    return TriMesh {};
}

pub fn load(path: &Path) -> TriMesh {
    info!("Loading from path: {}", path.display());

    match path.extension() {
        None => panic!("File extension is missing."),
        Some(os_str) => match os_str.to_str() {
            Some("stl") => load_stl(path),
            _ => panic!("Unsupported extension."),
        },
    }
}
