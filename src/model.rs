use std::fs::File;
use std::io::{ self, BufRead };
use std::path::Path;
use std::fmt;

#[derive(Debug)]
pub struct Vector3D {
  pub x: f32,
  pub y: f32,
  pub z: f32,
}

impl Vector3D {
  pub fn new(x: f32, y: f32, z: f32) -> Vector3D {
    Vector3D {
      x,
      y,
      z,
    }
  }
}

impl fmt::Display for Vector3D {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "(x={}, y={}, z={})", self.x, self.y, self.z)
  }
}

type Face = [i32; 3];

#[derive(Debug)]
pub struct Model {
  pub vertices: Vec<Vector3D>,
  pub faces: Vec<Face>,
}

impl Model {
  pub fn new<P>(filename: &P) -> Model
  where P: AsRef<Path>, {
    let file = File::open(filename).unwrap();
    let reader = io::BufReader::new(file);
    let lines = reader.lines();

    let mut vertices: Vec<Vector3D> = Vec::new();
    let mut faces: Vec<[i32; 3]> = Vec::new();

    for line in lines {
      if let Ok(line) = line {
        if line.starts_with("v ") {
          let splitted: Vec<&str> = line.split_whitespace().collect();
          vertices.push(Vector3D::new(
            splitted[1].parse().unwrap(),
            splitted[2].parse().unwrap(),
            splitted[3].parse().unwrap(),
          ));
        } else if line.starts_with("f ") {
          let mut face: Face = [1, 1, 1];
          let splitted: Vec<&str> = line.split_whitespace().collect();
          for i in 0..3 {
            face[i] = splitted[i + 1].split("/").next().unwrap().parse().unwrap();
            face[i] -= 1;
          }
          faces.push(face);
        }
      }
    }

    Model {
      vertices,
      faces,
    }
  }

  pub fn n_vertices(&self) -> usize {
    self.vertices.len()
  }

  pub fn n_faces(&self) -> usize {
    self.faces.len()
  }

  pub fn face(&self, idx: usize) -> &Face {
    &self.faces[idx]
  }

  pub fn vertex(&self, idx: usize) -> &Vector3D {
   &self.vertices[idx]
  }
}

