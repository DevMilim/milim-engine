use std::{
    any::{Any, TypeId},
    sync::Arc,
};

use crate::{
    core::{AppCommands, CoreApi, CoreSystems, EngineApi, EventApi, Id, SceneApi, WorldApi},
    events::{EventManager, GlobalEvent, SpawnEvent},
    object::GameObject,
    resources::Resources,
    world::Scene,
};

pub struct Context<'a> {
    pub(crate) system: &'a mut CoreSystems,
    pub(crate) resources: &'a mut Resources,
    pub(crate) event: &'a mut EventManager,
}

impl<'a> Context<'a> {
    pub fn new(
        system: &'a mut CoreSystems,
        resources: &'a mut Resources,
        event: &'a mut EventManager,
    ) -> Self {
        Self {
            system,
            resources,
            event,
        }
    }
}

impl<'a> EngineApi for Context<'a> {}

impl<'a> WorldApi for Context<'a> {
    fn spawn<T: GameObject + Send + 'static>(&mut self, obj: T) {
        self.emit(SpawnEvent::new(obj));
    }
    fn register_alive(&mut self, id: Id) {
        self.system.live_ids.insert(id);
    }
    fn unregister_alive(&mut self, id: Id) {
        self.system.live_ids.remove(&id);
    }
    fn destroy(&mut self, id: Id) {
        self.unregister_alive(id);
    }
}
impl<'a> CoreApi for Context<'a> {
    fn register_service<T: 'static>(&mut self, id: Id) {
        let type_id = TypeId::of::<T>();
        self.system.service_register.insert(type_id, id);
    }

    fn service_id<T: 'static>(&self) -> Option<Id> {
        self.system
            .service_register
            .get(&TypeId::of::<T>())
            .copied()
    }

    fn set_resource<T: 'static>(&mut self, value: T) {
        self.resources.set::<T>(value, false);
    }

    fn get_resource<T: 'static>(&self) -> Option<&T> {
        self.resources.get::<T>()
    }

    fn get2_resources_mut<T1: 'static, T2: 'static>(&mut self) -> Option<(&mut T1, &mut T2)> {
        self.resources.get2_mut::<T1, T2>()
    }
    fn get3_resources_mut<T1: 'static, T2: 'static, T3: 'static>(
        &mut self,
    ) -> Option<(&mut T1, &mut T2, &mut T3)> {
        self.resources.get3_mut::<T1, T2, T3>()
    }

    fn get_resource_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.resources.get_mut::<T>()
    }

    fn remove_resource<T: 'static>(&mut self) {
        self.resources.remove::<T>();
    }
}
impl<'a> EventApi for Context<'a> {
    /// Utilizado para enviar uma mensagem endereçada para um GameObject especifico
    /// A mensagem tem que ser do mesmo tipo que o definido em type Message = T;
    fn send<T: Send + 'static>(&mut self, id: Id, message: T) {
        let event = GlobalEvent::Targeted(id, Box::new(message));
        self.event.insert_global_event(event);
    }
    /// Utilizado para emitir um evento global que sera recebido por todos os GameObjects que definiram um #[game(subscribe(metodo: Tipo))]
    fn emit<T: Send + Sync + 'static>(&mut self, event: T) {
        self.event.insert_broadcast_event(Arc::new(event));
    }
    /// Envia um evento similar a mensagem mas que pode ser de qualquer tipo, geralmente utilizado para comunicação de Componente para GameObject
    fn emit_targeted<T: Send + 'static>(&mut self, id: Id, event: T) {
        let event = GlobalEvent::Targeted(id, Box::new(event));
        self.event.insert_global_event(event);
    }

    fn send_boxed_any(&mut self, id: Id, message: Box<dyn Any + Send + 'static>) {
        let event = GlobalEvent::Targeted(id, message);
        self.event.insert_global_event(event);
    }

    fn send_service<T: 'static, E: Send + 'static>(&mut self, event: E) {
        if let Some(id) = self.service_id::<T>() {
            self.send(id, event);
        }
    }

    fn take_mailbox(&mut self, id: Id) -> Option<Vec<GlobalEvent>> {
        self.event.take_mailbox(id)
    }

    fn register_subscriptions(&mut self, id: Id, types: &[TypeId]) {
        self.event.register_subscriptions(id, types);
    }

    fn unregister_subscriptions(&mut self, id: Id, types: &[TypeId]) {
        self.event.unregister_subscriptions(id, types);
    }
    fn poll_broadcasts(
        &mut self,
        id: Id,
        type_id: TypeId,
    ) -> Vec<Arc<dyn Any + Send + Sync + 'static>> {
        self.event.poll_broadcasts(id, type_id)
    }
    fn recycle_broadcast_buffer(&mut self, buf: Vec<Arc<dyn Any + Send + Sync + 'static>>) {
        self.event.recycle_broadcast_buffer(buf);
    }
}

impl<'a> SceneApi for Context<'a> {
    fn push_scene<T: Scene + 'static>(&mut self, scene: T) {
        self.event
            .aplication_commands
            .push_back(AppCommands::PushScene(Box::new(scene)));
    }

    fn change_scene<T: Scene + 'static>(&mut self, scene: T) {
        self.event
            .aplication_commands
            .push_back(AppCommands::ChangeScene(Box::new(scene)));
    }

    fn pop_scene(&mut self) {
        self.event
            .aplication_commands
            .push_back(AppCommands::PopScene);
    }

    fn clear_scene(&mut self) {
        self.event
            .aplication_commands
            .push_back(AppCommands::ClearScenes);
    }
}
