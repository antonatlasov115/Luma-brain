pub mod gnw;
pub mod iit;
pub mod spatial_brain;

pub use gnw::{GlobalWorkspace, Information, InformationSource};
pub use iit::{IITConsciousness, NetworkState};
pub use spatial_brain::{SpatialBrain, BrainRegion, RegionState, Connection};
