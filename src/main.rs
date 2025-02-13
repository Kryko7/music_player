mod audio;
use audio::AudioPlayer;
use eframe::egui;
use std::sync::{Arc, Mutex};

struct MusicApp {
    audio_player: Arc<Mutex<AudioPlayer>>,
    current_track: Option<String>,
    playing: bool,
}

impl Default for MusicApp {
    fn default() -> Self {
        MusicApp {
            audio_player: Arc::new(Mutex::new(AudioPlayer::new())),
            current_track: None,
            playing: false,
        }
    }
}

impl eframe::App for MusicApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Rust Music Player");

            if let Some(track) = &self.current_track {
                ui.label(format!("Now Playing: {}", track));
            }

            ui.horizontal(|ui| {
                if ui.button("Open File").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        let path_str = path.display().to_string();
                        self.current_track = Some(path_str.clone());
                        let player = self.audio_player.lock().unwrap();
                        player.play_file(&path_str);
                        self.playing = true;
                    }
                }

                if ui.button("Pause").clicked() && self.playing {
                    self.audio_player.lock().unwrap().pause();
                    self.playing = false;
                }

                if ui.button("Play").clicked() && !self.playing {
                    self.audio_player.lock().unwrap().play();
                    self.playing = true;
                }

                if ui.button("Stop").clicked() {
                    self.audio_player.lock().unwrap().stop();
                    self.playing = false;
                    self.current_track = None;
                }
            });
        });
    }
}

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Rust Music Player",
        native_options,
        Box::new(|_cc| Ok(Box::new(MusicApp::default()))),
    );
}