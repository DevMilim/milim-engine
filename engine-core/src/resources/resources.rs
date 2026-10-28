use std::any::{Any, TypeId, type_name};

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
            panic!(
                "Engine Error: Required resource not found: {}",
                type_name::<T>()
            )
        }
    }
    pub fn require_query_mut<Q: Query>(&mut self) -> Q::Refs<'_> {
        match Q::get_many_mut(self) {
            Some(res) => res,
            None => panic!(
                "Engine Error: Required resource not found: {}",
                type_name::<Q::Refs<'_>>()
            ),
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
    pub fn query_mut<Q: Query>(&mut self) -> Option<Q::Refs<'_>> {
        Q::get_many_mut(self)
    }
}

pub trait Query {
    type Refs<'a>;
    fn get_many_mut(res: &mut Resources) -> Option<Self::Refs<'_>>;
}

macro_rules! impl_get_many {
    ($n:literal; $($T:ident),+) => {
        impl<$($T: 'static),+> Query for ($($T,)+) {
            type Refs<'a> = ($(&'a mut $T,)+);

            #[allow(non_snake_case)]
            fn get_many_mut(res: &mut Resources) -> Option<Self::Refs<'_>> {
                let ids: [TypeId; $n] = [$(TypeId::of::<$T>()),+];

                for i in 0..$n {
                    for j in (i + 1)..$n {
                        if ids[i] == ids[j] {
                            return None;
                        }
                    }
                }

                let [$(Some($T)),+] = res.data.get_disjoint_mut(ids.each_ref()) else {
                    return None;
                };

                Some(($($T.resource.downcast_mut::<$T>()?,)+))
            }
        }
    };
}

impl_get_many!(1; A);
impl_get_many!(2; A, B);
impl_get_many!(3; A, B, C);
impl_get_many!(4; A, B, C, D);
impl_get_many!(5; A, B, C, D, E);
impl_get_many!(6; A, B, C, D, E, F);
impl_get_many!(7; A, B, C, D, E, F, G);
impl_get_many!(8; A, B, C, D, E, F, G, H);
