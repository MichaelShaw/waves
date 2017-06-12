extern crate waves;

use waves::*;

wave!(SineWave);

pub const TAU : f64 = std::f64::consts::PI * 2.0;

struct SineWave();
impl Wave for SineWave {
    type Input = ();
    type Output = f32;


    fn new() -> Self { // .... and a param definition
        SineWave {}
    }

    fn process(&mut self, time:TimeSpan, input: &[()]) -> Vec<f32> {
        let freq = 440.0;
        time.time_iter().map(|t| {
            (t * freq * TAU).sin() as f32 
        }).collect()
    }   
}
