use std::any::Any;

use crate::{
    core::{AppBuilder, Base, Context, CoreSystems, Runner},
    events::EventManager,
    math::Vector2,
    object::{EmptyGlobals, GameObjectDispatch},
    plugin::{Plugin, PluginManager},
    resources::{Query, Resources},
    world::{Scene, WorldState},
};

#[derive(Debug)]
pub enum AppCommands {
    ChangeScene(Box<dyn Any>),
    PushScene(Box<dyn Any>),
    ClearScenes,
    PopScene,
}

pub struct App<S: Scene + 'static, P: GameObjectDispatch = EmptyGlobals> {
    pub systems: CoreSystems,
    pub events: EventManager,
    pub world: WorldState<S, P>,
    pub resources: Resources,

    pub base: Base,
    pub camera_position: Vector2,
    pub update_frame_count: u64,
    pub fixed_frame_count: u64,
    pub plugins: PluginManager,
}

impl<S: Scene + 'static, P: GameObjectDispatch> App<S, P> {
    pub fn new(root_scene: S) -> Self {
        Self {
            systems: CoreSystems::default(),
            events: EventManager::default(),
            world: WorldState::new(root_scene),
            resources: Resources::new(),
            base: Base::default(),
            camera_position: Vector2::new(0.0, 0.0),
            update_frame_count: 0,
            fixed_frame_count: 0,
            plugins: PluginManager::new(),
        }
    }

    pub fn with_globals(mut self, global: P) -> Self {
        self.world.global = Some(global);
        self
    }
    pub fn add_plugin<T: Plugin>(&mut self, mut plugin: T) -> &mut Self {
        debug_assert!(
            !self.plugins.contains::<T>(),
            "Plugin ja existe, so e possivel adicionar um por tipo"
        );
        plugin.build(&mut AppBuilder::new(&mut self.resources, &mut self.plugins));
        self.plugins.add_plugin(plugin);
        self
    }
    pub fn get_plugin<T: Plugin + 'static>(&self) -> Option<&T> {
        self.plugins.get_plugin()
    }
    pub fn get_plugin_mut<T: Plugin + 'static>(&mut self) -> Option<&mut T> {
        self.plugins.get_plugin_mut()
    }

    pub fn set_resource<T: 'static>(&mut self, value: T) {
        self.resources.set::<T>(value, true);
    }

    pub fn get_resource<T: 'static>(&self) -> Option<&T> {
        self.resources.get::<T>()
    }
    pub fn require_resource_mut<T: 'static>(&mut self) -> &mut T {
        self.resources.require_mut::<T>()
    }

    pub fn get_resource_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.resources.get_mut::<T>()
    }

    pub fn query_resources_mut<Q: Query>(&mut self) -> Option<Q::Refs<'_>> {
        self.resources.query_mut::<Q>()
    }
    pub fn require_query_mut<Q: Query>(&mut self) -> Q::Refs<'_> {
        self.resources.require_query_mut::<Q>()
    }

    pub fn remove_resource<T: 'static>(&mut self) {
        self.resources.remove::<T>();
    }

    pub fn run_step(&mut self, delta: f32) -> bool {
        self.update_frame_count += 1;

        let mut ctx = Context::new(&mut self.systems, &mut self.resources, &mut self.events);

        for plugin in self.plugins.iter_mut() {
            plugin.start_frame(&mut ctx);
        }

        let blending = self.world.update(
            &mut ctx,
            &mut self.plugins,
            &self.base,
            &mut self.fixed_frame_count,
            delta,
        );

        Self::flush_events(&mut ctx, &mut self.world);

        self.world.destroy_pending_top(&mut ctx);

        while let Some(cmd) = ctx.event.aplication_commands.pop_front() {
            match cmd {
                AppCommands::ChangeScene(scene) => {
                    if let Ok(scene) = scene.downcast::<S>() {
                        self.world.change_scene(*scene, &mut ctx);
                    } else {
                        debug_assert!(false, "AppCommands com tipo de cena errado")
                    }
                }
                AppCommands::PushScene(scene) => {
                    if let Ok(scene) = scene.downcast::<S>() {
                        self.world.push_scene(*scene);
                    } else {
                        debug_assert!(false, "AppCommands com tipo de cena errado")
                    }
                }
                AppCommands::PopScene => {
                    self.world.pop_scene(&mut ctx);
                }
                AppCommands::ClearScenes => {
                    self.world.clear_scenes(&mut ctx);
                }
            }
        }
        self.world.reap_top();
        if self.world.scenes.is_empty() {
            self.world.quit();
        }
        Self::flush_events(&mut ctx, &mut self.world);
        ctx.event.end_frame();
        if self.world.is_running {
            self.world
                .render(&mut ctx, &mut self.plugins, &self.base, blending);
        }
        for plugin in self.plugins.iter_mut() {
            plugin.end_frame(&mut ctx);
        }
        self.world.is_running
    }
    fn flush_events(ctx: &mut Context, world: &mut WorldState<S, P>) {
        const MAX_EVENT_ROUNDS: u32 = 10;

        for round in 0..MAX_EVENT_ROUNDS {
            if !ctx.event.has_pending() {
                break;
            }
            ctx.event.mark_dispatched();

            if let Some(global) = &mut world.global {
                global.dispatch_events(ctx);
            }
            if let Some(scene) = world.last_scene() {
                scene.dispatch_events(ctx);
            }

            ctx.event.prune_dead(&ctx.system.live_ids);

            if round == MAX_EVENT_ROUNDS - 1 && ctx.event.has_pending() {
                eprintln!("limite de rounds de evento atingido, possível loop de eventos");
            }
        }
    }
    pub fn shutdown(&mut self) {
        let mut ctx = Context::new(&mut self.systems, &mut self.resources, &mut self.events);
        self.world.clear_scenes(&mut ctx);
        ctx.event.end_frame();
        if let Some(g) = &mut self.world.global {
            g.dispatch_destroy(&mut ctx);
        }
        for p in self.plugins.iter_mut().rev() {
            p.shutdown(&mut ctx);
        }
    }
    pub fn run(self, runner: impl Runner) -> Result<(), ()> {
        runner.run(self)
    }
}
