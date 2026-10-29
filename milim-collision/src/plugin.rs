use std::any::Any;

use engine_core::{
    core::{CoreApi, EventApi},
    object::Id,
    plugin::Plugin,
};

use crate::CollisionWorld;

pub struct CollisionPlugin {
    outbox: Vec<(Id, Box<dyn Any + Send>)>,
}

impl Default for CollisionPlugin {
    fn default() -> Self {
        Self { outbox: Vec::new() }
    }
}

impl Plugin for CollisionPlugin {
    fn build(&mut self, app: &mut engine_core::core::AppBuilder) {
        app.set_resource(CollisionWorld::default());
    }
    fn pre_fixed_update(&mut self, ctx: &mut engine_core::core::Context, _delta: f32) {
        ctx.require_resource_mut::<CollisionWorld>().step();
    }
    fn post_fixed_update(&mut self, ctx: &mut engine_core::core::Context, _delta: f32) {
        let world = ctx.require_resource_mut::<CollisionWorld>();
        world.step();
        self.outbox.clear();
        world.collect_trigger_events(&mut self.outbox);
        world.commit();

        for (id, event) in self.outbox.drain(..) {
            ctx.send_boxed_any(id, event);
        }
    }
}
