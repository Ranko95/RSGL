use std::fs::File;
use std::io::{ self, BufRead };
use std::path::Path;

use crate::geometry::Vector3D;

type Face = [i32; 3];
type Vertex = Vector3D<f32>;

#[derive(Debug)]
pub struct Model {
  pub vertices: Vec<Vertex>,
  pub faces: Vec<Face>,
}

impl Model {
  pub fn new<P>(filename: &P) -> Model
  where P: AsRef<Path>, {
    let file = File::open(filename).unwrap();
    let reader = io::BufReader::new(file);
    let lines = reader.lines();

    let mut vertices: Vec<Vertex> = Vec::new();
    let mut faces: Vec<Face> = Vec::new();

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

  pub fn vertex(&self, idx: usize) -> &Vertex {
   &self.vertices[idx]
  }
}

