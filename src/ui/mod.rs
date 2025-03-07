pub mod app;
pub mod context_menu;
pub mod drag;
pub mod hover;
pub mod image_ctx;
pub mod selector;
pub mod utils;

#[cfg(not(target_arch = "wasm32"))]
use std::num::NonZeroU32;

use femtovg::renderer::OpenGl;
use glutin::{
    config::{ConfigTemplateBuilder, GlConfig},
    context::{ContextApi, ContextAttributesBuilder},
    display::GetGlDisplay,
    prelude::{GlDisplay, NotCurrentGlContextSurfaceAccessor},
    surface::{GlSurface, SurfaceAttributesBuilder, WindowSurface},
};
use glutin_winit::DisplayBuilder;
use raw_window_handle::HasRawWindowHandle;
use winit::{
    event::{Event, KeyboardInput, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use crate::{
    state::Theme,
    utils::{Point2, Vec2},
};

pub type Canvas = femtovg::Canvas<femtovg::renderer::OpenGl>;
pub trait App {
    fn draw(&mut self, canvas: &mut Canvas, mouse: Point2, window_size: Point2);
    fn resize(&mut self, _ctx: &mut EventCtx, _width: u32, _height: u32) {}
    fn key_down(&mut self, _ctx: &mut EventCtx, _key: winit::event::VirtualKeyCode) {}
    fn key_up(&mut self, _ctx: &mut EventCtx, _key: winit::event::VirtualKeyCode) {}
    fn mouse_move(&mut self, _ctx: &mut EventCtx, _delta: Vec2) {}
    fn mouse_down(&mut self, _ctx: &mut EventCtx, _button: winit::event::MouseButton) {}
    fn mouse_up(&mut self, _ctx: &mut EventCtx, _button: winit::event::MouseButton) {}
    fn mouse_scroll(&mut self, _ctx: &mut EventCtx, _delta: winit::event::MouseScrollDelta) {}
}

pub fn start(
    #[cfg(not(target_arch = "wasm32"))] width: u32,
    #[cfg(not(target_arch = "wasm32"))] height: u32,
    #[cfg(not(target_arch = "wasm32"))] title: &'static str,
    #[cfg(not(target_arch = "wasm32"))] resizeable: bool,
    mut app: impl App + 'static,
) {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(all(debug_assertions, target_arch = "wasm32"))]
    console_error_panic_hook::set_once();

    let event_loop = EventLoop::new();

    #[cfg(not(target_arch = "wasm32"))]
    let (mut canvas, window, context, surface) = {
        let window_builder = WindowBuilder::new()
            .with_inner_size(winit::dpi::PhysicalSize::new(width, height))
            .with_resizable(resizeable)
            .with_title(title);

        let template = ConfigTemplateBuilder::new().with_alpha_size(8);

        let display_builder = DisplayBuilder::new().with_window_builder(Some(window_builder));

        let (window, gl_config) = display_builder
            .build(&event_loop, template, |configs| {
                // Find the config with the maximum number of samples, so our triangle will
                // be smooth.
                configs
                    .reduce(|accum, config| {
                        let transparency_check = config.supports_transparency().unwrap_or(false)
                            & !accum.supports_transparency().unwrap_or(false);

                        if transparency_check || config.num_samples() < accum.num_samples() {
                            config
                        } else {
                            accum
                        }
                    })
                    .unwrap()
            })
            .unwrap();

        let window = window.unwrap();

        let raw_window_handle = Some(window.raw_window_handle());

        let gl_display = gl_config.display();

        let context_attributes = ContextAttributesBuilder::new().build(raw_window_handle);
        let fallback_context_attributes = ContextAttributesBuilder::new()
            .with_context_api(ContextApi::Gles(None))
            .build(raw_window_handle);
        let mut not_current_gl_context = Some(unsafe {
            gl_display
                .create_context(&gl_config, &context_attributes)
                .unwrap_or_else(|_| {
                    gl_display
                        .create_context(&gl_config, &fallback_context_attributes)
                        .expect("failed to create context")
                })
        });

        let (width, height): (u32, u32) = window.inner_size().into();
        let raw_window_handle = window.raw_window_handle();
        let attrs = SurfaceAttributesBuilder::<WindowSurface>::new().build(
            raw_window_handle,
            NonZeroU32::new(width).unwrap(),
            NonZeroU32::new(height).unwrap(),
        );

        let surface = unsafe {
            gl_config
                .display()
                .create_window_surface(&gl_config, &attrs)
                .unwrap()
        };

        let gl_context = not_current_gl_context
            .take()
            .unwrap()
            .make_current(&surface)
            .unwrap();

        let renderer =
            unsafe { OpenGl::new_from_function_cstr(|s| gl_display.get_proc_address(s).cast()) }
                .expect("Cannot create renderer");

        let mut canvas = Canvas::new(renderer).expect("Cannot create canvas");
        canvas.set_size(width, height, window.scale_factor() as f32);
        canvas.add_font("assets/Roboto-Regular.ttf").unwrap();

        (canvas, window, gl_context, surface)
    };

    #[cfg(target_arch = "wasm32")]
    let (canvas, window) = {
        use wasm_bindgen::JsCast;

        let canvas = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("canvas")
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();

        use winit::platform::web::WindowBuilderExtWebSys;

        let renderer = OpenGl::new_from_html_canvas(&canvas).expect("Cannot create renderer");

        let window = WindowBuilder::new()
            .with_canvas(Some(canvas))
            .build(&event_loop)
            .unwrap();

        let canvas = Canvas::new(renderer).expect("Cannot create canvas");

        (canvas, window)
    };

    // let start = instant::Instant::now();
    // let mut prevt = start;

    let mut mouse = Point2::ZERO;
    // let mut dragging = false;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        let mut ctx = EventCtx {
            mouse: rel_mouse(mouse, &canvas),
            canvas: &mut canvas,
            window_size: Point2::new(width as f32, height as f32),
            redraw: false,
            control_flow,
        };

        match event {
            Event::LoopDestroyed => ctx.exit(),
            Event::WindowEvent { ref event, .. } => match event {
                #[cfg(not(target_arch = "wasm32"))]
                WindowEvent::Resized(physical_size) => {
                    surface.resize(
                        &context,
                        physical_size.width.try_into().unwrap(),
                        physical_size.height.try_into().unwrap(),
                    );
                    app.resize(&mut ctx, physical_size.width, physical_size.height);
                }
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state,
                            virtual_keycode: Some(virtual_keycode),
                            ..
                        },
                    ..
                } => {
                    if *state == winit::event::ElementState::Pressed {
                        app.key_down(&mut ctx, *virtual_keycode);
                    } else {
                        app.key_up(&mut ctx, *virtual_keycode);
                    }
                }
                WindowEvent::MouseWheel { delta, .. } => {
                    app.mouse_scroll(&mut ctx, *delta);
                }
                WindowEvent::CursorMoved { position, .. } => {
                    let new_position = Point2::new(position.x as f32, position.y as f32);
                    let delta = Vec2::from(new_position - mouse);
                    mouse = new_position;
                    ctx.mouse = rel_mouse(mouse, ctx.canvas);

                    app.mouse_move(&mut ctx, delta);
                }
                WindowEvent::MouseInput { state, button, .. } => {
                    if *state == winit::event::ElementState::Pressed {
                        app.mouse_down(&mut ctx, *button);
                    } else {
                        app.mouse_up(&mut ctx, *button);
                    }
                }
                WindowEvent::CloseRequested => ctx.exit(),
                _ => (),
            },
            Event::RedrawRequested(_) => {
                // let now = instant::Instant::now();
                // let dt = (now - prevt).as_secs_f32();
                // prevt = now;

                let dpi_factor = window.scale_factor();
                let size = window.inner_size();

                // let t = start.elapsed().as_secs_f32();

                ctx.canvas
                    .set_size(size.width, size.height, dpi_factor as f32);

                app.draw(ctx.canvas, ctx.mouse, ctx.window_size);

                ctx.canvas.save_with(|canvas| {
                    canvas.reset();
                });

                ctx.canvas.flush();
                #[cfg(not(target_arch = "wasm32"))]
                surface.swap_buffers(&context).unwrap();
            }
            Event::MainEventsCleared => {}
            _ => (),
        }

        if ctx.redraw {
            window.request_redraw();
        }
    });
}

