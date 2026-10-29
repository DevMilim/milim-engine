use engine_core::{
    core::EngineApi,
    math::Vector2,
    object::{Base, Component, Id},
};

use crate::{AABB, ColliderData, ColliderKey, CollisionWorld, EventFactory, Layer};

pub struct Collider {
    pub key: Id,
    pub width: i32,
    pub height: i32,
    pub offset_x: i32,
    pub offset_y: i32,
    pub layer: Layer,
    pub mask: Layer,
    pub debug: bool,
    pub disabled: bool,
    pub is_sensor: bool,
    pub one_way_collision: bool,
    pub follow_transform: bool,
    pub on_enter: Option<EventFactory>,
    pub on_exit: Option<EventFactory>,
    in_world: bool,
}

impl Collider {
    pub fn set_on_enter<T: Clone + Send + 'static>(&mut self, event: T) {
        self.on_enter = Some(Box::new(move || Box::new(event.clone())));
    }
    pub fn set_on_exit<T: Clone + Send + 'static>(&mut self, event: T) {
        self.on_exit = Some(Box::new(move || Box::new(event.clone())));
    }
    pub fn enable(&mut self) {
        self.disabled = false;
    }
    pub fn disable(&mut self) {
        self.disabled = true;
    }
}

impl Default for Collider {
    fn default() -> Self {
        Self {
            key: Id::new(),
            width: 16,
            height: 16,
            offset_x: 0,
            offset_y: 0,
            layer: Layer::LAYER_1,
            mask: Layer::LAYER_1,
            debug: false,
            disabled: false,
            is_sensor: false,
            on_enter: None,
            on_exit: None,
            one_way_collision: false,
            follow_transform: true,
            in_world: true,
        }
    }
}

impl Component for Collider {
    fn start(&mut self, ctx: &mut impl EngineApi, base: &mut Base) {
        if self.on_enter.is_some() || self.on_exit.is_some() {
            let key = ColliderKey {
                key: self.key,
                id: base.id,
            };
            let world = ctx.require_resource_mut::<CollisionWorld>();
            world.register_triggers(key, self.on_enter.take(), self.on_exit.take());
        }
    }

    fn fixed_update(&mut self, ctx: &mut impl EngineApi, base: &mut Base, _delta: f32) {
        let key = ColliderKey {
            key: self.key,
            id: base.id,
        };
        let world = ctx.require_resource_mut::<CollisionWorld>();

        if self.disabled {
            if self.in_world {
                world.remove_collider(key);
                self.in_world = false;
            }
            return;
        }

        let gp: Vector2 = base.transform.global_position;
        let x = gp.x.round() as i32 - self.width / 2 + self.offset_x;
        let y = gp.y.round() as i32 - self.height / 2 + self.offset_y;

        if self.follow_transform {
            world.update_collider(
                key,
                ColliderData {
                    aabb: AABB {
                        x,
                        y,
                        width: self.width,
                        height: self.height,
                    },
                    layer: self.layer,
                    mask: self.mask,
                    is_sensor: self.is_sensor,
                    on_way_collision: self.one_way_collision,
                },
            );
        } else {
            world.update_collider_geometry(
                key,
                self.layer,
                self.mask,
                self.is_sensor,
                self.one_way_collision,
                (self.width, self.height),
                (x, y),
            );
        }
        self.in_world = true;
    }

    fn destroy(&mut self, ctx: &mut impl EngineApi, base: &Base) {
        let key = ColliderKey {
            key: self.key,
            id: base.id,
        };
        let world = ctx.require_resource_mut::<CollisionWorld>();
        world.remove_collider(key);
        world.unregister_triggers(key);
    }
}
