use crate::SourceSampler;

pub struct SilenceSource;

impl<const N: usize> SourceSampler<N> for SilenceSource {
    fn get_samples(&mut self, _sample_rate: f32, samples: &mut [f32]) {
        for sample in samples.iter_mut() {
            *sample = 0.0;
        }
    }
}
