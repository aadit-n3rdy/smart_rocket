use sfml::graphics::*;
use sfml::window::*;

mod matrix;
use matrix::*;

mod perceptron;
use perceptron::Perceptron;

mod vec2;
use vec2::Vec2;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
const rocket_velocity_multiplier: f64 = 0.5;
const rocket_accel_multiplier: f64 = 20.0;
const rocket_velocity_decay_rate: f64 = 0.9;
const rocket_start_position: Vec2 = Vec2{x: 0.0, y: (f64)WINDOW_HEIGHT/2.0};
const rocket_ptron_shape: Vec<usize> = vec!(8, 8, 8, 2);
const rocket_radius: f32 = 40.0;
const rocket_point_count: u32 = 50;


pub struct Rocket{
    pub pos: Vec2,
    vel: Vec2,
    ptron: Perceptron,
    circle: ffi::sfCircleShape
}

impl Rocket<'a> {
    pub fn create() -> Rocket {
        return Rocket{pos: rocket_start_position,
                      vel: Vec2{x:0.0, y:0.0},
                      ptron: Perceptron::create(rocket_ptron_shape),
                      circle: CircleShape::new(rocket_radius, rocket_point_count);
        }.clone();
    }
    pub fn update(&mut self) {
        
    } //#TODO include obstacle reference and target reference
    pub fn draw(&self, window: &RenderWindow) {
        CircleShape::new(rocket_radius, rocket_point_count);
    }
}

fn main() {
    let mut window = RenderWindow::new((800, 600),
                                 "Hello World",
                                 Style::CLOSE,
                                       &Default::default());
    let mut circle : CircleShape = CircleShape::default();
    while window.is_open() {
        while let Some(event) = window.poll_event() {
            if event == Event::Closed {
                window.close();
            }
        }
        let v1 = Vector2::<f32>::from((10.0, 10.0));
        window.clear(Color::GREEN);
        circle.set_position(Vector2::<f32>::from((10.0, 10.0)));
        circle.set_fill_color(sfml::graphics::Color::RED);
        circle.set_radius(10.0);
        window.draw(&circle);
        window.display();
    }
}
