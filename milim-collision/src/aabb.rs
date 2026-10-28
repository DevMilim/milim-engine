use engine_core::math::Vector2;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct AABB {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl AABB {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn min_x(&self) -> i32 {
        self.x
    }

    pub fn max_x(&self) -> i32 {
        self.x + self.width
    }

    pub fn min_y(&self) -> i32 {
        self.y
    }

    pub fn max_y(&self) -> i32 {
        self.y + self.height
    }

    pub fn center(&self) -> Vector2 {
        Vector2::new(
            (self.x + self.width / 2) as f32,
            (self.y + self.height / 2) as f32,
        )
    }

    pub fn position(&self) -> Vector2 {
        Vector2::new(self.x as f32, self.y as f32)
    }

    pub fn intersects(&self, other: &AABB) -> bool {
        self.min_x() < other.max_x()
            && self.max_x() > other.min_x()
            && self.min_y() < other.max_y()
            && self.max_y() > other.min_y()
    }

    pub fn get_overlap(&self, other: &AABB) -> Option<Vector2> {
        if !self.intersects(other) {
            return None;
        }

        let overlap_x = if (self.min_x() + self.max_x()) < (other.min_x() + other.max_x()) {
            other.min_x() - self.max_x()
        } else {
            other.max_x() - self.min_x()
        };

        let overlap_y = if (self.min_y() + self.max_y()) < (other.min_y() + other.max_y()) {
            other.min_y() - self.max_y()
        } else {
            other.max_y() - self.min_y()
        };

        Some(Vector2::new(overlap_x as f32, overlap_y as f32))
    }

    pub fn contains_point(&self, point: Vector2) -> bool {
        point.x >= self.min_x() as f32
            && point.x <= self.max_x() as f32
            && point.y >= self.min_y() as f32
            && point.y <= self.max_y() as f32
    }
}
