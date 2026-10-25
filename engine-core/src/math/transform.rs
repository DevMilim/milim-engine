use std::f32::consts::{PI, TAU};

use crate::math::Vector2;

/// Estado global do pai, guardado no filho no último `apply_parent`.
/// Serve para converter global -> local nos setters.
#[derive(Clone, Copy, Debug)]
struct Frame {
    position: Vector2,
    scale: Vector2,
    rotation: f32,
    cos: f32,
    sin: f32,
}

impl Frame {
    const IDENTITY: Self = Self {
        position: Vector2::ZERO,
        scale: Vector2::ONE,
        rotation: 0.0,
        cos: 1.0,
        sin: 0.0,
    };
}

/// Ordem de composição: global = pai.posição + R(pai) * (S(pai) * local).
/// Com escala não uniforme no pai, a rotação global é uma aproximação (soma dos ângulos).
#[derive(Clone, Copy, Debug)]
pub struct Transform {
    pub position: Vector2,
    pub rotation: f32,
    pub scale: Vector2,

    pub global_position: Vector2,
    pub global_rotation: f32,
    pub global_scale: Vector2,

    // cache de sin/cos da rotação global, reaproveitado pelos filhos
    global_cos: f32,
    global_sin: f32,
    parent: Frame,
}

#[inline]
fn wrap_angle(a: f32) -> f32 {
    if (-PI..PI).contains(&a) {
        a
    } else {
        (a + PI).rem_euclid(TAU) - PI
    }
}

#[inline]
fn safe_div(a: f32, b: f32) -> f32 {
    if b.abs() > f32::EPSILON { a / b } else { 0.0 }
}

impl Transform {
    pub const EMPTY: Self = Self {
        position: Vector2::ZERO,
        rotation: 0.0,
        scale: Vector2::ONE,
        global_position: Vector2::ZERO,
        global_rotation: 0.0,
        global_scale: Vector2::ONE,
        global_cos: 1.0,
        global_sin: 0.0,
        parent: Frame::IDENTITY,
    };

    pub fn new(x: f32, y: f32) -> Self {
        let p = Vector2 { x, y };
        Self {
            position: p,
            global_position: p,
            ..Self::EMPTY
        }
    }

    pub fn apply_parent(&mut self, parent: &Transform, inherit: bool) {
        self.parent = if inherit {
            Frame {
                position: parent.global_position,
                scale: parent.global_scale,
                rotation: parent.global_rotation,
                cos: parent.global_cos,
                sin: parent.global_sin,
            }
        } else {
            Frame::IDENTITY
        };
        self.recompute_global();
    }

    fn recompute_global(&mut self) {
        let p = self.parent;

        self.global_scale = Vector2 {
            x: self.scale.x * p.scale.x,
            y: self.scale.y * p.scale.y,
        };
        self.global_rotation = wrap_angle(self.rotation + p.rotation);

        let sx = self.position.x * p.scale.x;
        let sy = self.position.y * p.scale.y;
        self.global_position = Vector2 {
            x: p.position.x + sx * p.cos - sy * p.sin,
            y: p.position.y + sx * p.sin + sy * p.cos,
        };

        if self.global_rotation == 0.0 {
            self.global_cos = 1.0;
            self.global_sin = 0.0;
        } else {
            (self.global_sin, self.global_cos) = self.global_rotation.sin_cos();
        }
    }

    pub fn set_global_position(&mut self, g: Vector2) {
        let p = self.parent;
        let dx = g.x - p.position.x;
        let dy = g.y - p.position.y;
        let rx = dx * p.cos + dy * p.sin;
        let ry = -dx * p.sin + dy * p.cos;
        self.position = Vector2 {
            x: safe_div(rx, p.scale.x),
            y: safe_div(ry, p.scale.y),
        };
        self.recompute_global();
    }

    pub fn set_global_rotation(&mut self, g: f32) {
        self.rotation = wrap_angle(g - self.parent.rotation);
        self.recompute_global();
    }

    pub fn set_global_scale(&mut self, g: Vector2) {
        self.scale = Vector2 {
            x: safe_div(g.x, self.parent.scale.x),
            y: safe_div(g.y, self.parent.scale.y),
        };
        self.recompute_global();
    }

    pub fn rotate(&mut self, delta: f32) {
        self.rotation = wrap_angle(self.rotation + delta);
    }

    pub fn to_global(&self, p: Vector2) -> Vector2 {
        let sx = p.x * self.global_scale.x;
        let sy = p.y * self.global_scale.y;
        Vector2 {
            x: self.global_position.x + sx * self.global_cos - sy * self.global_sin,
            y: self.global_position.y + sx * self.global_sin + sy * self.global_cos,
        }
    }

    pub fn to_local(&self, g: Vector2) -> Vector2 {
        let dx = g.x - self.global_position.x;
        let dy = g.y - self.global_position.y;
        Vector2 {
            x: safe_div(
                dx * self.global_cos + dy * self.global_sin,
                self.global_scale.x,
            ),
            y: safe_div(
                -dx * self.global_sin + dy * self.global_cos,
                self.global_scale.y,
            ),
        }
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::EMPTY
    }
}
