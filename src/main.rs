mod drivetrain;

use glam::Vec2;
use vexide::prelude::*;
use vexide::smart::motor::BrakeMode;

use crate::drivetrain::Drivetrain;

struct Arm {
    lift: [Motor; 2],
    claw: AdiDigitalOut,
}

struct Robot {
    controller: Controller,
    arm: Arm,
    drivetrain: Drivetrain,
}

impl Arm {}

impl Compete for Robot {
    async fn autonomous(&mut self) {
        println!("Autonomous!");
    }

    async fn driver(&mut self) {
        println!("Driver!");
        loop {
            let state = self.controller.state().unwrap_or_default();

            // Drivetrain movement
            let movement = Vec2 {
                // x and y is normalized Vec2 is not normalized
                x: state.left_stick.x() as f32,
                y: state.left_stick.y() as f32,
            };
            let speed = movement.length() * 12.0f32;
            let rotation = state.right_stick.x() as f32;

            self.drivetrain.trans(movement, speed, rotation).unwrap();

            // Arm movement
            for motor in self.arm.lift.iter_mut() {
                if state.button_up.is_now_pressed() {
                    motor.set_voltage(3.0).unwrap();
                } else if state.button_down.is_now_pressed() {
                    motor.set_voltage(-3.0).unwrap();
                } else {
                    motor.brake(BrakeMode::Brake).unwrap();
                }
            }
        }
    }
}

#[vexide::main]
async fn main(p: Peripherals) {
    let robot = Robot {
        controller: p.primary_controller,
        arm: Arm {
            lift: [
                Motor::new(p.port_1, Gearset::Red, Direction::Forward),
                Motor::new(p.port_2, Gearset::Red, Direction::Reverse),
            ],
            claw: AdiDigitalOut::new(p.adi_a),
        },
        drivetrain: Drivetrain {
            motors: [
                (
                    Motor::new(p.port_3, Gearset::Green, Direction::Forward),
                    0.0,
                ),
                (
                    Motor::new(p.port_4, Gearset::Green, Direction::Forward),
                    0.0,
                ),
                (
                    Motor::new(p.port_5, Gearset::Green, Direction::Forward),
                    0.0,
                ),
                (
                    Motor::new(p.port_6, Gearset::Green, Direction::Forward),
                    0.0,
                ),
            ],
        },
    };

    robot.compete().await;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_adds_two() {
        assert_eq!(2 + 2, 4);
    }
}
