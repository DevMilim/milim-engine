use std::any::{Any, TypeId};

use rustc_hash::FxHashMap;

#[derive(Debug)]
pub struct Security {
    resource: Box<dyn Any>,
    protected: bool,
}

#[derive(Debug)]
pub struct Resources {
    data: FxHashMap<TypeId, Security>,
}

impl Resources {
    pub fn new() -> Self {
        Self {
            data: FxHashMap::default(),
        }
    }
    pub fn set<T: 'static>(&mut self, val: T, protected: bool) {
        let type_id = TypeId::of::<T>();

        if let Some(security) = self.data.get(&type_id) {
            if security.protected {
                return;
            }
        }

        self.data.insert(
            type_id,
            Security {
                resource: Box::new(val),
                protected,
            },
        );
    }
    pub fn get<T: 'static>(&self) -> Option<&T> {
        self.data
            .get(&TypeId::of::<T>())
            .and_then(|boxed| boxed.resource.downcast_ref())
    }
    pub fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.data
            .get_mut(&TypeId::of::<T>())
            .and_then(|boxed| boxed.resource.downcast_mut())
    }
    pub fn require_mut<T: 'static>(&mut self) -> &mut T {
        if let Some(res) = self.get_mut::<T>() {
            return res;
        } else {
            panic!("Engine Error: Required resource not found")
        }
    }

    pub fn remove<T: 'static>(&mut self) -> Option<T> {
        let type_id = TypeId::of::<T>();

        if let Some(security) = self.data.get(&type_id) {
            if security.protected {
                return None;
            }
        }

        self.data
            .remove(&type_id)
            .and_then(|value| value.resource.downcast::<T>().ok())
            .map(|value| *value)
    }
    pub fn contains<T: 'static>(&self) -> bool {
        self.data.contains_key(&TypeId::of::<T>())
    }
    pub fn get2_mut<T1: 'static, T2: 'static>(&mut self) -> Option<(&mut T1, &mut T2)> {
        let (ia, ib) = (TypeId::of::<T1>(), TypeId::of::<T2>());

        if ia == ib {
            return None;
        }

        let [Some(x), Some(y)] = self.data.get_disjoint_mut([&ia, &ib]) else {
            return None;
        };

        Some((
            x.resource.downcast_mut::<T1>()?,
            y.resource.downcast_mut::<T2>()?,
        ))
    }
    pub fn get3_mut<T1: 'static, T2: 'static, T3: 'static>(
        &mut self,
    ) -> Option<(&mut T1, &mut T2, &mut T3)> {
        let (ia, ib, ic) = (TypeId::of::<T1>(), TypeId::of::<T2>(), TypeId::of::<T3>());

        if ia == ib || ia == ic || ib == ic {
            return None;
        }

        let [Some(x), Some(y), Some(z)] = self.data.get_disjoint_mut([&ia, &ib, &ic]) else {
            return None;
        };

        Some((
            x.resource.downcast_mut::<T1>()?,
            y.resource.downcast_mut::<T2>()?,
            z.resource.downcast_mut::<T3>()?,
        ))
    }
}
