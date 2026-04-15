use crate::framebuffer::FrameBuffer;

use sdl2::{
    AudioSubsystem, EventPump, VideoSubsystem, pixels::Color, rect::Rect, render::Canvas,
    video::Window,
};
use std::error::Error;

const EMULATOR_WIDTH: u32 = 64;
const EMULATOR_HEIGHT: u32 = 32;

pub struct SDLContext {
    video: VideoSubsystem,
    audio: AudioSubsystem,
    canvas: Canvas<Window>,
    event_pump: EventPump,
    scale: u32,
}

impl SDLContext {
    pub fn new(window_scale: u32) -> Result<Self, Box<dyn Error>> {
        let sdl = sdl2::init()?;

        let video = sdl.video()?;
        let audio = sdl.audio()?;
        let event_pump = sdl.event_pump()?;
        let scale = window_scale;

        let window = video
            .window(
                "Chip-8 Emulator",
                EMULATOR_WIDTH * scale,
                EMULATOR_HEIGHT * scale,
            )
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;

        let canvas = window
            .into_canvas()
            // TODO: toggle vsync
            .present_vsync()
            .build()
            .map_err(|e| e.to_string())?;

        Ok(Self {
            video,
            audio,
            canvas,
            event_pump,
            scale,
        })
    }

    pub fn render(&mut self, framebuffer: &FrameBuffer) {
        if !framebuffer.draw_flag {
            return;
        }

        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();
        self.canvas.set_draw_color(Color::WHITE);

        for y in 0..EMULATOR_HEIGHT {
            for x in 0..EMULATOR_WIDTH {
                let index = (y * EMULATOR_WIDTH + x) as usize;

                if framebuffer.buffer[index] != 0 {
                    let _ = self.canvas.fill_rect(Rect::new(
                        (x * self.scale) as i32,
                        (y * self.scale) as i32,
                        self.scale,
                        self.scale,
                    ));
                }
            }
        }

        self.canvas.present()
    }

    pub fn audio(&self) -> &AudioSubsystem {
        &self.audio
    }

    pub fn events(&mut self) -> &mut EventPump {
        &mut self.event_pump
    }
}
