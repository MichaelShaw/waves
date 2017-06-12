extern crate waves;
#[macro_use] 
extern crate log;

use waves::*;
use std::cmp::max;

wave!(ReverbWave);


struct Reverb {
    delay: f64,
    gain: f32,
}

struct ReverbWave {
    reverbs: Vec<Reverb>,
    delay_channel : Vec<f32>,
    at: usize,
}

impl Wave for ReverbWave {
    type Input = f32;
    type Output = f32;

    fn new() -> (Self, WaveDescription) { // .... and a param definition
        (
            ReverbWave {
                reverbs: vec![
                    Reverb { delay: 0.020, gain: 0.6 },
                    Reverb { delay: 0.033, gain: 0.22 },
                    Reverb { delay: 0.047, gain: 0.15 },
                ],
                delay_channel : Vec::new(),
                at: 0,
            },
            WaveDescription {
                name : "Reverb".into(),
                vendor : "Waves".into(),
                unique_id: 9999632,
                category: plugin::Category::Effect,
            }
        )
    }

    fn process(&mut self, time:TimeSpan, input: &[f32]) -> Vec<f32> {
        let mut out = Vec::with_capacity(time.samples);

        let reverbs : Vec<_> = self.reverbs.iter().map(|r| ((r.delay / time.time_per_sample()) as usize, r.gain)).collect();

        let max_samples = reverbs.last().expect("a reverb").0 + 1; // assumes ordered for now
        
        // ensure circular buffer is big enough
        while self.delay_channel.len() < max_samples {
            self.delay_channel.push(0.0);
        }

        for v in input.into_iter().cloned() {
            let frontier_idx = (self.at + max_samples - 1) % max_samples;
            // zero out frontier
            self.delay_channel[frontier_idx] = 0.0;

            let o = v + self.delay_channel[self.at];
            out.push(o);

            for &(delay, gain) in &reverbs {
                let delay_idx = (self.at + delay) % max_samples; // circ buffer
                self.delay_channel[delay_idx] = o * gain;
            }

            self.at = (self.at + 1) % max_samples;
        }

        out
    }   
}
