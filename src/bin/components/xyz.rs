/// This is a simple component.
#[derive(Debug)]
pub struct XYZ {
    x: i32,
    y: i32,
    z: i32,
}

impl XYZ {
    /// Create new `xyz` component.
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    /// Set the x value.
    pub fn set_x(&mut self, x: i32) {
        self.x = x;
    }

    /// Set the y value.
    pub fn set_y(&mut self, y: i32) {
        self.y = y;
    }

    /// Set the z value.
    pub fn set_z(&mut self, z: i32) {
        self.z = z;
    }
}