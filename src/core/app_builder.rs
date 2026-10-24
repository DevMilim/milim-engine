use crate::{
    plugin::{Plugin, PluginManager},
    resources::Resources,
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
        self.resources.set(T::default());
    }

    pub fn set_resource<T: 'static>(&mut self, value: T) {
        self.resources.set(value);
    }

    pub fn get_resource<T: 'static>(&self) -> Option<&T> {
        self.resources.get::<T>()
    }

    pub fn get_resource_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.resources.get_mut::<T>()
    }

    pub fn remove_resource<T: 'static>(&mut self) -> Option<T> {
        self.resources.remove::<T>()
    }
    pub fn add_plugin(mut self, mut plugin: impl Plugin) -> Self {
        plugin.build(&mut AppBuilder::new(&mut self.resources, &mut self.plugins));
        self.plugins.add_plugin(plugin);
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
