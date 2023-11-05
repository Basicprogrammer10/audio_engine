use sound_engine::{Engine, Source, source::SilenceSource};
use vector::vector;

fn main() {
    let engine = Engine::<2>::new_default().unwrap();
    engine.run().unwrap();

    engine.add_source(Source::builder().source(SilenceSource).position(vector!(1.0, 5.0)));

    ::std::thread::park();
}
