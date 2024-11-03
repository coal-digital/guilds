pub mod consts;
pub mod error;
pub mod instruction;
pub mod state;
pub mod sdk;

pub mod prelude {
    pub use crate::consts::*;
    pub use crate::error::*;
    pub use crate::instruction::*;
    pub use crate::state::*;
    pub use crate::sdk::*;
}

use steel::*;

declare_id!("E5nw38V5Br5wf4x2aVM5TqDQhYeYEpKS7CQxG7P3B6KR");