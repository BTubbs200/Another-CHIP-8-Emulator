use crate::framebuffer::FrameBuffer;

use sdl3::{
    AudioSubsystem, EventPump, VideoSubsystem, pixels::Color, rect::Rect, render::WindowCanvas,
};
use std::error::Error;

const SCREEN_WIDTH: u32 = 64;
const SCREEN_HEIGHT: u32 = 32;
const SCALE: u32 = 10; // Make window larger, each pixel becomes 10x10 window pixels

pub struct SDLCreate {
    vid_subsys: VideoSubsystem,
    aud_subsys: AudioSubsystem,
    canvas: WindowCanvas,
    event_pump: EventPump,
}

impl SDLCreate {
    pub fn init_display() -> Result<Self, Box<dyn Error>> {
        let sdl_context = sdl3::init()?;
        let video_subsys = sdl_context.video()?;
        let audio_subsys = sdl_context.audio().unwrap();

        let window = video_subsys
            .window(
                "Chip-8 Emulator",
                SCREEN_WIDTH * SCALE,
                SCREEN_HEIGHT * SCALE,
            )
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;

        let canvas = window.into_canvas();
        let event_pump = sdl_context.event_pump()?;

        Ok(Self {
            vid_subsys: video_subsys,
            aud_subsys: audio_subsys,
            canvas,
            event_pump,
        })
    }

    pub fn render(&mut self, framebuffer: &mut FrameBuffer) {
        if !framebuffer.draw_flag {
            return;
        }

        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();

        for y in 0..32 {
            for x in 0..64 {
                let index = y * 64 + x;
                if framebuffer.buffer[index] == 1 {
                    self.canvas.set_draw_color(Color::WHITE);
                    let _ = self
                        .canvas
                        .fill_rect(Rect::new(x as i32 * 10, y as i32 * 10, 10, 10));
                }
            }
        }

        self.canvas.present();
        framebuffer.draw_flag = false;
    }

    pub fn audio_subsystem(&self) -> &AudioSubsystem {
        &self.aud_subsys
    }

    pub fn event_pump(&mut self) -> &mut EventPump {
        &mut self.event_pump
    }
}
