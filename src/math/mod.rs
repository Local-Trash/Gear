mod vertex;

/// Used for using for making vectors
pub trait Vectors {
    /// Adds two Vectors together.
    fn add(self, vector: impl Vectors);
    /// Subtracts two Vectors together.
    fn subtract(self, vector: impl Vectors);
    /// Gets the 2d position of the Vector
    fn get2D(self) -> [f32; 2];
    /// Gets the 3d position of the Vector
    fn get3D(self) -> [f32; 3];
}

impl Vectors for Vec2 {
    fn add(mut self, vector: impl Vectors) {
        let [x, y] = vector.get2D();
        self.x += x;
        self.y += y;
    }

    fn subtract(mut self, vector: impl Vectors) {
        let [x, y] = vector.get2D();
        self.x -= x;
        self.y -= y;
    }

    fn get2D(self) -> [f32; 2] {
        [self.x, self.y]
    }

    fn get3D(self) -> [f32; 3] {
        [self.x, self.y, 0f32]
    }
}

/// A vector 2, used for positions on a 2d plain.
#[derive(Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
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
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vectors for Vec3 {
    fn add(mut self, vector: impl Vectors) {
        let [x,y,z] = vector.get3D();
        self.x += x;
        self.y += y;
        self.z += z;
    }

    fn subtract(mut self, vector: impl Vectors) {
        let [x,y,z] = vector.get3D();
        self.x -= x;
        self.y -= y;
        self.z -= z;
    }
    
    fn get2D(self) -> [f32; 2] {
        [self.x, self.y]
    }

    fn get3D(self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }
}