

use std::fmt::Debug;

// type ReadBuffer = &mut Vec<&mut [f32]>;



pub trait IO : Sized + Debug {
    fn channel_count() -> usize;
    fn sink(data: &[Self], buffer: &mut Vec<&mut [f32]>);
    fn read(buffer: &Vec<&mut [f32]>) -> Vec<Self>;
}

impl IO for () {
    fn channel_count() -> usize { 0 }
    #[allow(unused_variables)]
    fn sink(data: &[()], buffer: &mut Vec<&mut [f32]>) { }
    #[allow(unused_variables)]
    fn read(buffer: &Vec<&mut [f32]>) -> Vec<()> { Vec::new() }
}

impl IO for f32 {
    fn channel_count() -> usize { 1 }
    fn sink(data: &[f32], buffer: &mut Vec<&mut [f32]>) {
        for (i, d) in data.iter().enumerate() {
            buffer[0][i] = *d;
        }
    }
    fn read(buffer: &Vec<&mut [f32]>) -> Vec<f32> {
        assert!(buffer.len() == 1, "IO(f32) requires 1 channel, {:?} was given", buffer.len());

        let channel = &buffer[0];

        let mut v = Vec::with_capacity(channel.len());
        v.extend_from_slice(channel);
        v
    }
}

impl IO for (f32, f32) {
    fn channel_count() -> usize { 2 }
    fn sink(data: &[(f32, f32)], buffer: &mut Vec<&mut [f32]>) {
        for (i, &(l, r)) in data.iter().enumerate() {
            buffer[0][i] = l;
            buffer[1][i] = r;
        }
    }
    fn read(buffer: &Vec<&mut [f32]>) -> Vec<(f32, f32)> {
        assert!(buffer.len() == 2, "IO (f32, f32) requires 2 channel, {:?} was given", buffer.len());
        let ref a = buffer[0];
        let ref b = buffer[1];
        a.iter().zip(b.iter()).map( |(l, r)| { (*l, *r) } ).collect()
    }
}
    
