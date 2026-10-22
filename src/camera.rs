/// The window into the world. Only pixels inside the camera get rendered.
pub struct Camera {
    pub x: usize, // top-left corner, in world coordinates
    pub y: usize,
    pub width: usize, // size of the view, in pixels
    pub height: usize,
    /// How close (in pixels) the target can get to an edge before the camera scrolls.
    pub margin: usize,
}

impl Camera {
    pub fn new(width: usize, height: usize, margin: usize) -> Self {
        Self { x: 0, y: 0, width, height, margin }
    }

    /// Scroll so (tx, ty) stays at least `margin` pixels from every edge,
    /// without showing anything outside the world.
    pub fn follow(&mut self, tx: usize, ty: usize, world_w: usize, world_h: usize) {
        if tx < self.x + self.margin {
            self.x = tx.saturating_sub(self.margin);
        } else if tx + self.margin >= self.x + self.width {
            self.x = tx + self.margin + 1 - self.width;
        }

        if ty < self.y + self.margin {
            self.y = ty.saturating_sub(self.margin);
        } else if ty + self.margin >= self.y + self.height {
            self.y = ty + self.margin + 1 - self.height;
        }

        // Keep the view inside the world.
        self.x = self.x.min(world_w.saturating_sub(self.width));
        self.y = self.y.min(world_h.saturating_sub(self.height));
    }
}