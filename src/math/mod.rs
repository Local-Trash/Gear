mod vertex;

/// Used for using for making Vectors
pub trait VectorProp {
    /// Creats a new Vector
    fn new(pos: &[f32]) -> Self;
    /// Adds two VectorProp together.
    fn add(self, vector: impl VectorProp);
    /// Subtracts two VectorProp together.
    fn subtract(self, vector: impl VectorProp);
    /// Gets the 2d position of the Vector
    fn get2D(self) -> [f32; 2];
    /// Gets the 3d position of the Vector
    fn get3D(self) -> [f32; 3];
}

impl VectorProp for Vec2 {
    fn new(pos: &[f32]) -> Self {
        Self {
            x: pos[0],
            y: pos[1]
        }
    }

    fn add(mut self, vector: impl VectorProp) {
        let [x, y] = vector.get2D();
        self.x += x;
        self.y += y;
    }

    fn subtract(mut self, vector: impl VectorProp) {
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
    /// the x value on a coordinate plain
    pub x: f32,
    /// the y vale on a coordinate plain
    pub y: f32,
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

impl VectorProp for Vec3 {
    fn new(pos: &[f32]) -> Self {
        Self {
            x: pos[0],
            y: pos[1],
            z: pos[2]
        }
    }

    fn add(mut self, vector: impl VectorProp) {
        let [x,y,z] = vector.get3D();
        self.x += x;
        self.y += y;
        self.z += z;
    }

    fn subtract(mut self, vector: impl VectorProp) {
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

impl Default for Vec3 {
    fn default() -> Self {
        Self {
            x: 0f32,
            y: 0f32,
            z: 0f32
        }
    }
}

/// Stores the Vector for the Entity.
pub trait Vectors {
    type Vector: VectorProp;
}