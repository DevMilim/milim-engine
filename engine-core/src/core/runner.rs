use crate::{core::App, object::GameObjectDispatch, world::Scene};

pub trait Runner {
    fn run<S, P>(self, app: App<S, P>) -> Result<(), ()>
    where
        S: Scene + 'static,
        P: GameObjectDispatch;
}
