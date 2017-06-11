

extern crate waves;

use waves::*;

wave!(SineWave);

use std::f64::consts::PI;
pub const TAU : f64 = PI * 2.0;

struct SineWave();
impl Wave for SineWave {
    type Input = ();
    type Output = f32;

    fn new() -> Self { // .... and a param definition
        SineWave {}
    }

    fn process(time:TimeSpan) -> Vec<f32> {
        let freq = 440.0;
        time.time_iter().map(|t| {
            (t * freq * TAU).sin() as f32 
        }).collect()
    }   
}
