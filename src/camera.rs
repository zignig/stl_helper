// Find the bounding box of the STL
// and position the camera

use std::{fs::OpenOptions, path::Path};
use stl_io::{read_stl, IndexedMesh};

fn load_stl(path: &Path) -> IndexedMesh {
    let mut file = OpenOptions::new().read(true).open(path).unwrap();
    read_stl(&mut file).unwrap()
}

#[derive(Debug)]
struct BB {
    tr: [f32; 3],
    bl: [f32; 3],
}

fn bounding_box(mesh: IndexedMesh) -> BB {
    let mut bounds = BB {
        tr: [f32::MIN, f32::MIN, f32::MIN],
        bl: [f32::MAX, f32::MAX, f32::MAX],
    };
    let vecs = mesh.vertices;
    for v in vecs {
        // MIN
        if v[0] < bounds.bl[0] {
            bounds.bl[0] = v[0];
        }
        if v[1] < bounds.bl[1] {
            bounds.bl[1] = v[1];
        }
        if v[2] < bounds.bl[2] {
            bounds.bl[2] = v[2];
        }
        // MAX
        if v[0] > bounds.tr[0] {
            bounds.tr[0] = v[0];
        }
        if v[1] > bounds.bl[1] {
            bounds.tr[1] = v[1];
        }
        if v[2] > bounds.tr[2] {
            bounds.tr[2] = v[2];
        }
    }
    println!("{:#?}", bounds);
    bounds
}

pub fn process(path: &Path) {
    let mesh = load_stl(path);
    let _ = bounding_box(mesh);
}
