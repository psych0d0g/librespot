#[cfg(not(feature = "with-tremor"))]
extern crate vorbis;
#[cfg(feature = "with-tremor")]
extern crate tremor as vorbis;

extern crate portaudio;

use std::fs::File;
use std::env::args;

pub fn main() {
    let mut args = args().skip(1);
    let file_name = args.next().expect("No arguments given");
    let file = File::open(file_name).unwrap();
    let mut decoder = vorbis::Decoder::new(file).unwrap();

    portaudio::initialize().unwrap();

    let stream = portaudio::stream::Stream::<i16, i16>::open_default(
                    0, 2, 44100.0, portaudio::stream::FRAMES_PER_BUFFER_UNSPECIFIED, None
                ).unwrap();

    stream.start().unwrap();
    for packet in decoder.packets() {

        match packet {
            Ok(packet) => {
                match stream.write(&packet.data) {
                    Ok(_) => (),
                    Err(portaudio::PaError::OutputUnderflowed) => println!("Underflow"),
                    Err(e) => panic!("PA Error {}", e),
                };
            }
            Err(vorbis::VorbisError::Hole) => (),
            Err(e) => panic!("Vorbis error {:?}", e),
        }
    }

    drop(stream);

    portaudio::terminate().unwrap();
}
