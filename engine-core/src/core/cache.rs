use rustc_hash::FxHashMap;

use crate::core::Handler;

pub struct AssetCache<T> {
    assets: FxHashMap<usize, T>,
    path_map: FxHashMap<String, usize>,
    next_id: usize,
    id_to_path: FxHashMap<usize, String>,
}

impl<T> AssetCache<T> {
    pub fn new() -> Self {
        Self {
            assets: FxHashMap::default(),
            path_map: FxHashMap::default(),
            next_id: 0,
            id_to_path: FxHashMap::default(),
        }
    }
    /// Retorna um id para uso
    fn current_id(&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        id
    }
    /// Obtem o id de um asset pelo seu caminho
    pub fn get_id(&self, path: &str) -> Option<usize> {
        self.path_map.get(path).copied()
    }
    /// Retorna um asset usando handler
    pub fn get(&self, id: Handler<T>) -> Option<&T> {
        self.assets.get(&id.id)
    }
    /// Obtem um asset mutavel
    pub fn get_mut(&mut self, id: Handler<T>) -> Option<&mut T> {
        self.assets.get_mut(&id.id)
    }
    /// insere um asset
    pub fn insert(&mut self, path: &str, asset: T) -> Handler<T> {
        if let Some(asset_id) = self.path_map.get(path).copied() {
            return Handler::new(asset_id);
        }
        let asset_id = self.current_id();

        self.assets.insert(asset_id, asset);
        self.path_map.insert(path.to_string(), asset_id);
        self.id_to_path.insert(asset_id, path.to_string());

        Handler::new(asset_id)
    }
    pub fn clear(&mut self) {
        self.assets.clear();
        self.path_map.clear();
        self.id_to_path.clear();
    }
    pub fn remove(&mut self, asset_id: Handler<T>) -> Option<T> {
        let asset = self.assets.remove(&asset_id.id);
        if let Some(path) = self.id_to_path.remove(&asset_id.id) {
            self.path_map.remove(&path);
        }
        asset
    }
}

impl<T> Default for AssetCache<T> {
    fn default() -> Self {
        Self::new()
    }
}