fn rel_mouse(abs_mouse: Point2, canvas: &Canvas) -> Point2 {
    canvas
        .transform()
        .inversed()
        .transform_point(abs_mouse.x, abs_mouse.y)
        .into()
}

pub struct DrawCtx<'a> {
    pub canvas: &'a mut Canvas,
    pub theme: &'a Theme,
    pub mouse: Point2,
    #[allow(dead_code)]
    pub window_size: Point2,
}
impl DrawCtx<'_> {
    pub fn top_left_of_window(&self) -> Point2 {
        self.canvas
            .transform()
            .inversed()
            .transform_point(0.0, 0.0)
            .into()
    }
}

pub struct EventCtx<'a> {
    pub canvas: &'a mut Canvas,
    pub mouse: Point2,
    pub window_size: Point2,

    redraw: bool,

    control_flow: &'a mut ControlFlow,
}

impl EventCtx<'_> {
    pub fn exit(&mut self) {
        *self.control_flow = ControlFlow::Exit;
    }

    pub fn zoom_at_mouse(&mut self, zoom: f32) {
        let pt = self
            .canvas
            .transform()
            .inversed()
            .transform_point(self.mouse.x, self.mouse.y);
        self.canvas.translate(pt.0, pt.1);
        self.canvas.scale(1.0 + (zoom / 10.0), 1.0 + (zoom / 10.0));
        self.canvas.translate(-pt.0, -pt.1);
        self.redraw = true;
    }

    pub fn translate_by(&mut self, relative: Vec2) {
        let p0 = self
            .canvas
            .transform()
            .inversed()
            .transform_point(self.mouse.x, self.mouse.y);
        let p1 = self
            .canvas
            .transform()
            .inversed()
            .transform_point(self.mouse.x + relative.x, self.mouse.y + relative.y);

        self.canvas.translate(p1.0 - p0.0, p1.1 - p0.1);
        self.redraw = true;
    }

    pub fn redraw(&mut self) {
        self.redraw = true;
    }
}
