use sdl3::audio::{AudioCallback, AudioSpec, AudioStreamWithCallback};

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

impl AudioCallback<f32> for SquareWave {
    fn callback(&mut self, stream: &mut sdl3::audio::AudioStream, requested: i32) {
        let mut buffer = Vec::<f32>::with_capacity(requested as usize);

        for _ in 0..requested {
            if self.playing {
                let sample = if self.phase <= 0.5 {
                    self.volume
                } else {
                    -self.volume
                };
                buffer.push(sample);
            } else {
                buffer.push(0.0)
            }
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
        stream.put_data_f32(&buffer);
    }
}

pub fn init_audio_stream(
    audio_subsystem: &sdl3::AudioSubsystem,
) -> AudioStreamWithCallback<SquareWave> {
    let spec = AudioSpec {
        freq: Some(44100),
        channels: Some(1),
        format: Some(sdl3::audio::AudioFormat::f32_sys()),
    };

    let stream = audio_subsystem
        .open_playback_stream(
            &spec,
            SquareWave {
                phase_inc: 440.0 / 44100.0,
                phase: 0.0,
                volume: 0.05,
                playing: false,
            },
        )
        .unwrap();

    // Start playback
    stream.resume().unwrap();

    stream
}
