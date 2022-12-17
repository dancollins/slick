use log::debug;
use std::fs::OpenOptions;
use std::io;
use std::path::Path;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Vector {
    x: f32,
    y: f32,
    z: f32,
}

pub type Vertex = Vector;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct IndexedTriangle {
    indicies: [usize; 3],
    normal: Vector,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Mesh {
    vertices: Vec<Vertex>,
    triangles: Vec<IndexedTriangle>,
}

fn load_stl(path: &Path) -> Mesh {
    debug!("Opening STL from: {}", path.to_str().unwrap());
    let mut file = OpenOptions::new().read(true).open(path).unwrap();
    let stl = stl_io::read_stl(&mut file).unwrap();

    /* Pull stl_io internal representation into our Mesh struct */
    let mut vertices = Vec::<Vertex>::new();
    for vertex in stl.vertices {
        vertices.push(Vertex {
            x: vertex[0],
            y: vertex[1],
            z: vertex[2],
        });
    }

    let mut triangles = Vec::<IndexedTriangle>::new();
    for triangle in stl.faces {
        triangles.push(IndexedTriangle {
            indicies: triangle.vertices,
            normal: Vector {
                x: triangle.normal[0],
                y: triangle.normal[1],
                z: triangle.normal[2],
            },
        });
    }

    debug!(
        "Loaded {} verticies and {} triangles.",
        vertices.iter().count(),
        triangles.iter().count()
    );

    Mesh {
        vertices,
        triangles,
    }
}

pub fn load(path: &Path) -> Mesh {
    match path.extension() {
        None => panic!("File extension is missing."),
        Some(os_str) => match os_str.to_str() {
            Some("stl") => load_stl(path),
            _ => panic!("Unsupported extension."),
        },
    }
}
