use crate::math::{Anchor, Vector2};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl Rect {
    pub const fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn into_tuple(self) -> (i32, i32, u32, u32) {
        (self.x, self.y, self.width, self.height)
    }

    pub fn min_x(&self) -> i32 {
        self.x
    }

    pub fn max_x(&self) -> i32 {
        self.x + self.width as i32
    }

    pub fn min_y(&self) -> i32 {
        self.y
    }

    pub fn max_y(&self) -> i32 {
        self.y + self.height as i32
    }
    pub fn intersects(&self, other: &Self) -> bool {
        self.min_x() < other.max_x()
            && self.max_x() > other.min_x()
            && self.min_y() < other.max_y()
            && self.max_y() > other.min_y()
    }

    pub const fn from_center(center: Vector2, size: Vector2) -> Self {
        let x = round_to_i32(center.x - size.x / 2.0);
        let y = round_to_i32(center.y - size.y / 2.0);

        Self::new(x, y, round_to_u32(size.x), round_to_u32(size.y))
    }
    pub fn new_from_vecs(position: Vector2, size: Vector2) -> Self {
        Self {
            x: position.x.round() as i32,
            y: position.y.round() as i32,
            width: size.x.round() as u32,
            height: size.y.round() as u32,
        }
    }
    pub fn anchored(position: Vector2, size: Vector2, anchor: Anchor) -> Self {
        let (x, y) = anchor.top_left(position.x, position.y, size.x, size.y);
        Self::new_from_vecs(Vector2::new(x, y), size)
    }

    pub fn get_anchor(self, anchor: Anchor) -> Self {
        Self::anchored(
            Vector2::new(self.x as f32, self.y as f32),
            Vector2::new(self.width as f32, self.height as f32),
            anchor,
        )
    }
}
const fn round_to_i32(value: f32) -> i32 {
    if value >= 0.0 {
        (value + 0.5) as i32
    } else {
        (value - 0.5) as i32
    }
}
const fn round_to_u32(value: f32) -> u32 {
    if value >= 0.0 {
        (value + 0.5) as u32
    } else {
        (value - 0.5) as u32
    }
}
