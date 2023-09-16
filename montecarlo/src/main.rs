use speedy2d::color::Color;
use speedy2d::{Graphics2D, Window};
use speedy2d::window::{WindowHandler, WindowHelper};
use montecarlo::*;
use rand::Rng;

fn main() {
    // Config variables
    let width: u32 = 1920;
    let height: u32 = 1080;
    let num_points = 1_000_00;

    // Generate points
    let mut rng = rand::thread_rng();
    let mut canvas = Canvas::new(width, height);
    for _ in 0..(num_points - 1) {
        let new_x: f32 = rng.gen_range(0.0..width as f32);
        let new_y: f32 = rng.gen_range(0.0..height as f32);
        let new_point = Point::new(new_x, new_y);
        canvas.points.push(new_point);
    }
    
    // Setup Window
    let window = Window::new_centered("Monte Carlo", (width, height)).unwrap();
    window.run_loop(MyWindowHandler{canvas});
}

struct MyWindowHandler {
    pub canvas: Canvas,
}

impl WindowHandler for MyWindowHandler {
    /// Handles drawing the window and the objects in it
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        graphics.clear_screen(Color::from_rgb(0.8, 0.9, 1.0));
        //graphics.draw_circle((100.0, 100.0), 70.0, Color::BLUE);
        //graphics.draw_circle((100.0, 100.0), 2.0, Color::BLACK);
        
        for i in 0..(self.canvas.points.len() - 1) {
            let draw_point = &self.canvas.points[i];
            graphics.draw_circle((draw_point.x, draw_point.y), 2.0, Color::BLACK);
        }

        helper.request_redraw();
    }
}
