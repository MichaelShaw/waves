

use std::fmt::Debug;

pub trait IO : Sized + Debug {
    
    fn channel_count() -> usize;
    fn sink(data: &[Self], buffer: &mut Vec<&mut [f32]>);
    // fn sink(v:Vec<Item>);


 
    // a zipper thingy
}

impl IO for () {
    fn channel_count() -> usize { 0 }
    #[allow(unused_variables)]
    fn sink(data: &[()], buffer: &mut Vec<&mut [f32]>) { }
}

impl IO for f32 {
    // type Item = f32;
    fn channel_count() -> usize { 1 }
    fn sink(data: &[f32], buffer: &mut Vec<&mut [f32]>) {
        for (i, d) in data.iter().enumerate() {
            buffer[0][i] = *d;
        }
    }
}

// impl IO for (f32, f32) {
//     type Item = (f32, f32);
//     fn channel_count() -> usize { 2 }
// }
    
