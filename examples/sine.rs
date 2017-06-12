extern crate waves;

use waves::*;

wave!(SineWave);



pub const TAU : f64 = std::f64::consts::PI * 2.0;


struct SineWave();
impl Wave for SineWave {
    type Input = ();
    type Output = f32;


    fn new() -> (Self, WaveDescription) { // .... and a param definition
        (
            SineWave {},
            WaveDescription {
                name : "SineWave".into(),
                vendor : "Waves".into(),
                unique_id: 9999631,
                category: plugin::Category::Effect,
            }
        )
    }

    fn process(&mut self, time:TimeSpan, input: &[()]) -> Vec<f32> {
        let freq = 440.0;
        time.time_iter().map(|t| {
            (t * freq * TAU).sin() as f32 
        }).collect()
    }   
}
