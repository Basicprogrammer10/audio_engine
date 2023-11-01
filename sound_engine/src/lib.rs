use std::collections::HashMap;

use vector::Vector;

pub struct Engine<const N: usize> {
    pub channels: HashMap<u32, Channel>,
    pub sounds: Vec<AudioInstance<N>>,
    pub pickups: Vec<Pickup<N>>,
    pub master_volume: f32,
}

pub struct Channel {
    pub id: u32,
    pub volume: f32,
}

pub struct AudioInstance<const N: usize> {
    pub id: u32,
    pub volume: f32,
    pub channel: u32,
    pub position: Vector<f32, N>,
}

pub struct Pickup<const N: usize> {
    pub id: u32,
    pub volume: f32,
    pub position: Vector<f32, N>,
}
