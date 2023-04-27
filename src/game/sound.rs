use soloud::*;
use std::path::Path;
use tokio::sync::mpsc;

pub struct Player {
    pub sl: Soloud,
    pub wav: audio::Wav,
    playing: bool,
    sound_file: String,
    rx: mpsc::Receiver<usize>,
}

impl Player {
    pub fn new(sound_file: &str, rx: mpsc::Receiver<usize>) -> Self {
        Self {
            sl: Soloud::default().unwrap(),
            wav: audio::Wav::default(),
            playing: false,
            sound_file: sound_file.to_string(),
            rx,
        }
    }

    pub fn play(&mut self) {
        if self.playing {
            return;
        }
        let file_path = Path::new(&self.sound_file);
        self.wav.load(file_path).unwrap();
        self.sl.play(&self.wav);
        while self.sl.voice_count() > 0 {
            std::thread::sleep(std::time::Duration::from_millis(100));
            if self.rx.try_recv().is_ok() {
                self.sl.stop_all();
                self.playing = false;
                break;
            }
        }
    }
}
