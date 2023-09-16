use speedy2d::color::Color;
use speedy2d::{Graphics2D, Window};
use speedy2d::window::{WindowHandler, WindowHelper};
use montecarlo::*;

fn main() {
    // Config variables
    let width = 1920;
    let height = 1080;

    // Setup points
    let canvas = Canvas::new(width, height);
    //TODO: Setup points here
    
    // Setup Window
    let window = Window::new_centered("Monte Carlo", (width, height)).unwrap();
    window.run_loop(MyWindowHandler{});
}

struct MyWindowHandler {}

impl WindowHandler for MyWindowHandler {
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        graphics.clear_screen(Color::from_rgb(0.8, 0.9, 1.0));
        // graphics.draw_circle((100.0, 100.0), 70.0, Color::BLUE);
        // graphics.draw_circle((100.0, 100.0), 2.0, Color::BLACK);

        helper.request_redraw();
    }
}
