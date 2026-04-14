//FIXME: Sounds only playing intermittently!

use sdl2::audio::{AudioCallback, AudioDevice, AudioSpecDesired};

pub struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32,
    playing: bool,
}

impl SquareWave {
    pub fn set_playing(&mut self, playing: bool) {
        self.playing = playing;
    }
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        for sample in out.iter_mut() {
            if self.playing {
                *sample = if self.phase <= 0.5 {
                    self.volume
                } else {
                    -self.volume
                };
            } else {
                *sample = 0.0;
            }

            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}

pub fn init_audio_device(audio_subsystem: &sdl2::AudioSubsystem) -> AudioDevice<SquareWave> {
    let desired_spec = AudioSpecDesired {
        freq: Some(44100),
        channels: Some(1),
        samples: None,
    };

    let device = audio_subsystem
        .open_playback(None, &desired_spec, |spec| SquareWave {
            phase_inc: 440.0 / spec.freq as f32,
            phase: 0.0,
            volume: 0.05,
            playing: false,
        })
        .unwrap();

    // Start playback
    device.resume();
    device
}
