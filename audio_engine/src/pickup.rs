use vector::Vector;

use crate::{consts::SOUND_SPEED, Source};

pub struct Pickup<const N: usize> {
    pub id: u32,
    pub volume: f32,

    pub position: Vector<f32, N>,
    pub velocity: Vector<f32, N>,
}

impl<const N: usize> Pickup<N> {
    pub fn sample(&mut self, sample_rate: f32, sounds: &[Source<N>]) -> f32 {
        let mut out = 0.0;

        for sound in sounds {
            let distance = (sound.position - self.position).magnitude();
            let delay = distance / SOUND_SPEED;
            let delay_samples = (delay * sample_rate) as usize;
            out += sound.source.lock().sample(sample_rate);
        }

        out
    }
}

impl<const N: usize> Pickup<N> {
    pub fn builder() -> Self {
        Default::default()
    }

    pub fn id(mut self, id: u32) -> Self {
        self.id = id;
        self
    }

    pub fn volume(mut self, volume: f32) -> Self {
        self.volume = volume;
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
}

impl<const N: usize> Default for Pickup<N> {
    fn default() -> Self {
        Self {
            id: 0,
            volume: 1.0,

            position: Vector::zero(),
            velocity: Vector::zero(),
        }
    }
}
