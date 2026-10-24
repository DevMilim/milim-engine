use std::ops::{Index, IndexMut};

use crate::{
    core::EMPTY_BASE,
    object::{Base, GameObject, GameObjectBase, GameObjectDispatch},
};

const EMPTY: u32 = u32::MAX;

#[derive(Clone)]
pub struct SparseSet<V> {
    sparse: Vec<u32>,
    dense_ids: Vec<u32>,
    values: Vec<V>,
}

impl<T> Default for SparseSet<T> {
    fn default() -> Self {
        Self {
            sparse: Default::default(),
            dense_ids: Default::default(),
            values: Default::default(),
        }
    }
}

impl<V> SparseSet<V> {
    pub fn new() -> Self {
        Self {
            sparse: Vec::new(),
            dense_ids: Vec::new(),
            values: Vec::new(),
        }
    }

    #[inline]
    fn slot(&self, id: u32) -> Option<usize> {
        match self.sparse.get(id as usize) {
            Some(&s) if s != EMPTY => Some(s as usize),
            _ => None,
        }
    }
    #[inline]
    pub fn contains(&self, id: u32) -> bool {
        self.slot(id).is_some()
    }
    #[inline]
    pub fn id_at(&self, index: usize) -> u32 {
        self.dense_ids[index]
    }
    #[inline]
    pub fn at_mut(&mut self, index: usize) -> &mut V {
        &mut self.values[index]
    }

    pub fn insert(&mut self, id: u32, value: V) -> Option<V> {
        if let Some(s) = self.slot(id) {
            return Some(std::mem::replace(&mut self.values[s], value));
        }
        let i = id as usize;
        if i >= self.sparse.len() {
            self.sparse.resize(i + 1, EMPTY);
        }
        self.sparse[i] = self.values.len() as u32;
        self.dense_ids.push(id);
        self.values.push(value);
        None
    }

    pub fn remove(&mut self, id: u32) -> Option<V> {
        let s = self.slot(id)?;
        self.sparse[id as usize] = EMPTY;
        let value = self.values.swap_remove(s);
        self.dense_ids.swap_remove(s);
        if s < self.values.len() {
            let moved = self.dense_ids[s];
            self.sparse[moved as usize] = s as u32;
        }
        Some(value)
    }
    pub fn get_many_mut(&mut self, ids: impl IntoIterator<Item = u32>) -> Option<Vec<&mut V>> {
        let slots: Vec<usize> = ids.into_iter().filter_map(|id| self.slot(id)).collect();

        let mut sorted = slots.clone();
        sorted.sort_unstable();
        if sorted.windows(2).any(|w| w[0] == w[1]) {
            return None;
        }

        let base = self.values.as_mut_ptr();
        Some(
            slots
                .into_iter()
                .map(|s| unsafe { &mut *base.add(s) })
                .collect(),
        )
    }

    #[inline]
    pub fn get(&self, id: u32) -> Option<&V> {
        self.slot(id).map(|s| &self.values[s])
    }

    #[inline]
    pub fn get_mut(&mut self, id: u32) -> Option<&mut V> {
        self.slot(id).map(|s| &mut self.values[s])
    }

    pub fn get2_mut(&mut self, a: u32, b: u32) -> Option<(&mut V, &mut V)> {
        let (ia, ib) = (self.slot(a)?, self.slot(b)?);
        let [x, y] = self.values.get_disjoint_mut([ia, ib]).ok()?;
        Some((x, y))
    }

    pub fn iter(&self) -> impl Iterator<Item = (u32, &V)> {
        self.dense_ids.iter().copied().zip(self.values.iter())
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (u32, &mut V)> {
        self.dense_ids.iter().copied().zip(self.values.iter_mut())
    }
    pub fn values(&self) -> &[V] {
        &self.values
    }
    pub fn values_mut(&mut self) -> &mut [V] {
        &mut self.values
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
}

pub struct Pool<T: GameObject + GameObjectDispatch> {
    pub(crate) items: Vec<T>,
}

impl<T: GameObject + GameObjectDispatch> Default for Pool<T> {
    fn default() -> Self {
        Self { items: Vec::new() }
    }
}

impl<T: GameObject + GameObjectDispatch> Index<usize> for Pool<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.items[index]
    }
}
impl<T: GameObject + GameObjectDispatch> IndexMut<usize> for Pool<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.items[index]
    }
}

impl<T: GameObject + GameObjectDispatch> Pool<T> {
    pub fn spawn(&mut self, object: T) {
        self.items.push(object);
    }
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.items.iter()
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.items.iter_mut()
    }
    pub fn queue_free_all(&mut self) {
        for item in &mut self.items {
            item.base_mut().queue_free();
        }
    }
    pub fn len(&self) -> usize {
        self.items.len()
    }
    pub fn capacity(&self) -> usize {
        self.items.capacity()
    }
}

impl<T: GameObject + GameObjectDispatch> GameObject for Pool<T> {}
impl<T: GameObjectBase + GameObject + GameObjectDispatch> GameObjectBase for Pool<T> {
    fn base(&self) -> &Base {
        &EMPTY_BASE
    }

    fn base_mut(&mut self) -> &mut Base {
        panic!("Tentativa invalida de acessar base_mut em um Vec<GameObject>")
    }
}
