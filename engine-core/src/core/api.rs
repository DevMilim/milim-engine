use std::{
    any::{Any, TypeId},
    sync::Arc,
};

use crate::{
    events::GlobalEvent,
    object::{GameObject, Id},
    world::Scene,
};

pub trait EngineApi: CoreApi + WorldApi + SceneApi + EventApi {}

pub trait CoreApi {
    fn register_service<T: 'static>(&mut self, id: Id);
    fn service_id<T: 'static>(&self) -> Option<Id>;
    fn set_resource<T: 'static>(&mut self, value: T);
    fn get_resource<T: 'static>(&self) -> Option<&T>;
    fn get_resource_mut<T: 'static>(&mut self) -> Option<&mut T>;
    fn require_resource_mut<T: 'static>(&mut self) -> &mut T;
    fn get2_resources_mut<T1: 'static, T2: 'static>(&mut self) -> Option<(&mut T1, &mut T2)>;
    fn get3_resources_mut<T1: 'static, T2: 'static, T3: 'static>(
        &mut self,
    ) -> Option<(&mut T1, &mut T2, &mut T3)>;
    fn remove_resource<T: 'static>(&mut self);
}

pub trait WorldApi {
    fn spawn<T: GameObject + Send + 'static>(&mut self, obj: T);
    fn start_object(&mut self, id: Id);
    fn destroy_object(&mut self, id: Id);
}

pub trait EventApi {
    fn send<T: Send + 'static>(&mut self, id: Id, message: T);
    fn send_boxed_any(&mut self, id: Id, message: Box<dyn Any + Send + 'static>);
    fn emit<T: Send + Sync + 'static>(&mut self, event: T);
    fn emit_targeted<T: Send + 'static>(&mut self, id: Id, event: T);
    fn send_service<T: 'static, E: Send + 'static>(&mut self, event: E);
    fn take_mailbox(&mut self, id: Id) -> Option<Vec<GlobalEvent>>;
    fn register_subscriptions(&mut self, id: Id, types: &[TypeId]);
    fn unregister_subscriptions(&mut self, id: Id, types: &[TypeId]);
    fn register_connect(&mut self, id: Id);
    fn poll_broadcasts(
        &mut self,
        id: Id,
        type_id: TypeId,
    ) -> Vec<Arc<dyn Any + Send + Sync + 'static>>;
    fn recycle_broadcast_buffer(&mut self, buf: Vec<Arc<dyn Any + Send + Sync + 'static>>);
}
pub trait SceneApi {
    fn push_scene<T: Scene + 'static>(&mut self, scene: T);
    fn change_scene<T: Scene + 'static>(&mut self, scene: T);
    fn pop_scene(&mut self);
    fn clear_scene(&mut self);
}
