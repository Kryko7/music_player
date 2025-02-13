use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;

pub struct AudioPlayer {
    _stream: OutputStream,
    sink: Sink,
}

impl AudioPlayer {
    pub fn new() -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        AudioPlayer { _stream, sink }
    }

    pub fn play_file(&self, file_path: &str) {
        self.sink.stop(); // Stop any current playback and clear the queue
        let file = File::open(file_path).expect("File not found");
        let reader = BufReader::new(file);
        let source = Decoder::new(reader).expect("Error decoding file");
        self.sink.append(source);
        self.sink.play(); // Ensure playback starts immediately
    }

    pub fn play(&self) {
        self.sink.play();
    }

    pub fn pause(&self) {
        self.sink.pause();
    }

    pub fn stop(&self) {
        self.sink.stop();
    }
}