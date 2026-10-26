use crate::{
    core::{Context, EngineApi},
    object::{Base, EmptyGlobals, GameObjectDispatch},
    plugin::PluginManager,
};

pub const FIXED_DT: f32 = 1.0 / 60.0;
pub const MAX_DELTA: f32 = 0.25;

pub trait Scene {
    fn get_dispatch(&mut self) -> &mut impl GameObjectDispatch;
}

pub struct WorldState<S: Scene, P: GameObjectDispatch = EmptyGlobals> {
    pub scenes: Vec<S>,
    pub global: Option<P>,
    pub accumulator: f32,
    pub is_running: bool,
}

impl<S: Scene, P: GameObjectDispatch> WorldState<S, P> {
    pub fn new(root_scene: S) -> Self {
        Self {
            scenes: vec![root_scene],
            accumulator: 0.0,
            global: None,
            is_running: true,
        }
    }
    pub fn change_scene(&mut self, scene: S, ctx: &mut impl EngineApi) {
        if let Some(mut old) = self.scenes.pop() {
            old.get_dispatch().dispatch_destroy(ctx);
        }
        self.scenes.push(scene);
    }
    pub fn pop_scene(&mut self, ctx: &mut impl EngineApi) {
        if let Some(mut scene) = self.scenes.pop() {
            scene.get_dispatch().dispatch_destroy(ctx);
        }
    }
    pub fn push_scene(&mut self, scene: S) {
        self.scenes.push(scene);
    }
    pub fn clear_scenes(&mut self, ctx: &mut impl EngineApi) {
        for mut scene in self.scenes.drain(..) {
            scene.get_dispatch().dispatch_destroy(ctx);
        }
    }
    pub fn last_scene(&mut self) -> Option<&mut impl GameObjectDispatch> {
        Some(self.scenes.last_mut()?.get_dispatch())
    }
    pub fn destroy_pending_top(&mut self, ctx: &mut impl EngineApi) {
        if let Some(top) = self.scenes.last_mut() {
            let d = top.get_dispatch();
            if d.is_pending_removal() {
                d.dispatch_destroy(ctx);
            }
        }
    }

    pub fn reap_top(&mut self) {
        while self
            .scenes
            .last_mut()
            .is_some_and(|s| s.get_dispatch().is_pending_removal())
        {
            self.scenes.pop();
        }
    }

    pub fn update(
        &mut self,
        ctx: &mut Context,
        plugins: &mut PluginManager,
        base: &Base,
        fixed_update_count: &mut u64,
        mut delta_time: f32,
    ) -> f32 {
        delta_time = delta_time.min(MAX_DELTA);

        self.accumulator += delta_time;

        let Some(object) = self.scenes.last_mut() else {
            self.quit();
            return 0.0;
        };

        let object = object.get_dispatch();
        const MAX_FIXED_STEPS: u32 = 5;
        let mut steps = 0;
        while self.accumulator >= FIXED_DT && steps < MAX_FIXED_STEPS {
            ctx.is_fixed_update = true;
            *fixed_update_count += 1;
            for plugin in plugins.iter_mut() {
                plugin.pre_fixed_update(ctx, FIXED_DT);
            }
            if let Some(global) = &mut self.global {
                global.dispatch_fixed_update(ctx, base, FIXED_DT);
            }
            object.dispatch_fixed_update(ctx, base, FIXED_DT);
            for plugin in plugins.iter_mut().rev() {
                plugin.post_fixed_update(ctx, FIXED_DT);
            }
            self.accumulator -= FIXED_DT;
            steps += 1;
            ctx.is_fixed_update = false;
        }
        if self.accumulator >= FIXED_DT {
            self.accumulator = 0.0;
        }

        for plugin in plugins.iter_mut() {
            plugin.pre_update(ctx, delta_time);
        }
        if let Some(global) = &mut self.global {
            global.dispatch_update(ctx, base, delta_time);
        }
        object.dispatch_update(ctx, base, delta_time);
        for plugin in plugins.iter_mut().rev() {
            plugin.post_update(ctx, delta_time);
        }

        for plugin in plugins.iter_mut() {
            plugin.pre_late_update(ctx, delta_time);
        }
        if let Some(global) = &mut self.global {
            global.dispatch_late_update(ctx, base, delta_time);
        }
        object.dispatch_late_update(ctx, base, delta_time);
        for plugin in plugins.iter_mut().rev() {
            plugin.post_late_update(ctx, delta_time);
        }

        let blending = self.accumulator / FIXED_DT;
        blending
    }
    pub fn render(
        &mut self,
        ctx: &mut Context,
        plugins: &mut PluginManager,
        base: &Base,
        blending: f32,
    ) {
        let Some(object) = self.scenes.last_mut() else {
            self.quit();
            return;
        };
        for plugin in plugins.iter_mut() {
            plugin.begin_draw(ctx, blending);
        }
        if let Some(global) = &mut self.global {
            global.dispatch_draw(ctx, base, blending);
        }
        object.get_dispatch().dispatch_draw(ctx, base, blending);
        for plugin in plugins.iter_mut() {
            plugin.end_draw(ctx, blending);
        }
    }
    pub fn quit(&mut self) {
        self.is_running = false
    }
}
