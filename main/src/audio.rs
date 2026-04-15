//FIXME: Sounds only playing intermittently!

use sdl2::audio::{AudioCallback, AudioDevice, AudioSpecDesired};

pub struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32,
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        for sample in out.iter_mut() {
            *sample = if self.phase <= 0.5 {
                self.volume
            } else {
                -self.volume
            };

            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}

pub fn init_audio_device(
    audio_subsystem: &sdl2::AudioSubsystem,
    volume: u32,
) -> AudioDevice<SquareWave> {
    let desired_spec = AudioSpecDesired {
        freq: Some(44100),
        channels: Some(1),
        samples: None,
    };

    let device = audio_subsystem
        .open_playback(None, &desired_spec, |spec| SquareWave {
            phase_inc: 440.0 / spec.freq as f32,
            phase: 0.0,
            // TODO: volume arg divided by 1000
            volume: volume as f32 / 1000.0,
        })
        .unwrap();

    // Start playback
    device.resume();
    device
}
