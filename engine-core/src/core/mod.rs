mod api;
mod app;
mod app_builder;
mod cache;
mod context;
mod handler;
mod runner;

pub use api::*;
pub use app::*;
pub use app_builder::*;
pub use cache::*;
pub use context::*;
pub use core::*;
pub use handler::*;
pub use runner::*;

use rustc_hash::{FxHashMap, FxHashSet};
use std::{any::TypeId, sync::LazyLock};

use crate::object::{Base, Id};

pub static EMPTY_BASE: LazyLock<Base> = LazyLock::new(Base::default);

pub struct CoreSystems {
    pub service_register: FxHashMap<TypeId, Id>,
    pub live_ids: FxHashSet<Id>,
}

impl Default for CoreSystems {
    fn default() -> Self {
        Self {
            service_register: FxHashMap::default(),
            live_ids: FxHashSet::default(),
        }
    }
}
