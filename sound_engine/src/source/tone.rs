use std::f32::consts::PI;

use crate::SourceSampler;

pub struct ToneSource {
    /// The index of the current sample.
    i: usize,
    /// The frequency of the tone (Hz).
    tone: f32,
    /// An optional duration for the tone in samples.
    /// Will just cut off the tone when the duration is reached.
    duration: Option<usize>,
}

impl ToneSource {
    /// Create a new tone with the given frequency.
    pub fn new(tone: f32) -> Self {
        Self {
            i: 0,
            tone,
            duration: None,
        }
    }

    /// Sets the duration of the tone in samples.
    pub fn duration(mut self, duration: usize) -> Self {
        self.duration = Some(duration);
        self
    }
}

impl SourceSampler for ToneSource {
    fn sample(&mut self, sample_rate: f32) -> f32 {
        self.i += 1;

        match self.duration {
            Some(i) if self.i > i => return 0.0,
            _ => {}
        };

        (self.i as f32 * self.tone * 2.0 * PI / sample_rate).sin()
    }
}
