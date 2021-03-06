
extern crate vst2;
#[macro_use] 
extern crate log;
extern crate simplelog;
extern crate num_traits;
extern crate log_panics;

use simplelog::*;
use std::fs::File;

use vst2::buffer::AudioBuffer;
use vst2::plugin::{Plugin, Info, HostCallback};

// use num_traits::Float;

pub mod io;
pub mod param;
pub mod time;

use io::IO;

pub use time::*;
pub use vst2::*;    

pub type Category = vst2::plugin::Category;

#[macro_export]
macro_rules! wave {
    ($wave:ty) => {
        plugin_main!(WaveHost<$wave>);
    }
}

pub struct WaveHost<W> where W : Wave {
    pub wave : W,
    pub description: WaveDescription,
    pub sample_rate : f64,
    pub time: f64,
    // params
}

pub struct WaveDescription {
    pub name : String,
    pub vendor : String,
    pub unique_id: i32,
    pub category: Category,
}

pub trait Wave where Self : Sized {
    type Input : IO;
    type Output : IO;
    // type state?

    fn new() -> (Self, WaveDescription);
    // input: &[Self::Input]
    fn process(&mut self, time:TimeSpan, input: &[Self::Input]) -> Vec<Self::Output>;
}


impl<W> Default for WaveHost<W> where W : Wave {
    fn default() -> Self {
        log_panics::init();

        let _ = CombinedLogger::init(
            vec![
                // TermLogger::new( LevelFilter::Warn, Config::default()).unwrap(),
                WriteLogger::new(LogLevelFilter::Info, Config::default(), File::create("/tmp/waves.log").unwrap()),
            ]
        );

        info!("someone calling new wave!");

        let (wave, description) = W::new();

        WaveHost {
            wave : wave,
            description: description,
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
        let in_count = W::Input::channel_count() as i32;
        let out_count = W::Output::channel_count() as i32;

        // info!("asked for info in_count {} out_count {}", in_count, out_count);

        Info {
            name: self.description.name.clone(),
            vendor: self.description.vendor.clone(),
            unique_id: self.description.unique_id,
            category: self.description.category,
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

            // info!("got some samples input {} output {}", inputs.len(), outputs.len());

            let input = W::Input::read(&inputs);

            let out = self.wave.process(time_span, &input);
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

