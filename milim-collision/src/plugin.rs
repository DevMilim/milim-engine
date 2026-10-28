use engine_core::{core::CoreApi, plugin::Plugin};

use crate::CollisionWorld;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&mut self, app: &mut engine_core::core::AppBuilder) {
        app.set_resource(CollisionWorld::default());
    }
    fn pre_fixed_update(&mut self, ctx: &mut engine_core::core::Context, _delta: f32) {
        ctx.require_resource_mut::<CollisionWorld>().step();
    }
    fn post_fixed_update(&mut self, ctx: &mut engine_core::core::Context, _delta: f32) {
        ctx.require_resource_mut::<CollisionWorld>().commit();
    }
}
