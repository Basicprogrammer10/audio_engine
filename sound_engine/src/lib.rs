use vector::Vector;

pub mod engine;
pub use engine::Engine;

pub mod source;
pub use source::{SourceSampler, Source};

pub struct Channel {
    pub id: u32,
    pub volume: f32,
}

pub struct Pickup<const N: usize> {
    pub id: u32,
    pub volume: f32,

    pub position: Vector<f32, N>,
    pub velocity: Vector<f32, N>,
}
