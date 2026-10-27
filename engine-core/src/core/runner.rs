use crate::{core::App, object::GameObjectDispatch, world::Scene};

pub trait Runner {
    type Err;
    fn run<S, P>(self, app: App<S, P>) -> Result<(), Self::Err>
    where
        S: Scene + 'static,
        P: GameObjectDispatch;
}
