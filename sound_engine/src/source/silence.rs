use crate::SourceSampler;

pub struct SilenceSource;

impl SourceSampler for SilenceSource {
    fn sample(&mut self, _sample_rate: f32) -> f32 {
        0.0
    }
}
