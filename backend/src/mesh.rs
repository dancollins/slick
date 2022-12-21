use log::info;
use serde::{Serialize, Serializer};
use std::fs::OpenOptions;
use std::io::{Error, ErrorKind};
use std::path::Path;
use three_d_asset::geometry::{Indices, Positions, TriMesh};
use three_d_asset::prelude::{Color, Vec2, Vec3, Vec4, Vector3};

#[derive(Serialize)]
#[serde(remote = "Positions")]
enum PositionsDef {
    F32(Vec<Vec3>),
    F64(Vec<Vector3<f64>>),
}

#[derive(Serialize)]
#[serde(remote = "Indices")]
enum IndicesDef {
    None,
    U8(Vec<u8>),
    U16(Vec<u16>),
    U32(Vec<u32>),
}

#[derive(Serialize)]
struct ColorInternal {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

fn to_internal_color(color: &Color) -> ColorInternal {
    ColorInternal {
        r: color.r,
        g: color.g,
        b: color.b,
        a: color.a,
    }
}

fn serialise_option_color<S: Serializer>(
    opt_vec: &Option<Vec<Color>>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    match opt_vec {
        Some(vec) => {
            let vec2: Vec<ColorInternal> = vec.iter().map(to_internal_color).collect();
            vec2.serialize(serializer)
        }
        None => serializer.serialize_none(),
    }
}

#[derive(Serialize)]
#[serde(remote = "TriMesh")]
struct TriMeshDef {
    pub name: String,
    pub material_name: Option<String>,
    #[serde(with = "PositionsDef")]
    pub positions: Positions,
    #[serde(with = "IndicesDef")]
    pub indices: Indices,
    pub normals: Option<Vec<Vec3>>,
    pub tangents: Option<Vec<Vec4>>,
    pub uvs: Option<Vec<Vec2>>,
    #[serde(serialize_with = "serialise_option_color")]
    pub colors: Option<Vec<Color>>,
}

#[derive(Serialize)]
pub struct Mesh {
    #[serde(with = "TriMeshDef")]
    pub mesh: TriMesh,
}

fn load_stl(filename: &Path) -> Result<Mesh, Error> {
    let mut file = OpenOptions::new().read(true).open(filename)?;
    let stl = stl_io::read_stl(&mut file)?;

    let mut positions: Vec<Vec3> = Vec::new();
    for pos in stl.vertices {
        positions.push(Vec3::new(pos[0], pos[1], pos[2]));
    }

    let mut indices: Vec<u32> = Vec::new();
    let mut normals: Vec<Vec3> = Vec::new();
    for triangle in stl.faces {
        indices.push(u32::try_from(triangle.vertices[0]).unwrap());
        indices.push(u32::try_from(triangle.vertices[1]).unwrap());
        indices.push(u32::try_from(triangle.vertices[2]).unwrap());
        normals.push(Vec3::new(
            triangle.normal[0],
            triangle.normal[1],
            triangle.normal[2],
        ));
    }

    let mesh = Mesh {
        mesh: TriMesh {
            name: "stlmesh".to_string(),
            positions: Positions::F32(positions),
            indices: Indices::U32(indices),
            normals: Some(normals),
            ..Default::default()
        },
    };

    Ok(mesh)
}

pub fn load(filename: &Path) -> Result<Mesh, Error> {
    info!("Loading from path: {}", filename.display());

    match filename.extension() {
        None => Err(Error::new(
            ErrorKind::InvalidInput,
            "No file extension provided!",
        )),
        Some(os_str) => match os_str.to_str() {
            Some("stl") => load_stl(filename),
            _ => Err(Error::new(
                ErrorKind::Unsupported,
                format!("Unsupported extension for path {}", filename.display()),
            )),
        },
    }
}
