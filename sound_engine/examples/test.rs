use sound_engine::{source::ToneSource, Engine, Pickup, Source};
use vector::vector;

fn main() {
    let engine = Engine::<2>::new_default().unwrap();

    engine.add_pickup(Pickup::builder());
    engine.add_pickup(Pickup::builder());
    engine.add_source(
        Source::builder()
            .source(ToneSource::new(340.0))
            .position(vector!(1.0, 5.0)),
    );

    ::std::thread::park();
}
