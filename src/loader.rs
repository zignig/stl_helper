use serde::Serialize;
use serde_json::{Result, Value};
use std::env::current_dir;
use std::io::Read;
use std::path::PathBuf;
use std::{fs::OpenOptions, io::Seek};
use stl_io::{read_stl, IndexedMesh};

use crate::storage::Storage;

pub fn process(path: &PathBuf, storage: &Storage) -> Option<View> {
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

                // Store the file and view 
                let mut maps = storage.map.lock().unwrap();
                maps.insert(doc_name.to_string(), view.clone());

                // Stash the data
                file.rewind().unwrap();
                let mut buf: Vec<u8> = Vec::new();
                let _ = file.read_to_end(&mut buf).unwrap();
                let mut data = storage.data.lock().unwrap();
                data.insert(doc_name.to_string(),buf);
                println!("{:#?}", data.keys());
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
