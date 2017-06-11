
#[macro_use]
extern crate vst2;
#[macro_use] 
extern crate log;
extern crate simplelog;
extern crate num_traits;

use simplelog::*;
use std::fs::File;

use vst2::buffer::AudioBuffer;
use vst2::plugin::{Category, Plugin, Info, HostCallback};

// use num_traits::Float;

pub mod io;
pub mod param;
pub mod time;

use io::IO;

use time::TimeSpan;

use std::f64::consts::PI;

pub struct WaveHost<W> where W : Wave {
    pub wave : W,
    pub sample_rate : f64,
    pub time: f64,
    // params
}


pub trait Wave {
    type Input : IO;
    type Output : IO;

    fn new() -> Self;
    fn process(time:TimeSpan) -> Vec<<Self as Wave>::Output>;
}

pub const TAU : f64 = PI * 2.0;

plugin_main!(WaveHost<SineWave>);

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

impl<W> Default for WaveHost<W> where W : Wave {
    fn default() -> Self {
        let _ = CombinedLogger::init(
            vec![
                // TermLogger::new( LevelFilter::Warn, Config::default()).unwrap(),
                WriteLogger::new(LogLevelFilter::Info, Config::default(), File::create("/tmp/waves.log").unwrap()),
            ]
        );

        info!("someone calling new wave!");

        WaveHost {
            wave : W::new(),
            sample_rate: 44100.0,
            time: 0.0,
        }
    }

}

impl<W> Plugin for WaveHost<W> where W : Wave {
    fn new(host: HostCallback) -> Self {
        Self::default()
    }

    fn set_sample_rate(&mut self, rate: f32) { 
        self.sample_rate = rate as f64;
    }

    fn get_info(&self) -> Info {
        // info!("someone calling get info!!");

        let in_count = W::Input::channel_count() as i32;
        let out_count = W::Output::channel_count() as i32;

        // info!("asked for info in_count {} out_count {}", in_count, out_count);

        Info {
            name: "Waves".to_string(),
            vendor: "WaveyHost".to_string(),
            unique_id: 25032522,
            category: Category::Effect,
            inputs: in_count,
            outputs: out_count,
            parameters: 0,
            ..Info::default()
        }
    }

    fn process(&mut self, buffer: AudioBuffer<f32>) {
        let (inputs, mut outputs) = buffer.split();
        let sample_count = sample_count(&inputs).or(sample_count(&outputs));

        if let Some(samples) = sample_count {
            let time_span = TimeSpan {
                start: self.time,
                samples: samples,
                sample_rate: self.sample_rate,
            };

            let out = W::process(time_span);
            W::Output::sink(out.as_slice(), &mut outputs);

            self.time += time_span.total_time();
        } else {
            info!("no samples :-(")
        }
    }
}

fn sample_count<'a>(buff: &Vec<&'a mut [f32]>) -> Option<usize> {
    buff.first().map(|b| b.len() )
}

