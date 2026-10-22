use crate::camera::Camera;
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub density: f32,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Material {
    /// Color packed as 0x00RRGGBB, the format minifb draws.
    pub fn color(&self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | self.b as u32
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Pixel {
    pub material: Material,
}

/// A starting pixel to grow the world from.
pub struct Seed {
    pub x: usize,
    pub y: usize,
    pub pixel: Pixel,
}

pub struct World {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Pixel>, // row by row: index = y * width + x
    pub camera: Camera,
}

impl World {
    /// Place the seeds, then grow outward from them until the world is full.
    /// Each empty cell copies the seed that reaches it first, so every seed
    /// grows into a region around itself.
    pub fn grow_from_seeds(width: usize, height: usize, seeds: &[Seed], camera: Camera) -> Self {
        assert!(!seeds.is_empty(), "need at least one seed to grow a world");

        let mut cells: Vec<Option<Pixel>> = vec![None; width * height];
        let mut frontier = VecDeque::new();

        for seed in seeds {
            let i = seed.y * width + seed.x;
            cells[i] = Some(seed.pixel);
            frontier.push_back((seed.x, seed.y));
        }

        // Breadth-first flood fill: each step claims the empty neighbors
        // of cells that are already filled.
        while let Some((x, y)) = frontier.pop_front() {
            let pixel = cells[y * width + x];
            let neighbors = [
                (x.wrapping_sub(1), y),
                (x + 1, y),
                (x, y.wrapping_sub(1)),
                (x, y + 1),
            ];
            for (nx, ny) in neighbors {
                // wrapping_sub turns -1 into a huge number, so this one check
                // covers both edges.
                if nx < width && ny < height && cells[ny * width + nx].is_none() {
                    cells[ny * width + nx] = pixel;
                    frontier.push_back((nx, ny));
                }
            }
        }

        let pixels = cells.into_iter().map(|c| c.unwrap()).collect();
        Self { width, height, pixels, camera }
    }

    pub fn get(&self, x: usize, y: usize) -> &Pixel {
        &self.pixels[y * self.width + x]
    }

    /// Draw only the part of the world the camera sees into `buffer`,
    /// which must be camera.width * camera.height long.
    pub fn render(&self, buffer: &mut [u32]) {
        let cam = &self.camera;
        for sy in 0..cam.height {
            for sx in 0..cam.width {
                let pixel = self.get(cam.x + sx, cam.y + sy);
                buffer[sy * cam.width + sx] = pixel.material.color();
            }
        }
    }
}