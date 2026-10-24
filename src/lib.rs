pub mod core;
pub mod events;
pub mod math;
pub mod object;
pub mod plugin;
pub mod resources;
pub mod world;

pub use macros::*;

pub mod prelude {
    pub use crate::core::*;
    pub use crate::events::*;
    pub use crate::math::*;
    pub use crate::object::*;
    pub use crate::plugin::*;
    pub use crate::resources::*;
    pub use crate::world::*;
    pub use macros::*;
}
