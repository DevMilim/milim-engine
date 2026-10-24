use std::any::Any;

use crate::{
    core::{Base, Context, CoreSystems, Runner},
    events::EventManager,
    math::Vector2,
    object::{EmptyGlobals, GameObjectDispatch},
    plugin::{Plugin, PluginManager},
    resources::Resources,
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
    pub state: Resources,

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
            state: Resources::new(),
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
    pub fn add_plugin(mut self, mut plugin: impl Plugin) -> Self {
        plugin.build(&mut ());
        self.plugins.add_plugin(plugin);
        self
    }

    pub fn run_step(&mut self, delta: f32) -> bool {
        self.update_frame_count += 1;

        let mut ctx = Context::new(&mut self.systems, &mut self.state, &mut self.events);

        let blending = self.world.update(
            &mut ctx,
            &mut self.plugins,
            &self.base,
            &mut self.fixed_frame_count,
            delta,
        );

        const MAX_EVENT_ROUNDS: u32 = 10;
        let mut last_broadcast_version = 0;

        for round in 0..MAX_EVENT_ROUNDS {
            let mailboxes_pending = !ctx.event.mailboxes.is_empty();
            let broadcasts_pending = ctx.event.broadcast_version != last_broadcast_version;

            if !mailboxes_pending && !broadcasts_pending {
                break;
            }
            last_broadcast_version = ctx.event.broadcast_version;

            if let Some(global) = &mut self.world.global {
                global.dispatch_events(&mut ctx);
            }
            let Some(scene) = self.world.last_scene() else {
                break;
            };
            scene.dispatch_events(&mut ctx);

            ctx.event.prune_dead_mailboxes(&ctx.system.live_ids);

            let still_pending = !ctx.event.mailboxes.is_empty()
                || ctx.event.broadcast_version != last_broadcast_version;

            if round == MAX_EVENT_ROUNDS - 1 && still_pending {
                eprintln!("limite de rounds de evento atingido, possível loop de eventos");
            }
        }

        ctx.event.clear_broadcast_log();

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
        if self.world.is_running {
            self.world
                .render(&mut ctx, &mut self.plugins, &self.base, blending);
        }
        self.world.is_running
    }
    pub fn shutdown(&mut self) {
        let mut ctx = Context::new(&mut self.systems, &mut self.state, &mut self.events);
        self.world.clear_scenes(&mut ctx);
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
