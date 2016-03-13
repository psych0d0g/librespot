extern crate portaudio;

use std::fs::File;
use std::env::args;
use std::f32::consts::PI;

pub fn main() {
    portaudio::initialize().unwrap();

    let stream = portaudio::stream::Stream::<i16, i16>::open_default(
                    0, 2, 44100.0, portaudio::stream::FRAMES_PER_BUFFER_UNSPECIFIED, None
                ).unwrap();

    stream.start().unwrap();

    let mut buf = [0i16; 44100];
    for (idx, sample) in buf.iter_mut().enumerate() {
        let phase = (idx as f32) / 100.0 * PI * 2.0;
        *sample = (phase.sin() * 32768f32) as i16;
    }

    loop {
        for packet in buf.chunks(2048) {
            match stream.write(&packet) {
                Ok(_) => (),
                Err(portaudio::PaError::OutputUnderflowed) => println!("Underflow"),
                Err(e) => panic!("PA Error {}", e),
            };
        }
    }

    drop(stream);

    portaudio::terminate().unwrap();
}
