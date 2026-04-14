use crate::framebuffer::FrameBuffer;

use sdl2::{
    AudioSubsystem, EventPump, VideoSubsystem, pixels::Color, rect::Rect, render::Canvas,
    video::Window,
};
use std::error::Error;

const SCREEN_WIDTH: u32 = 64;
const SCREEN_HEIGHT: u32 = 32;
const SCALE: u32 = 10; // Make window larger, each pixel becomes 10x10 window pixels

pub struct SDLContext {
    video: VideoSubsystem,
    audio: AudioSubsystem,
    canvas: Canvas<Window>,
    event_pump: EventPump,
}

impl SDLContext {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let sdl = sdl2::init()?;

        let video = sdl.video()?;
        let audio = sdl.audio()?;
        let event_pump = sdl.event_pump()?;

        let window = video
            .window(
                "Chip-8 Emulator",
                SCREEN_WIDTH * SCALE,
                SCREEN_HEIGHT * SCALE,
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
        })
    }

    /*
    pub fn init_display() -> Result<Self, Box<dyn Error>> {
        let sdl_context = sdl2::init()?;
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
    */

    pub fn render(&mut self, framebuffer: &FrameBuffer) {
        if !framebuffer.draw_flag {
            return;
        }

        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();
        self.canvas.set_draw_color(Color::WHITE);

        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let index = (y * SCREEN_WIDTH + x) as usize;

                if framebuffer.buffer[index] != 0 {
                    let _ = self.canvas.fill_rect(Rect::new(
                        (x * SCALE) as i32,
                        (y * SCALE) as i32,
                        SCALE,
                        SCALE,
                    ));
                }
            }
        }

        self.canvas.present();
    }

    pub fn audio(&self) -> &AudioSubsystem {
        &self.audio
    }

    pub fn events(&mut self) -> &mut EventPump {
        &mut self.event_pump
    }
}
