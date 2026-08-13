pub mod events;
pub mod shared;
pub mod stats;

pub mod prelude {
    pub use crate::components::shared::actions::containers::*;
    pub use crate::components::shared::actions::delete::*;
    pub use crate::components::shared::actions::edit::*;
    pub use crate::components::shared::actions::exec::*;
    pub use crate::components::shared::actions::follow::*;
    pub use crate::components::shared::actions::logs::*;
    pub use crate::components::shared::actions::namespaces::*;
    pub use crate::components::shared::actions::previous::*;
    pub use crate::components::shared::actions::prompt::*;
    pub use crate::components::shared::actions::save::*;
    pub use crate::components::shared::actions::scale::*;
    pub use crate::components::shared::actions::*;
    pub use crate::components::shared::card::card_circle::*;
    pub use crate::components::shared::card::card_string::*;
    pub use crate::components::shared::data::*;
    pub use crate::components::shared::dialog::apply_yaml::*;
    pub use crate::components::shared::table::*;
    pub use crate::components::shared::wrapper::*;
}
