use std::any::TypeId;

use std::any::Any;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

use rustc_hash::{FxHashMap, FxHashSet};

use crate::core::AppCommands;
use crate::object::GameObject;
use crate::object::Id;

pub type CallbackEvent = Box<dyn Fn() -> Box<dyn Any + Send + 'static>>;

#[derive(Debug)]
pub enum GlobalEvent {
    Targeted(Id, Box<dyn Any + Send + 'static>),
    Broadcast(Arc<dyn Any + Send + Sync + 'static>),
}

pub struct EventManager {
    pub mailboxes: FxHashMap<Id, Vec<GlobalEvent>>,
    pub broadcasts: FxHashMap<TypeId, Vec<Arc<dyn Any + Send + Sync + 'static>>>,
    pub broadcast_cursors: FxHashMap<(Id, TypeId), usize>,
    pub subscribers: FxHashMap<TypeId, FxHashSet<Id>>,

    pub broadcast_version: u64,
    pub broadcast_scratch_pool: Vec<Vec<Arc<dyn Any + Send + Sync + 'static>>>,
    pub dispatched_version: u64,
    pub connected: FxHashSet<Id>,
    pub aplication_commands: VecDeque<AppCommands>,
}

impl Default for EventManager {
    fn default() -> Self {
        Self {
            mailboxes: FxHashMap::default(),
            broadcasts: FxHashMap::default(),
            broadcast_cursors: FxHashMap::default(),
            subscribers: FxHashMap::default(),
            broadcast_version: 0,
            broadcast_scratch_pool: Vec::new(),
            aplication_commands: VecDeque::new(),
            dispatched_version: 0,
            connected: FxHashSet::default(),
        }
    }
}

impl EventManager {
    pub fn has_pending(&self) -> bool {
        !self.mailboxes.is_empty() || self.broadcast_version != self.dispatched_version
    }

    pub fn mark_dispatched(&mut self) {
        self.dispatched_version = self.broadcast_version;
    }

    pub fn forget(&mut self, id: Id) {
        self.mailboxes.remove(&id);
        self.connected.remove(&id);
    }

    pub fn end_frame(&mut self) {
        self.mailboxes.clear();
        self.clear_broadcast_log();
        self.dispatched_version = self.broadcast_version;
    }

    pub fn insert_global_event(&mut self, event: GlobalEvent) {
        match event {
            GlobalEvent::Broadcast(payload) => self.insert_broadcast_event(payload),
            GlobalEvent::Targeted(id, payload) => {
                if !self.connected.contains(&id) {
                    debug_assert!(false, "evento Targeted para id sem #[connect]");
                    return;
                }
                self.mailboxes
                    .entry(id)
                    .or_default()
                    .push(GlobalEvent::Targeted(id, payload));
            }
        }
    }
    pub fn take_mailbox(&mut self, id: Id) -> Option<Vec<GlobalEvent>> {
        self.mailboxes.remove(&id)
    }

    pub fn register_subscriptions(&mut self, id: Id, types: &[TypeId]) {
        for &type_id in types {
            self.subscribers.entry(type_id).or_default().insert(id);
        }
    }

    pub fn prune_dead(&mut self, live_ids: &FxHashSet<Id>) {
        self.mailboxes.retain(|id, _| live_ids.contains(id));
        self.connected.retain(|id| live_ids.contains(id));
        self.broadcast_cursors
            .retain(|(id, _), _| live_ids.contains(id));
    }

    pub fn unregister_subscriptions(&mut self, id: Id, types: &[TypeId]) {
        for &type_id in types {
            if let Some(ids) = self.subscribers.get_mut(&type_id) {
                ids.remove(&id);
                if ids.is_empty() {
                    self.subscribers.remove(&type_id);
                }
            }
            self.broadcast_cursors.remove(&(id, type_id));
        }
    }

    pub fn poll_broadcasts(
        &mut self,
        id: Id,
        type_id: TypeId,
    ) -> Vec<Arc<dyn Any + Send + Sync + 'static>> {
        let mut buf = self.broadcast_scratch_pool.pop().unwrap_or_default();

        let Some(log) = self.broadcasts.get(&type_id) else {
            return buf;
        };

        let cursor = self.broadcast_cursors.entry((id, type_id)).or_insert(0);
        let start = (*cursor).min(log.len());
        *cursor = log.len();

        if start < log.len() {
            buf.extend(log[start..].iter().cloned());
        }

        buf
    }

    pub fn recycle_broadcast_buffer(&mut self, mut buf: Vec<Arc<dyn Any + Send + Sync + 'static>>) {
        buf.clear();
        self.broadcast_scratch_pool.push(buf);
    }

    pub fn insert_broadcast_event(&mut self, event: Arc<dyn Any + Send + Sync + 'static>) {
        let type_id = (*event).type_id();

        if !self.subscribers.contains_key(&type_id) {
            return;
        }

        self.broadcasts.entry(type_id).or_default().push(event);
        self.broadcast_version = self.broadcast_version.wrapping_add(1);
    }

    pub fn clear_broadcast_log(&mut self) {
        self.broadcasts.clear();
        self.broadcast_cursors.clear();
    }
}

/// Evento utilizado para Spawn
/// Exemplo:
/// ```
/// #[derive(GameObject)]
/// #[game(subscribe(SpawnEvent<Bullet> => spawn_bullet))]
/// pub struct MainScene {
///     #[base]
///     base: Base,
///     #[object]
///     bullets: Pool<Bullet>,
/// }
/// impl MainScene {
///     fn spawn_bullet(&mut self, _ctx: &mut impl EngineApi, event: &SpawnEvent<Bullet>) {
///         self.bullets.spawn(event.take().expect("Erro ao spawnar bullet"));
///     }
/// }
///
/// ```
pub struct SpawnEvent<T> {
    payload: Mutex<Option<T>>,
}

impl<T: GameObject> SpawnEvent<T> {
    pub fn new(obj: T) -> Self {
        Self {
            payload: Mutex::new(Some(obj)),
        }
    }
    pub fn take(&self) -> Option<T> {
        self.payload.lock().ok()?.take()
    }
}
