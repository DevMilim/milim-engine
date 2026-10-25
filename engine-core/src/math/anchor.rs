#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Anchor {
    Center,
    TopLeft,
    TopCenter,
    TopRight,
    CenterLeft,
    CenterRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

impl Anchor {
    #[inline]
    pub const fn factors(&self) -> (f32, f32) {
        match self {
            Anchor::TopLeft => (0.0, 0.0),
            Anchor::TopCenter => (0.5, 0.0),
            Anchor::TopRight => (1.0, 0.0),
            Anchor::CenterLeft => (0.0, 0.5),
            Anchor::Center => (0.5, 0.5),
            Anchor::CenterRight => (1.0, 0.5),
            Anchor::BottomLeft => (0.0, 1.0),
            Anchor::BottomCenter => (0.5, 1.0),
            Anchor::BottomRight => (1.0, 1.0),
        }
    }

    #[inline]
    pub fn top_left(&self, x: f32, y: f32, w: f32, h: f32) -> (f32, f32) {
        let (fx, fy) = self.factors();
        (x - fx * w, y - fy * h)
    }
    #[inline]
    pub fn rotated_center(&self, x: f32, y: f32, w: f32, h: f32, rotation: f32) -> (f32, f32) {
        let (fx, fy) = self.factors();
        let ox = (0.5 - fx) * w;
        let oy = (0.5 - fy) * h;
        let (sin, cos) = rotation.sin_cos();
        (x + ox * cos - oy * sin, y + ox * sin + oy * cos)
    }
}
