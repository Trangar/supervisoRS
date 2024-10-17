use femtovg::Color;

// mod factorio;
mod ui;

fn main() {
    ui::start(1000, 800, "SupervisoRS", true, App {});
}

struct App {}

impl ui::App for App {
    fn draw(&mut self, canvas: &mut ui::Canvas) {
        canvas.clear_rect(0, 0, 100, 80, Color::rgbf(0.0, 0.0, 0.0));
    }

    fn key_down(&mut self, ctx: &mut ui::EventCtx, key: winit::event::VirtualKeyCode) {
        if key == winit::event::VirtualKeyCode::Escape {
            ctx.exit();
        }
    }
}
