use image::GenericImageView;
use nannou::prelude::*;

const FILE_NAME: &str = "test.jpg";
const STEP_SIZE: usize = 3;

struct Model {}
fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    app.new_window().view(view).build().unwrap();
    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    let img = image::open(FILE_NAME).unwrap();
    let (width, height) = img.dimensions();    
    let window_rect = app.window_rect();
    
    for y in (0..height).step_by(STEP_SIZE as usize) {
        let mut points = Vec::new();
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let brightness = 0.299 * pixel[0] as f32 + 0.587 * pixel[1] as f32 + 0.114 * pixel[2] as f32;
            let amp = map_range(brightness, 0.0, 255.0, 10.0, 50.0);
            let mapped_x = map_range(x as f32, 0.0, width as f32, window_rect.left(), window_rect.right());
            let mapped_y = map_range(y as f32, 0.0, height as f32, window_rect.top(), window_rect.bottom()) - amp;
            points.push(pt2(mapped_x, mapped_y));
        }
        draw.polyline().points(points).color(WHITE);
    }
    draw.to_frame(app, &frame).unwrap();
}
