mod vertex;

/// Used for using for making Vectors
pub trait Vector {
    /// Gets the 2d position of the Vector
    fn get2D(&self) -> [f32; 2];
    /// Gets the 3d position of the Vector
    fn get3D(&self) -> [f32; 3];
}

impl<'a> Vector for Vec2 {

    fn get2D(&self) -> [f32; 2] {
        [self.x, self.y]
    }

    fn get3D(&self) -> [f32; 3] {
        [self.x, self.y, 0f32]
    }
}

/// A vector 2, used for positions on a 2d plain.
#[derive(Clone, Copy)]
pub struct Vec2 {
    /// the x value on a coordinate plain
    pub x: f32,
    /// the y vale on a coordinate plain
    pub y: f32,
}

impl Vec2 {
    pub fn new(pos: [f32;2]) -> Self {
        Vec2 {
            x: pos[0],
            y: pos[1]
        }
    }
}

impl Default for Vec2 {
    fn default() -> Self {
        Self {
            x: 0f32,
            y: 0f32
        }
    }
}

/// A 3d vector, used for position on a 3d plain.
#[derive(Clone, Copy)]
pub struct Vec3 {
    /// the x value on a coordinate plain
    pub x: f32,
    /// the y value on a coordinate plain
    pub y: f32,
    /// the x value on a coordinate plain
    pub z: f32,
}

impl Vector for Vec3 {
    
    fn get2D(&self) -> [f32; 2] {
        [self.x, self.y]
    }

    fn get3D(&self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Self {
            x: 0f32,
            y: 0f32,
            z: 0f32
        }
    }
}