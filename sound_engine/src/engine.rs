use std::{collections::HashMap, sync::atomic::AtomicU32};

use anyhow::{Context, Result};
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Device, Stream, SupportedStreamConfig,
};
use parking_lot::Mutex;

use crate::{Channel, Pickup, Source};

pub struct Engine<const N: usize> {
    pub channels: Mutex<HashMap<u32, Channel>>,
    pub sounds: Mutex<Vec<Source<N>>>,
    pub pickups: Mutex<Vec<Pickup<N>>>,
    pub master_volume: AtomicU32,

    stream: Stream,
}

impl<const N: usize> Engine<N> {
    pub fn new_default() -> Result<Self> {
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

    pub fn new(device: Device, config: SupportedStreamConfig) -> Result<Self> {
        let channels = config.channels() as usize;
        let stream = device.build_output_stream(
            &config.into(),
            move |data: &mut [f32], _info: &cpal::OutputCallbackInfo| {
                let samples = data.len() / channels;
                for channel in 0..channels {
                    todo!()
                }
            },
            move |err| eprintln!("an error occurred on stream: {}", err),
            None,
        )?;

        Ok(Self {
            channels: Mutex::new(HashMap::new()),
            sounds: Mutex::new(Vec::new()),
            pickups: Mutex::new(Vec::new()),
            master_volume: AtomicU32::new(u32::MAX),
            stream,
        })
    }

    pub fn run(&self) -> Result<()> {
        self.stream.play()?;
        Ok(())
    }

    pub fn add_source(&self, source: Source<N>) {
        self.sounds.lock().push(source);
    }
}
