pub trait TooF64 {
    fn too_f64(&self) -> f64;
}

impl TooF64 for u32 {
    fn too_f64(&self) -> f64 {
        *self as f64
    }
}

impl TooF64 for i32 {
    fn too_f64(&self) -> f64 {
        *self as f64
    }
}

impl TooF64 for f32 {
    fn too_f64(&self) -> f64 {
        *self as f64
    }
}

impl TooF64 for f64 {
    fn too_f64(&self) -> f64 {
        *self
    }
}

pub struct CVector {
    x: f64,
    y: f64,
}

impl CVector {
    pub fn new<T>(x: T, y: T) -> CVector
    where
        T: Copy + TooF64,
    {
        let x = x.too_f64();
        let y = y.too_f64();

        CVector { x, y }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn mult_x<T>(&mut self, multiple: T)
    where
        T: Copy + TooF64,
    {
        let multiple = multiple.too_f64();
        self.x *= multiple;
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn mult_y<T>(&mut self, multiple: T)
    where
        T: Copy + TooF64,
    {
        let multiple = multiple.too_f64();
        self.y *= multiple;
    }

    pub fn mag(&self) -> f64 {
        ((self.x * self.x) + (self.y * self.y)).sqrt()
    }

    pub fn add(&mut self, other: &CVector) {
        self.x += other.x();
        self.y += other.y();
    }

    pub fn sub(&mut self, other: &CVector) {
        self.x -= other.x();
        self.y -= other.y();
    }

    pub fn normalise(&mut self) {
        let mag = self.mag();
        self.x /= mag;
        self.y /= mag;
    }

    pub fn mult_mag<T>(&mut self, multiple: T)
    where
        T: Copy + TooF64,
    {
        let multiple = multiple.too_f64();

        self.x *= multiple;
        self.y *= multiple;
    }

    pub fn set_mag<T>(&mut self, new_mag: T)
    where
        T: Copy + TooF64,
    {
        let new_mag = new_mag.too_f64();
        self.normalise();
        self.mult_mag(new_mag);
    }
}
