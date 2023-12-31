use nd_vec::Vector;
use parking_lot::Mutex;

mod silence;
mod tone;
pub use silence::SilenceSource;
pub use tone::ToneSource;

pub struct Source<const N: usize> {
    pub id: u32,
    pub volume: f32,
    pub channel: u32,

    pub position: Vector<f32, N>,
    pub velocity: Vector<f32, N>,

    pub source: Mutex<Box<dyn SourceSampler + Send + Sync>>,
}

pub trait SourceSampler {
    fn sample(&mut self, sample_rate: f32) -> f32;
}

impl<const N: usize> Source<N> {
    pub fn builder() -> Self {
        Default::default()
    }
}

impl<const N: usize> Source<N> {
    pub fn id(mut self, id: u32) -> Self {
        self.id = id;
        self
    }

    pub fn volume(mut self, volume: f32) -> Self {
        self.volume = volume;
        self
    }

    pub fn channel(mut self, channel: u32) -> Self {
        self.channel = channel;
        self
    }

    pub fn position(mut self, position: Vector<f32, N>) -> Self {
        self.position = position;
        self
    }

    pub fn velocity(mut self, velocity: Vector<f32, N>) -> Self {
        self.velocity = velocity;
        self
    }

    pub fn source(mut self, source: impl SourceSampler + Send + Sync + 'static) -> Self {
        self.source = Mutex::new(Box::new(source));
        self
    }

    pub fn build(self) -> Source<N> {
        Source {
            id: self.id,
            volume: self.volume,
            channel: self.channel,

            position: self.position,
            velocity: self.velocity,

            source: self.source,
        }
    }
}

impl<const N: usize> Default for Source<N> {
    fn default() -> Self {
        Self {
            id: 0,
            volume: 1.0,
            channel: 0,
            position: Vector::zero(),
            velocity: Vector::zero(),
            source: Mutex::new(Box::new(SilenceSource)),
        }
    }
}
