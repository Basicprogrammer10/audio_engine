pub mod engine;
pub use engine::Engine;
pub mod pickup;
pub use pickup::Pickup;
pub mod source;
pub use source::{Source, SourceSampler};
pub mod consts;

pub struct Channel {
    pub id: u32,
    pub volume: f32,
}
