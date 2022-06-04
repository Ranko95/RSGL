use std::path::Path;

use crate::image::{ Image, Color };
use crate::model::Model;
use crate::geometry::{
  Vector2D,
  Vector3D,
  find_triangle_bounding_box,
  barycentric,
};

pub struct GL {
  pub image: Image,
}

impl GL {
  pub fn new(image: Image) -> GL {
    GL {
      image,
    }
  }

  pub fn set_point(&mut self, x: i32, y: i32, color: Color) -> () {
    if x < 0 || y < 0 {
      return;
    }

    if x >= self.image.width || y >= self.image.height {
      return;
    }

    self.image.data[(x + y * self.image.width) as usize] = color;
  }

  pub fn draw_line(
    &mut self,
    p0: &Vector2D<i32>,
    p1: &Vector2D<i32>,
    color: Color,
  ) -> () {
    let mut dx = p1.x - p0.x;
    let mut dy = p1.y - p0.y;

    let mut step_x = 0;
    let mut step_y = 0;

    if dx < 0 {
      dx = dx.abs();
      step_x = -1;
    } else {
      step_x = 1;
    }

    if dy < 0 {
      dy = dy.abs();
      step_y = -1;
    } else {
      step_y = 1;
    }

    dx <<= 1;
    dy <<= 1;

    let mut x = p0.x;
    let mut y = p0.y;

    self.set_point(x, y, color);

    if dx > dy {
      let mut p = dy - (dx >> 1);

      while x != p1.x {
        x += step_x;
        if p >= 0 {
          y += step_y;
          p -= dx;
        }
        p += dy;

        self.set_point(x, y, color);
      }
    } else {
      let mut p = dx - (dy >> 1);

      while y != p1.y {
        y += step_y;

        if p >= 0 {
          x += step_x;
          p -= dy;
        }
        p += dx;

        self.set_point(x, y, color);
      }
    }
  }

  pub fn draw_triangle(
    &mut self,
    pts: [Vector2D<i32>; 3],
    color: Color,
  ) -> () {
    let [top_left, bottom_right] = find_triangle_bounding_box(&pts);

    let mut x = top_left.x;
    let mut y = bottom_right.y;
  
    while x <= bottom_right.x {
      while y <= top_left.y {
        let bc_screen = barycentric(Vector2D::new(x, y), pts);

        if bc_screen.x >= 0.0 && bc_screen.y >= 0.0 && bc_screen.z >= 0.0 {
          self.set_point(x, y, color);
        }

        y += 1;
      }
      y = bottom_right.y;
      x += 1;
    }
  }

  pub fn render(&mut self, model: &Model) -> () {
    let light_dir = Vector3D::new(0.0, 0.0, -1.0);

    let half_width = self.image.width as f32/2.;
    let half_height = self.image.height as f32/2.;  

    for i in 0..model.n_faces() {
      let face = model.face(i);

      let mut screen_coords: [Vector2D<i32>; 3] = [Vector2D::new(0, 0); 3];
      let mut world_coords: [Vector3D<f32>; 3] = [Vector3D::new(0., 0., 0.); 3];

      for j in 0..3 {
        let v = model.vertex(face[j] as usize);
        let x = ((v.x + 1.) * half_width) as i32;
        let y = ((v.y + 1.) * half_height) as i32;

        let p = Vector2D::new(x, y);

        screen_coords[j] = p;
        world_coords[j] = *v;
      }

      let n = (world_coords[2] - world_coords[1]) ^ (world_coords[1] - world_coords[0]);
      let normalized = n.normalize();

      let intensity = normalized * light_dir;

      if intensity > 0.0 {
        let alpha = (intensity * 255.0) as u8;
        self.draw_triangle(screen_coords, Color(alpha, alpha, alpha, 255));
      }

    }

    self.image.write(&Path::new("output.tga")).unwrap();
  }
}
