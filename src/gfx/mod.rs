mod canvas;
mod draw_ui_ctx;
mod draw_world_ctx;
mod event_ctx;
mod font_cache;
mod image_cache;
mod window;

pub use canvas::Canvas;
pub use draw_ui_ctx::DrawUiCtx;
pub use draw_world_ctx::DrawWorldCtx;
pub use event_ctx::EventCtx;
pub use window::Window;

use crate::{
    ui::App,
    utils::{Point2, Vec2},
};
pub use sdl3::keyboard::Keycode as KeyCode;
pub use sdl3::mouse::MouseButton;

#[derive(Clone, Copy, PartialEq, Debug, serde::Serialize, serde::Deserialize, Hash)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    pub const fn to_f32_array(&self) -> [f32; 4] {
        [
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0,
            self.a as f32 / 255.0,
        ]
    }

    pub(crate) const fn rgbaf(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color {
            r: (r * 255.0) as u8,
            g: (g * 255.0) as u8,
            b: (b * 255.0) as u8,
            a: (a * 255.0) as u8,
        }
    }
}

impl Into<sdl3::pixels::Color> for Color {
    fn into(self) -> sdl3::pixels::Color {
        sdl3::pixels::Color::RGBA(self.r, self.g, self.b, self.a)
    }
}

#[derive(Clone, Copy, Debug, Hash)]
pub struct Paint {
    pub color: Color,
    pub line_width: Option<u16>,
    pub font_size: Option<u16>,
}
impl Paint {
    pub fn color(layer_color: Color) -> Self {
        Self {
            color: layer_color,
            line_width: None,
            font_size: None,
        }
    }

    pub fn with_line_width(self, arg: u16) -> Self {
        Self {
            line_width: Some(arg),
            ..self
        }
    }

    pub fn with_font_size(self, arg: u16) -> Self {
        Self {
            font_size: Some(arg),
            ..self
        }
    }
}

pub fn run(window: &mut Window, app: &mut crate::ui::app::App) {
    let mut event_pump = window.sdl_context.event_pump().unwrap();
    let mut running = true;

    let mut mouse = Point2::ZERO;
    let window_size = Point2::new(
        window.canvas.viewport().width() as f32,
        window.canvas.viewport().height() as f32,
    );

    let mut canvas = Canvas {
        rect: window.canvas.viewport(),
        window_size,
        canvas: &mut window.canvas,
    };

    while running {
        let world_mouse = canvas.ui_to_world(mouse);
        for event in event_pump.poll_iter() {
            let mut event_ctx = EventCtx {
                ui_mouse: mouse,
                world_mouse,
                canvas: &mut canvas,
                window_size,
                running: &mut running,
                redraw: false,
            };
            match event {
                sdl3::event::Event::Quit { .. } => *event_ctx.running = false,
                sdl3::event::Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => app.key_up(&mut event_ctx, keycode),
                sdl3::event::Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => app.key_down(&mut event_ctx, keycode),
                sdl3::event::Event::MouseMotion {
                    x, y, xrel, yrel, ..
                } => {
                    mouse = Point2::new(x, y);
                    event_ctx.ui_mouse = mouse;
                    event_ctx.world_mouse = event_ctx.canvas.ui_to_world(mouse);
                    app.mouse_move(&mut event_ctx, Vec2::new(xrel, yrel));
                }
                sdl3::event::Event::MouseButtonDown {
                    mouse_btn, x, y, ..
                } => {
                    mouse = Point2::new(x, y);
                    event_ctx.ui_mouse = mouse;
                    event_ctx.world_mouse = event_ctx.canvas.ui_to_world(mouse);
                    app.mouse_down(&mut event_ctx, mouse_btn);
                }
                sdl3::event::Event::MouseButtonUp {
                    mouse_btn, x, y, ..
                } => {
                    mouse = Point2::new(x, y);
                    event_ctx.ui_mouse = mouse;
                    event_ctx.world_mouse = event_ctx.canvas.ui_to_world(mouse);
                    app.mouse_up(&mut event_ctx, mouse_btn);
                }
                sdl3::event::Event::MouseWheel { x, y, .. } => {
                    event_ctx.ui_mouse = mouse;
                    event_ctx.world_mouse = event_ctx.canvas.ui_to_world(mouse);
                    event_ctx.zoom_at_mouse(x as f32);
                }
                _ => {}
            }
        }

        let mut draw_ctx = DrawWorldCtx {
            canvas: &mut canvas,
            font_cache: &mut window.font_cache,
            image_cache: &mut window.image_cache,
            ui_mouse: mouse,
            world_mouse,
            window_size,
        };
        app.draw_world(&mut draw_ctx);

        let mut draw_ctx = DrawUiCtx {
            canvas: &mut canvas,
            font_cache: &mut window.font_cache,
            image_cache: &mut window.image_cache,
            ui_mouse: mouse,
            world_mouse,
            window_size,
        };
        app.draw_ui(&mut draw_ctx);

        canvas.canvas.present();
    }
}
