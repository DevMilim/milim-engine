use crate::{
    math::{Transform, Vector2},
    object::Id,
};

#[derive(Clone)]
pub struct Base {
    pub id: Id,
    pub parent: Option<Id>,
    pub transform: Transform,
    pub pending_removal: bool,
    pub top_level: bool,
    is_started: bool,
    destroyed: bool,
}

impl Base {
    pub fn new(position: Vector2) -> Self {
        Self {
            id: Id::new(),
            parent: None,
            transform: Transform::new(position.x, position.y),
            pending_removal: false,
            top_level: false,
            is_started: false,
            destroyed: false,
        }
    }
}

impl Default for Base {
    fn default() -> Self {
        Base::new(Vector2::ZERO)
    }
}

impl GameObjectBase for Base {
    fn base(&self) -> &Base {
        self
    }
    fn base_mut(&mut self) -> &mut Base {
        self
    }
}

pub trait GameObjectBase {
    fn base(&self) -> &Base;
    fn base_mut(&mut self) -> &mut Base;

    fn id(&self) -> Id {
        self.base().id
    }
    fn parent(&self) -> Option<Id> {
        self.base().parent
    }

    fn is_started(&self) -> bool {
        self.base().is_started
    }
    fn is_destroyed(&self) -> bool {
        self.base().destroyed
    }
    fn mark_as_started(&mut self) {
        self.base_mut().is_started = true
    }
    fn mark_as_destroyed(&mut self) {
        let b = self.base_mut();
        b.destroyed = true;
        b.pending_removal = true;
    }
    fn top_level(&mut self) {
        self.base_mut().top_level = true
    }
    fn queue_free(&mut self) {
        self.base_mut().pending_removal = true
    }

    fn position(&self) -> Vector2 {
        self.base().transform.position
    }

    fn rotation(&self) -> f32 {
        self.base().transform.rotation
    }
    fn scale(&self) -> Vector2 {
        self.base().transform.scale
    }
    fn global_position(&self) -> Vector2 {
        self.base().transform.global_position
    }
    fn global_rotation(&self) -> f32 {
        self.base().transform.global_rotation
    }
    fn global_scale(&self) -> Vector2 {
        self.base().transform.global_scale
    }

    fn set_position(&mut self, p: Vector2) {
        self.base_mut().transform.position = p
    }
    fn set_rotation(&mut self, r: f32) {
        self.base_mut().transform.rotation = r
    }
    fn set_scale(&mut self, s: Vector2) {
        self.base_mut().transform.scale = s
    }

    fn translate(&mut self, d: Vector2) {
        let p = &mut self.base_mut().transform.position;
        p.x += d.x;
        p.y += d.y;
    }
    fn rotate(&mut self, delta: f32) {
        self.base_mut().transform.rotate(delta)
    }

    fn set_global_position(&mut self, p: Vector2) {
        self.base_mut().transform.set_global_position(p)
    }
    fn set_global_rotation(&mut self, r: f32) {
        self.base_mut().transform.set_global_rotation(r)
    }
    fn set_global_scale(&mut self, s: Vector2) {
        self.base_mut().transform.set_global_scale(s)
    }

    fn to_global(&self, local: Vector2) -> Vector2 {
        self.base().transform.to_global(local)
    }
    fn to_local(&self, global: Vector2) -> Vector2 {
        self.base().transform.to_local(global)
    }

    fn look_at(&mut self, target: Vector2) {
        let g = self.global_position();
        self.set_global_rotation((target.y - g.y).atan2(target.x - g.x));
    }
}
