use std::{
    collections::HashMap,
    sync::{atomic::AtomicU32, Arc}, mem::ManuallyDrop,
};

use anyhow::{Context, Result};
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Device, SupportedStreamConfig,
};
use parking_lot::Mutex;

use crate::{Channel, Pickup, Source};

pub struct Engine<const N: usize> {
    pub channels: Mutex<HashMap<u32, Channel>>,
    pub sounds: Mutex<Vec<Source<N>>>,
    pub pickups: Mutex<Vec<Pickup<N>>>,
    pub master_volume: AtomicU32,
}

impl<const N: usize> Engine<N> {
    pub fn new_default() -> Result<Arc<Self>> {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .context("no output device available")?;

        let mut supported_configs_range = device
            .supported_output_configs()
            .context("error while querying configs")?;
        let supported_config = supported_configs_range
            .next()
            .context("no supported config?!")?
            .with_max_sample_rate();

        Self::new(device, supported_config)
    }

    pub fn new(device: Device, config: SupportedStreamConfig) -> Result<Arc<Self>> {
        let engine = Arc::new(Self {
            channels: Mutex::new(HashMap::new()),
            sounds: Mutex::new(Vec::new()),
            pickups: Mutex::new(Vec::new()),
            master_volume: AtomicU32::new(u32::MAX),
        });

        let this = engine.clone();
        let channels = config.channels() as usize;
        let sample_rate = config.sample_rate();
        let stream = device.build_output_stream(
            &config.into(),
            move |data: &mut [f32], _info: &cpal::OutputCallbackInfo| {
                let mut pickups = this.pickups.lock();
                for samples in data.chunks_mut(channels) {
                    for pickup in 0..channels {
                        if pickup >= pickups.len() {
                            break;
                        }

                        samples[pickup] =
                            pickups[pickup].sample(sample_rate.0 as f32, &this.sounds.lock());
                    }
                }
            },
            move |err| eprintln!("an error occurred on stream: {}", err),
            None,
        )?;
        let stream = ManuallyDrop::new(stream);
        stream.play()?;

        Ok(engine)
    }

    // pub fn run(&self) -> Result<()> {
    //     self.stream.play()?;
    //     Ok(())
    // }

    pub fn add_source(&self, source: Source<N>) {
        self.sounds.lock().push(source);
    }

    pub fn add_pickup(&self, pickup: Pickup<N>) {
        self.pickups.lock().push(pickup);
    }
}
