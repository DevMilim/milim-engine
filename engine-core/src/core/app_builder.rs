use crate::{
    plugin::{Plugin, PluginManager},
    resources::{Query, Resources},
};

pub struct AppBuilder<'a> {
    resources: &'a mut Resources,
    plugins: &'a mut PluginManager,
}

impl<'a> AppBuilder<'a> {
    pub fn new(resources: &'a mut Resources, plugins: &'a mut PluginManager) -> Self {
        Self { resources, plugins }
    }

    pub fn init_resource<T: Default + 'static>(&mut self) {
        self.resources.set(T::default(), true);
    }

    pub fn set_resource<T: 'static>(&mut self, value: T) {
        self.resources.set(value, true);
    }

    pub fn get_resource<T: 'static>(&self) -> Option<&T> {
        self.resources.get::<T>()
    }
    pub fn require_resource_mut<T: 'static>(&mut self) -> &mut T {
        self.resources.require_mut::<T>()
    }

    pub fn query_resources_mut<Q: Query>(&mut self) -> Option<Q::Refs<'_>> {
        self.resources.query_mut::<Q>()
    }
    pub fn require_query_mut<Q: Query>(&mut self) -> Q::Refs<'_> {
        self.resources.require_query_mut::<Q>()
    }

    pub fn get_resource_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.resources.get_mut::<T>()
    }

    pub fn remove_resource<T: 'static>(&mut self) -> Option<T> {
        self.resources.remove::<T>()
    }
    pub fn add_plugin<T: Plugin>(&mut self, mut plugin: T) -> &mut Self {
        if !self.plugins.reserve::<T>() {
            return self;
        }
        plugin.build(self);
        self.plugins.push_reserved(plugin);
        self
    }
    pub fn has_plugin<T: Plugin>(&self) -> bool {
        self.plugins.get_plugin::<T>().is_some()
    }

    pub fn get_plugin<T: Plugin>(&self) -> Option<&T> {
        self.plugins.get_plugin()
    }
    pub fn get_plugin_mut<T: Plugin>(&mut self) -> Option<&mut T> {
        self.plugins.get_plugin_mut()
    }
}
