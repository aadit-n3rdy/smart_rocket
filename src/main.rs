use sfml::graphics::*;
use sfml::window::*;

mod matrix;
use matrix::*;

mod perceptron;
use perceptron::Perceptron;

mod vec2;
use vec2::Vec2;

use sfml::system::Vector2;

use rand::prelude::*;

mod constants;
use constants::*;

#[derive(Debug, Clone)]
pub struct Target {
    pos: Vec2,
    rad: f64
}

impl Target {
    pub fn create() -> Target {
        return Target {
            pos: Vec2 { x: 0.0, y: 0.0},
            rad: 50.0
        };
    }
    pub fn create_from_details(position: Vec2, radius: f64) -> Target {
        return Target {
            pos: position,
            rad: radius
        };
    }
    pub fn get_pos(&self) -> Vec2 {
        return self.pos.clone();
    }
    pub fn get_rad(&self) -> f64 {
        return self.rad;
    }
}

//TODO: Move code to rocket.rs

#[derive(Debug, Clone)]
enum RocketState {
    ALIVE,
    DEAD,
    DONE
}

#[derive(Debug, Clone)]
pub struct Rocket {
    pub pos: Vec2,
    vel: Vec2,
    ptron: Perceptron,
    state: RocketState
}

impl Rocket {
    pub fn create() -> Rocket {
        return Rocket {
            pos: rocket_start_position,
            vel: Vec2 { x: 0.0, y: 0.0 },
            ptron: Perceptron::create(vec![8, 8, 8, 2]),
            state: RocketState::ALIVE
        }
        .clone();
    }
    pub fn create_from_parent(parent: &Rocket, enable_variation: bool) -> Rocket {
        let mut rng = thread_rng();
        if !enable_variation {
            return Rocket {
                pos: rocket_start_position,
                vel: Vec2 { x: 0.0, y: 0.0 },
                ptron: parent.ptron.clone(),
                state: RocketState::ALIVE
            };
        } else {
            let mut temp = Rocket {
                pos: rocket_start_position,
                vel: Vec2 { x: 0.0, y: 0.0 },
                ptron: parent.ptron.clone(),
                state: RocketState::ALIVE
            };
            let mut temp_wts = temp.ptron.get_wts();
            for k in 0..temp_wts.len() {
                for i in 0..temp_wts[k].rows() {
                    for j in 0..temp_wts[k].cols() {
                        let cur = temp_wts[k].get(i, j);
                        temp_wts[k].set(i, j, cur + rng.gen::<f64>() * 10.0 - 5.0);
                    }
                }
            }
            temp.ptron.set_wts(temp_wts);
            return temp;
        }
    }
    pub fn get_state(&self) -> RocketState {
        return self.state;
    }
    pub fn update(&mut self, target: &Target) {
        let inp = Matrix::create_init(
            8,
            1,
            vec![
                self.pos.x, self.pos.y, self.vel.x, self.vel.y,
                target.get_pos().x,
                target.get_pos().y,
                0.0f64, //TODO OBSTACLE POSITION
                0.0f64,
            ],
        );
        let accel_matrix = self.ptron.calculate(&inp);
        let mut accel = Vector2 {
            x: accel_matrix.get(0, 0),
            y: accel_matrix.get(1, 0),
        };
        accel = accel * rocket_accel_multiplier * 0.0;
        self.vel.x += accel.x;
        self.vel.y += accel.y;
        self.vel = self.vel * rocket_velocity_multiplier * rocket_velocity_decay_rate;
        self.pos.x += self.vel.x;
        self.pos.y += self.vel.y;

        if self.pos.get_distance(&target.get_pos()) <= target.get_rad() {
            self.state = RocketState::DONE;
        }

        //TODO: Add check for distance from obstacles
    }
    pub fn draw(&self, mut window: RenderWindow) -> RenderWindow {
        let mut circle = CircleShape::new(rocket_radius, rocket_point_count);
        circle.set_position(Vector2::from((self.pos.x as f32, self.pos.y as f32)));
        //TODO: Change colour based on state
        window.draw(&circle);
        return window;
    }
}

fn main() {
    let mut window =
        RenderWindow::new((800, 600), "Hello World", Style::CLOSE, &Default::default());
    let mut rocket = Rocket::create();
    let mut targets= vec![Target::create()];
    while window.is_open() {
        while let Some(event) = window.poll_event() {
            if event == Event::Closed {
                window.close();
            }
        }
        window.clear(Color::GREEN);
        window = rocket.draw(window);
        rocket.update();
        window.display();
    }
}
