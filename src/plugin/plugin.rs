use std::any::{Any, TypeId};

use rustc_hash::FxHashSet;

use crate::core::Context;

pub trait Plugin: Any + 'static {
    fn build(&mut self, _app: &mut ()) {}
    fn pre_fixed_update(&mut self, _ctx: &mut Context, _delta: f32) {}
    fn post_fixed_update(&mut self, _ctx: &mut Context, _delta: f32) {}
    fn pre_update(&mut self, _ctx: &mut Context, _delta: f32) {}
    fn post_update(&mut self, _ctx: &mut Context, _delta: f32) {}
    fn pre_late_update(&mut self, _ctx: &mut Context, _delta: f32) {}
    fn post_late_update(&mut self, _ctx: &mut Context, _delta: f32) {}
    fn begin_draw(&mut self, _ctx: &mut Context, _blending: f32) {}
    fn end_draw(&mut self, _ctx: &mut Context, _blending: f32) {}
    fn shutdown(&mut self, _ctx: &mut Context) {}
}

pub struct PluginManager {
    plugins: Vec<Box<dyn Plugin>>,
    registered: FxHashSet<TypeId>,
}
impl PluginManager {
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
            registered: FxHashSet::default(),
        }
    }
    pub fn add_plugin<T: Plugin>(&mut self, plugin: T) {
        let ty = TypeId::of::<T>();
        if self.registered.contains(&ty) {
            debug_assert!(
                false,
                "Plugin ja existe, so e possivel adicionar um por tipo"
            );
        } else {
            self.plugins.push(Box::new(plugin));
            self.registered.insert(ty);
        }
    }
    pub fn get_plugin<T: Plugin + Any + 'static>(&self) -> Option<&T> {
        self.plugins.iter().find_map(|p| {
            let any = p.as_ref() as &dyn Any;
            any.downcast_ref::<T>()
        })
    }
    pub fn get_plugin_mut<T: Plugin + Any + 'static>(&mut self) -> Option<&mut T> {
        self.plugins.iter_mut().find_map(|p| {
            let any = p.as_mut() as &mut dyn Any;
            any.downcast_mut::<T>()
        })
    }
    pub fn iter_mut(
        &mut self,
    ) -> impl Iterator<Item = &mut Box<dyn Plugin>> + DoubleEndedIterator + ExactSizeIterator {
        self.plugins.iter_mut()
    }
    pub fn remove(&mut self, index: usize) -> Box<dyn Plugin> {
        self.plugins.remove(index)
    }
    pub fn insert(&mut self, index: usize, plugin: Box<dyn Plugin>) {
        self.plugins.insert(index, plugin)
    }
    pub fn len(&self) -> usize {
        self.plugins.len()
    }
}
