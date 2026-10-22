// RGB Helper Function
pub fn rgb(r: u8, g: u8, b: u8) -> u32 {
    ((r as u32) << 16) | ((g as u32) << 8) | b as u32
}

pub struct Game {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u32>, // one u32 per pixel, row by row
}

impl Game {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height, 
            pixels: vec![0; width * height],
        }
    }

    /// Advance the world by one tick. Empty for now — sand physics goes here later.
    pub fn update(&mut self) {}

    /// Write the world into the pixel buffer.
    pub fn draw(&mut self) {
        self.pixels.fill(rgb(20, 20, 30)); // clear to dark background

        // Our single pixel, dead center.
        let (x, y) = (self.width / 2, self.height / 2);
        self.pixels[y * self.width + x] = rgb(255, 200, 50);
        self.pixels[y * self.width + x + 1] = rgb(255, 150, 50);
        self.pixels[y * self.width + x + 2] = rgb(255, 100, 50);
        self.pixels[y * self.width + x + 3] = rgb(255, 80, 50);
        self.pixels[y * self.width + x + 5] = rgb(255, 50, 50);
    }
}