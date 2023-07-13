use serde::Serialize;
use serde_json::{Result, Value};
use std::fs::OpenOptions;
use std::path::PathBuf;
use stl_io::{read_stl, IndexedMesh};
use std::env::current_dir;

pub fn process(path: &PathBuf) -> Option<View> {
    if let Some(file_name) = path.as_os_str().to_str() {
        let mut file = OpenOptions::new().read(true).open(file_name).unwrap();
        // TODO need to handle this
        let file_res = read_stl(&mut file);
        match file_res {
            Ok(mesh) => {
                //println!("{:?}", mesh);
                let mut bb = bounding_box(mesh);
                let doc_name = path.file_name().unwrap().to_str().unwrap();
                let mut view = View::new();
                view.file = doc_name.to_string();
                view.centroid = bb.centroid();
                println!("{:#?}", view);
                // Copy the file into place
                let mut dest = PathBuf::new();
                dest.push(current_dir().unwrap());
                dest.push("static");
                dest.push("models");
                dest.push(doc_name);
                println!("Destination {:?}", dest);
                let _ = std::fs::copy(path,dest);
                return Some(view);
            }
            Err(err) => {
                println!("{:?}", err);
            }
        }
    }
    None
}

#[derive(Debug, Serialize, Clone)]
struct Point {
    x: f32,
    y: f32,
    z: f32,
}

impl Point {
    fn new() -> Self {
        Self {
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }

    fn min() -> Self {
        Self {
            x: f32::MIN,
            y: f32::MIN,
            z: f32::MIN,
        }
    }

    fn max() -> Self {
        Self {
            x: f32::MAX,
            y: f32::MAX,
            z: f32::MAX,
        }
    }
}
#[derive(Debug)]
struct BoundingBox {
    tl: Point,
    br: Point,
}

impl BoundingBox {
    fn full() -> Self {
        Self {
            tl: Point::min(),
            br: Point::max(),
        }
    }

    fn centroid(&mut self) -> Point {
        let mut p = Point::new();
        p.x = (self.tl.x + self.br.x) / 2.0;
        p.y = (self.tl.y + self.br.y) / 2.0;
        p.z = (self.tl.x + self.br.z) / 2.0;
        p
    }
}

fn bounding_box(mesh: IndexedMesh) -> BoundingBox {
    let mut bb = BoundingBox::full();
    for i in mesh.vertices {
        let x = i[0];
        let y = i[1];
        let z = i[2];

        // Min
        if x < bb.br.x {
            bb.br.x = x;
        }
        if y < bb.br.y {
            bb.br.y = y;
        }
        if z < bb.br.z {
            bb.br.z = z;
        }
        // Max
        if x > bb.tl.x {
            bb.tl.x = x;
        }
        if y > bb.tl.y {
            bb.tl.y = y;
        }
        if z > bb.tl.z {
            bb.tl.z = z;
        }
    }
    bb
}

#[derive(Debug, Serialize, Clone)]
pub struct View {
    pos: Point,
    look_at: Point,
    centroid: Point,
    file: String,
}

impl View {
    pub fn new() -> Self {
        Self {
            pos: Point::new(),
            look_at: Point::new(),
            file: "".to_string(),
            centroid: Point::new(),
        }
    }
    pub fn get_json(&mut self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}
